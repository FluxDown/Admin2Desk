use actix_web::{web, HttpRequest, HttpResponse};

use crate::errors::AppError;
use crate::middleware::auth::AuthenticatedUser;
use crate::models::session::Session;
use crate::AppState;

async fn list_sessions(
    state: web::Data<AppState>,
    user: AuthenticatedUser,
) -> Result<HttpResponse, AppError> {
    let sessions = if user.0.role == "admin" {
        Session::list_all(&state.db).await?
    } else {
        Session::list_by_user(&state.db, &user.0.sub).await?
    };
    Ok(HttpResponse::Ok().json(sessions))
}

async fn ws_connect(
    state: web::Data<AppState>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, AppError> {
    // WebSocket connection is handled in ws/ssh_ws.rs
    crate::ws::ssh_ws::handle_ws(state, req, stream).await
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(list_sessions)))
        .service(web::resource("/connect").route(web::get().to(ws_connect)));
}
