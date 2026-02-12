use crate::errors::AppError;
use async_trait::async_trait;
use russh::*;
use russh_keys::key::PublicKey;
use std::sync::Arc;
use std::time::Duration;

pub struct SshHandler;

#[async_trait]
impl client::Handler for SshHandler {
    type Error = russh::Error;

    async fn check_server_key(
        &mut self,
        _server_public_key: &PublicKey,
    ) -> Result<bool, Self::Error> {
        // TODO: In production, verify against known_hosts
        Ok(true)
    }
}

pub async fn connect_ssh(
    hostname: &str,
    port: u16,
    username: &str,
    auth_type: &str,
    decrypted_credential: &[u8],
) -> Result<(client::Handle<SshHandler>, Channel<client::Msg>), AppError> {
    let config = Arc::new(client::Config {
        inactivity_timeout: Some(Duration::from_secs(3600)),
        ..Default::default()
    });

    let handler = SshHandler;
    let mut handle = client::connect(config, (hostname, port), handler)
        .await
        .map_err(|e| AppError::Ssh(format!("Connection failed: {}", e)))?;

    match auth_type {
        "password" => {
            let password = std::str::from_utf8(decrypted_credential)
                .map_err(|_| AppError::Ssh("Invalid credential encoding".to_string()))?;
            let auth_result = handle
                .authenticate_password(username, password)
                .await
                .map_err(|e| AppError::Ssh(format!("Password auth failed: {}", e)))?;
            if !auth_result {
                return Err(AppError::Ssh("Password authentication rejected".to_string()));
            }
        }
        "key" => {
            let key_str = std::str::from_utf8(decrypted_credential)
                .map_err(|_| AppError::Ssh("Invalid key encoding".to_string()))?;
            let key_pair = russh_keys::decode_secret_key(key_str, None)
                .map_err(|e| AppError::Ssh(format!("Invalid private key: {}", e)))?;
            let auth_result = handle
                .authenticate_publickey(username, Arc::new(key_pair))
                .await
                .map_err(|e| AppError::Ssh(format!("Key auth failed: {}", e)))?;
            if !auth_result {
                return Err(AppError::Ssh("Key authentication rejected".to_string()));
            }
        }
        _ => return Err(AppError::Validation("Unknown auth type".to_string())),
    }

    let channel = handle
        .channel_open_session()
        .await
        .map_err(|e| AppError::Ssh(format!("Channel open failed: {}", e)))?;

    channel
        .request_pty(false, "xterm-256color", 80, 24, 0, 0, &[])
        .await
        .map_err(|e| AppError::Ssh(format!("PTY request failed: {}", e)))?;

    channel
        .request_shell(false)
        .await
        .map_err(|e| AppError::Ssh(format!("Shell request failed: {}", e)))?;

    Ok((handle, channel))
}
