use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow, Serialize)]
pub struct SshTarget {
    pub id: String,
    pub name: String,
    pub hostname: String,
    pub port: i16,
    pub description: Option<String>,
    pub created_by: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateTargetRequest {
    pub name: String,
    pub hostname: String,
    pub port: Option<i16>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTargetRequest {
    pub name: Option<String>,
    pub hostname: Option<String>,
    pub port: Option<i16>,
    pub description: Option<String>,
}

impl SshTarget {
    pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<SshTarget>, sqlx::Error> {
        sqlx::query_as::<_, SshTarget>("SELECT * FROM ssh_targets WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_all(pool: &PgPool) -> Result<Vec<SshTarget>, sqlx::Error> {
        sqlx::query_as::<_, SshTarget>("SELECT * FROM ssh_targets ORDER BY name ASC")
            .fetch_all(pool)
            .await
    }

    pub async fn create(
        pool: &PgPool,
        id: &str,
        name: &str,
        hostname: &str,
        port: i16,
        description: Option<&str>,
        created_by: &str,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO ssh_targets (id, name, hostname, port, description, created_by) VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(id)
        .bind(name)
        .bind(hostname)
        .bind(port)
        .bind(description)
        .bind(created_by)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn update(
        pool: &PgPool,
        id: &str,
        name: &str,
        hostname: &str,
        port: i16,
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE ssh_targets SET name = $1, hostname = $2, port = $3, description = $4, updated_at = NOW() WHERE id = $5",
        )
        .bind(name)
        .bind(hostname)
        .bind(port)
        .bind(description)
        .bind(id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM ssh_targets WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
