use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: Option<String>,
    pub action: String,
    pub details: Option<serde_json::Value>,
    pub ip_address: String,
    pub created_at: chrono::NaiveDateTime,
}

impl AuditLog {
    pub async fn create(
        pool: &PgPool,
        id: &str,
        user_id: Option<&str>,
        action: &str,
        details: Option<serde_json::Value>,
        ip_address: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO audit_logs (id, user_id, action, details, ip_address) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(user_id)
        .bind(action)
        .bind(details)
        .bind(ip_address)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT * FROM audit_logs ORDER BY created_at DESC LIMIT 500",
        )
        .fetch_all(pool)
        .await
    }

    pub async fn list_by_user(pool: &PgPool, user_id: &str) -> Result<Vec<AuditLog>, sqlx::Error> {
        sqlx::query_as::<_, AuditLog>(
            "SELECT * FROM audit_logs WHERE user_id = $1 ORDER BY created_at DESC LIMIT 500",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }
}
