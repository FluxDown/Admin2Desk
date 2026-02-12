use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

pub async fn create_pool(database_url: &str) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await
        .expect("Failed to connect to PostgreSQL")
}

pub async fn run_migrations(pool: &PgPool) {
    // Run migration files from ../database/migrations/
    let migrations = [
        include_str!("../../database/migrations/001_create_users.sql"),
        include_str!("../../database/migrations/002_create_ssh_targets.sql"),
        include_str!("../../database/migrations/003_create_ssh_credentials.sql"),
        include_str!("../../database/migrations/004_create_sessions.sql"),
        include_str!("../../database/migrations/005_create_audit_logs.sql"),
    ];

    for migration in &migrations {
        sqlx::query(migration)
            .execute(pool)
            .await
            .expect("Failed to run migration");
    }

    log::info!("Database migrations completed successfully");
}
