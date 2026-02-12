use actix_web::{web, HttpResponse};

use crate::errors::AppError;
use crate::middleware::auth::AdminUser;
use crate::models::user::User;
use crate::AppState;

async fn list_users(
    state: web::Data<AppState>,
    _admin: AdminUser,
) -> Result<HttpResponse, AppError> {
    let users = User::list_all(&state.db).await?;
    let response: Vec<_> = users.iter().map(|u| u.to_response()).collect();
    Ok(HttpResponse::Ok().json(response))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(list_users)));
}
