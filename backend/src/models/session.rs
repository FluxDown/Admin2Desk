use serde::Serialize;
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct Session {
    pub id: String,
    pub user_id: String,
    pub target_id: String,
    pub credential_id: String,
    pub started_at: chrono::NaiveDateTime,
    pub ended_at: Option<chrono::NaiveDateTime>,
    pub status: String,
    pub recording_path: Option<String>,
}

impl Session {
    pub async fn create(
        pool: &PgPool,
        id: &str,
        user_id: &str,
        target_id: &str,
        credential_id: &str,
        recording_path: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO sessions (id, user_id, target_id, credential_id, recording_path) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(user_id)
        .bind(target_id)
        .bind(credential_id)
        .bind(recording_path)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn end_session(
        pool: &PgPool,
        id: &str,
        status: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE sessions SET ended_at = NOW(), status = $1 WHERE id = $2")
            .bind(status)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn list_by_user(pool: &PgPool, user_id: &str) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>(
            "SELECT * FROM sessions WHERE user_id = $1 ORDER BY started_at DESC",
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM sessions ORDER BY started_at DESC")
            .fetch_all(pool)
            .await
    }

    pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<Session>, sqlx::Error> {
        sqlx::query_as::<_, Session>("SELECT * FROM sessions WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }
}
