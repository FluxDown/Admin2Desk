pub mod auth;
pub mod users;
pub mod targets;
pub mod sessions;
pub mod audit;

use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(web::scope("/auth").configure(auth::configure))
            .service(web::scope("/users").configure(users::configure))
            .service(web::scope("/targets").configure(targets::configure))
            .service(web::scope("/sessions").configure(sessions::configure))
            .service(web::scope("/audit-logs").configure(audit::configure)),
    );
}
