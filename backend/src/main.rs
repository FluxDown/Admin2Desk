use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use sqlx::PgPool;

mod config;
mod db;
mod errors;
mod middleware;
mod models;
mod routes;
mod services;
mod ws;

use config::AppConfig;

pub struct AppState {
    pub db: PgPool,
    pub config: AppConfig,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let config = AppConfig::from_env();
    let listen_address = config.listen_address.clone();

    log::info!("Connecting to database...");
    let pool = db::create_pool(&config.database_url).await;

    log::info!("Running migrations...");
    db::run_migrations(&pool).await;

    // Ensure recordings directory exists
    std::fs::create_dir_all(&config.recordings_dir).ok();

    let state = web::Data::new(AppState {
        db: pool,
        config,
    });

    log::info!("Starting server on {}", listen_address);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials()
            .max_age(3600);

        App::new()
            .app_data(state.clone())
            .wrap(cors)
            .configure(routes::configure)
    })
    .bind(&listen_address)?
    .run()
    .await
}
