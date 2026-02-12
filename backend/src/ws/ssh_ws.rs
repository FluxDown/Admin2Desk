use actix_web::{web, HttpRequest, HttpResponse};
use actix_ws::Message;
use futures::StreamExt;
use jsonwebtoken::{decode, DecodingKey, Validation};
use russh::ChannelMsg;
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::Claims;
use crate::models::audit_log::AuditLog;
use crate::models::session::Session;
use crate::models::ssh_credential::SshCredential;
use crate::models::ssh_target::SshTarget;
use crate::services::{auth_service, recording_service, ssh_service};
use crate::AppState;

#[derive(serde::Deserialize)]
pub struct WsQuery {
    pub token: String,
    pub target_id: String,
    pub credential_id: String,
}

fn get_ip(req: &HttpRequest) -> String {
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

pub async fn handle_ws(
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, AppError> {
    let query = web::Query::<WsQuery>::from_query(req.query_string())
        .map_err(|_| AppError::Validation("Missing query parameters: token, target_id, credential_id".to_string()))?;

    // Validate JWT from query param
    let claims = decode::<Claims>(
        &query.token,
        &DecodingKey::from_secret(state.config.jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))?
    .claims;

    // Load target and credential
    let target = SshTarget::find_by_id(&state.db, &query.target_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Target not found".to_string()))?;

    let credential = SshCredential::find_by_id(&state.db, &query.credential_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    // Decrypt credential
    let decrypted = auth_service::decrypt_credential(
        &credential.encrypted_credential,
        &state.config.encryption_key,
    )?;

    // Upgrade to WebSocket
    let (response, mut ws_session, mut msg_stream) = actix_ws::handle(&req, stream)
        .map_err(|e| AppError::Internal(format!("WebSocket upgrade failed: {}", e)))?;

    let db = state.db.clone();
    let recordings_dir = state.config.recordings_dir.clone();
    let ip = get_ip(&req);
    let user_id = claims.sub.clone();

    // Spawn the SSH relay task
    actix_rt::spawn(async move {
        let session_id = Uuid::new_v4().to_string();

        // Create session record
        let recording_path = recording_service::recording_path(&recordings_dir, &session_id);
        if let Err(e) = Session::create(
            &db,
            &session_id,
            &user_id,
            &target.id,
            &credential.id,
            Some(&recording_path),
        )
        .await
        {
            log::error!("Failed to create session record: {}", e);
            let _ = ws_session.text("Error: Failed to create session").await;
            let _ = ws_session.close(None).await;
            return;
        }

        // Audit log
        let _ = AuditLog::create(
            &db,
            &Uuid::new_v4().to_string(),
            Some(&user_id),
            "session.start",
            Some(serde_json::json!({"session_id": &session_id, "target": &target.name})),
            &ip,
        )
        .await;

        // Connect to SSH target
        let (_handle, mut channel) = match ssh_service::connect_ssh(
            &target.hostname,
            target.port as u16,
            &credential.username,
            &credential.auth_type,
            &decrypted,
        )
        .await
        {
            Ok(result) => result,
            Err(e) => {
                log::error!("SSH connection failed: {}", e);
                let _ = ws_session
                    .text(format!("\r\n\x1b[31mSSH connection failed: {}\x1b[0m\r\n", e))
                    .await;
                let _ = ws_session.close(None).await;
                let _ = Session::end_session(&db, &session_id, "failed").await;
                return;
            }
        };

        // Start recording
        let mut recorder = match recording_service::RecordingWriter::new(&recordings_dir, &session_id, 80, 24)
        {
            Ok(r) => Some(r),
            Err(e) => {
                log::warn!("Failed to start recording: {}", e);
                None
            }
        };

        // Relay loop
        let final_status = "completed";

        loop {
            tokio::select! {
                // Message from WebSocket (browser -> SSH)
                ws_msg = msg_stream.next() => {
                    match ws_msg {
                        Some(Ok(Message::Binary(data))) => {
                            if data.is_empty() {
                                continue;
                            }
                            // Check for control message (0x02 prefix = resize)
                            if data[0] == 0x02 {
                                if let Ok(ctrl) = serde_json::from_slice::<serde_json::Value>(&data[1..]) {
                                    if ctrl.get("type").and_then(|t| t.as_str()) == Some("resize") {
                                        let cols = ctrl.get("cols").and_then(|c| c.as_u64()).unwrap_or(80) as u32;
                                        let rows = ctrl.get("rows").and_then(|r| r.as_u64()).unwrap_or(24) as u32;
                                        let _ = channel.window_change(cols, rows, 0, 0).await;
                                    }
                                }
                            } else {
                                // Regular data - forward to SSH as &[u8]
                                let bytes: &[u8] = &data;
                                let _ = channel.data(bytes).await;
                                if let Some(ref mut rec) = recorder {
                                    let _ = rec.write_input(&data);
                                }
                            }
                        }
                        Some(Ok(Message::Text(text))) => {
                            let bytes: &[u8] = text.as_bytes();
                            let _ = channel.data(bytes).await;
                            if let Some(ref mut rec) = recorder {
                                let _ = rec.write_input(text.as_bytes());
                            }
                        }
                        Some(Ok(Message::Close(_))) | None => {
                            log::info!("WebSocket closed for session {}", session_id);
                            break;
                        }
                        Some(Ok(Message::Ping(data))) => {
                            let _ = ws_session.pong(&data).await;
                        }
                        _ => {}
                    }
                }
                // Message from SSH (target -> browser)
                ssh_msg = channel.wait() => {
                    match ssh_msg {
                        Some(ChannelMsg::Data { data }) => {
                            let bytes = data.to_vec();
                            if let Some(ref mut rec) = recorder {
                                let _ = rec.write_output(&bytes);
                            }
                            if ws_session.binary(bytes).await.is_err() {
                                break;
                            }
                        }
                        Some(ChannelMsg::ExtendedData { data, ext: _ }) => {
                            let bytes = data.to_vec();
                            if let Some(ref mut rec) = recorder {
                                let _ = rec.write_output(&bytes);
                            }
                            if ws_session.binary(bytes).await.is_err() {
                                break;
                            }
                        }
                        Some(ChannelMsg::Eof) | Some(ChannelMsg::Close) | None => {
                            log::info!("SSH channel closed for session {}", session_id);
                            let _ = ws_session
                                .text("\r\n\x1b[33m--- Session ended ---\x1b[0m\r\n")
                                .await;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }

        // Cleanup
        if let Some(ref mut rec) = recorder {
            let _ = rec.finish();
        }
        let _ = ws_session.close(None).await;
        let _ = Session::end_session(&db, &session_id, final_status).await;

        let _ = AuditLog::create(
            &db,
            &Uuid::new_v4().to_string(),
            Some(&user_id),
            "session.end",
            Some(serde_json::json!({"session_id": &session_id, "status": final_status})),
            &ip,
        )
        .await;
    });

    Ok(response)
}
