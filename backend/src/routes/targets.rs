use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::middleware::auth::{AdminUser, AuthenticatedUser};
use crate::models::audit_log::AuditLog;
use crate::models::ssh_credential::{CreateCredentialRequest, SshCredential};
use crate::models::ssh_target::{CreateTargetRequest, SshTarget, UpdateTargetRequest};
use crate::services::auth_service;
use crate::AppState;

fn get_ip(req: &HttpRequest) -> String {
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

async fn list_targets(
    state: web::Data<AppState>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let targets = SshTarget::list_all(&state.db).await?;
    Ok(HttpResponse::Ok().json(targets))
}

async fn get_target(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();
    let target = SshTarget::find_by_id(&state.db, &target_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Target not found".to_string()))?;
    Ok(HttpResponse::Ok().json(target))
}

async fn create_target(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
    body: web::Json<CreateTargetRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    if body.name.is_empty() {
        return Err(AppError::Validation("Name is required".to_string()));
    }
    if body.hostname.is_empty() {
        return Err(AppError::Validation("Hostname is required".to_string()));
    }

    let target_id = Uuid::new_v4().to_string();
    let port = body.port.unwrap_or(22);

    SshTarget::create(
        &state.db,
        &target_id,
        &body.name,
        &body.hostname,
        port,
        body.description.as_deref(),
        &user.0.sub,
    )
    .await?;

    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&user.0.sub),
        "target.create",
        Some(serde_json::json!({"target_id": &target_id, "name": &body.name})),
        &get_ip(&req),
    )
    .await?;

    let target = SshTarget::find_by_id(&state.db, &target_id)
        .await?
        .ok_or_else(|| AppError::Internal("Target creation failed".to_string()))?;

    Ok(HttpResponse::Created().json(target))
}

async fn update_target(
    state: web::Data<AppState>,
    path: web::Path<String>,
    user: AuthenticatedUser,
    body: web::Json<UpdateTargetRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();
    let existing = SshTarget::find_by_id(&state.db, &target_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Target not found".to_string()))?;

    // Only creator or admin can update
    if existing.created_by != user.0.sub && user.0.role != "admin" {
        return Err(AppError::Forbidden);
    }

    let name = body.name.as_deref().unwrap_or(&existing.name);
    let hostname = body.hostname.as_deref().unwrap_or(&existing.hostname);
    let port = body.port.unwrap_or(existing.port);
    let description = body.description.as_deref().or(existing.description.as_deref());

    SshTarget::update(&state.db, &target_id, name, hostname, port, description).await?;

    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&user.0.sub),
        "target.update",
        Some(serde_json::json!({"target_id": &target_id})),
        &get_ip(&req),
    )
    .await?;

    let target = SshTarget::find_by_id(&state.db, &target_id)
        .await?
        .ok_or_else(|| AppError::Internal("Target not found after update".to_string()))?;

    Ok(HttpResponse::Ok().json(target))
}

async fn delete_target(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _admin: AdminUser,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();
    SshTarget::find_by_id(&state.db, &target_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Target not found".to_string()))?;

    SshTarget::delete(&state.db, &target_id).await?;

    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&_admin.0.sub),
        "target.delete",
        Some(serde_json::json!({"target_id": &target_id})),
        &get_ip(&req),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

// Credential endpoints for a target
async fn list_credentials(
    state: web::Data<AppState>,
    path: web::Path<String>,
    _user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();
    let creds = SshCredential::list_by_target(&state.db, &target_id).await?;
    let response: Vec<_> = creds.iter().map(|c| c.to_response()).collect();
    Ok(HttpResponse::Ok().json(response))
}

async fn create_credential(
    state: web::Data<AppState>,
    path: web::Path<String>,
    user: AuthenticatedUser,
    body: web::Json<CreateCredentialRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let target_id = path.into_inner();

    // Verify target exists
    SshTarget::find_by_id(&state.db, &target_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Target not found".to_string()))?;

    if body.auth_type != "password" && body.auth_type != "key" {
        return Err(AppError::Validation(
            "auth_type must be 'password' or 'key'".to_string(),
        ));
    }

    let cred_id = Uuid::new_v4().to_string();
    let encrypted = auth_service::encrypt_credential(
        body.credential.as_bytes(),
        &state.config.encryption_key,
    )?;

    SshCredential::create(
        &state.db,
        &cred_id,
        &target_id,
        &body.username,
        &body.auth_type,
        &encrypted,
    )
    .await?;

    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&user.0.sub),
        "credential.create",
        Some(serde_json::json!({"target_id": &target_id, "credential_id": &cred_id})),
        &get_ip(&req),
    )
    .await?;

    let cred = SshCredential::find_by_id(&state.db, &cred_id)
        .await?
        .ok_or_else(|| AppError::Internal("Credential creation failed".to_string()))?;

    Ok(HttpResponse::Created().json(cred.to_response()))
}

async fn delete_credential(
    state: web::Data<AppState>,
    path: web::Path<(String, String)>,
    _admin: AdminUser,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let (target_id, cred_id) = path.into_inner();

    SshCredential::find_by_id(&state.db, &cred_id)
        .await?
        .ok_or_else(|| AppError::NotFound("Credential not found".to_string()))?;

    SshCredential::delete(&state.db, &cred_id).await?;

    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&_admin.0.sub),
        "credential.delete",
        Some(serde_json::json!({"target_id": &target_id, "credential_id": &cred_id})),
        &get_ip(&req),
    )
    .await?;

    Ok(HttpResponse::NoContent().finish())
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::get().to(list_targets))
            .route(web::post().to(create_target)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(get_target))
            .route(web::put().to(update_target))
            .route(web::delete().to(delete_target)),
    )
    .service(
        web::resource("/{id}/credentials")
            .route(web::get().to(list_credentials))
            .route(web::post().to(create_credential)),
    )
    .service(
        web::resource("/{target_id}/credentials/{cred_id}")
            .route(web::delete().to(delete_credential)),
    );
}
