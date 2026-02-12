use actix_web::{web, HttpRequest, HttpResponse};
use uuid::Uuid;

use crate::errors::AppError;
use crate::models::audit_log::AuditLog;
use crate::models::user::{CreateUserRequest, LoginRequest, User};
use crate::services::auth_service;
use crate::AppState;

fn get_ip(req: &HttpRequest) -> String {
    req.connection_info()
        .realip_remote_addr()
        .unwrap_or("unknown")
        .to_string()
}

async fn register(
    state: web::Data<AppState>,
    body: web::Json<CreateUserRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    // Validate input
    if body.username.len() < 3 || body.username.len() > 64 {
        return Err(AppError::Validation(
            "Username must be between 3 and 64 characters".to_string(),
        ));
    }
    if !body.email.contains('@') {
        return Err(AppError::Validation("Invalid email address".to_string()));
    }
    if body.password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters".to_string(),
        ));
    }

    // Check if username or email already exists
    if User::find_by_username(&state.db, &body.username)
        .await?
        .is_some()
    {
        return Err(AppError::Validation("Username already taken".to_string()));
    }
    if User::find_by_email(&state.db, &body.email).await?.is_some() {
        return Err(AppError::Validation("Email already registered".to_string()));
    }

    let user_id = Uuid::new_v4().to_string();
    let password_hash = auth_service::hash_password(&body.password)?;

    User::create(&state.db, &user_id, &body.username, &body.email, &password_hash).await?;

    let user = User::find_by_id(&state.db, &user_id)
        .await?
        .ok_or_else(|| AppError::Internal("User creation failed".to_string()))?;

    // Audit log
    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&user_id),
        "user.register",
        Some(serde_json::json!({"username": &body.username})),
        &get_ip(&req),
    )
    .await?;

    let token = auth_service::create_jwt(
        &user.id,
        &user.username,
        &user.role,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    Ok(HttpResponse::Created().json(serde_json::json!({
        "token": token,
        "user": user.to_response()
    })))
}

async fn login(
    state: web::Data<AppState>,
    body: web::Json<LoginRequest>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let user = User::find_by_username(&state.db, &body.username)
        .await?
        .ok_or_else(|| AppError::Unauthorized("Invalid credentials".to_string()))?;

    let valid = auth_service::verify_password(&body.password, &user.password_hash)?;
    if !valid {
        // Audit failed login
        AuditLog::create(
            &state.db,
            &Uuid::new_v4().to_string(),
            Some(&user.id),
            "user.login_failed",
            None,
            &get_ip(&req),
        )
        .await?;

        return Err(AppError::Unauthorized("Invalid credentials".to_string()));
    }

    let token = auth_service::create_jwt(
        &user.id,
        &user.username,
        &user.role,
        &state.config.jwt_secret,
        state.config.jwt_expiration_hours,
    )?;

    // Audit successful login
    AuditLog::create(
        &state.db,
        &Uuid::new_v4().to_string(),
        Some(&user.id),
        "user.login",
        None,
        &get_ip(&req),
    )
    .await?;

    Ok(HttpResponse::Ok().json(serde_json::json!({
        "token": token,
        "user": user.to_response()
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register").route(web::post().to(register)))
        .service(web::resource("/login").route(web::post().to(login)));
}
