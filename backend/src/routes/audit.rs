use actix_web::{web, HttpResponse};

use crate::errors::AppError;
use crate::middleware::auth::AdminUser;
use crate::models::audit_log::AuditLog;
use crate::AppState;

async fn list_audit_logs(
    state: web::Data<AppState>,
    _admin: AdminUser,
) -> Result<HttpResponse, AppError> {
    let logs = AuditLog::list_all(&state.db).await?;
    Ok(HttpResponse::Ok().json(logs))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(list_audit_logs)));
}
