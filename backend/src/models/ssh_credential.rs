use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Debug, sqlx::FromRow)]
pub struct SshCredential {
    pub id: String,
    pub target_id: String,
    pub username: String,
    pub auth_type: String,
    pub encrypted_credential: Vec<u8>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Serialize)]
pub struct SshCredentialResponse {
    pub id: String,
    pub target_id: String,
    pub username: String,
    pub auth_type: String,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Deserialize)]
pub struct CreateCredentialRequest {
    pub username: String,
    pub auth_type: String,
    pub credential: String, // plaintext password or private key - will be encrypted
}

impl SshCredential {
    pub async fn find_by_id(pool: &PgPool, id: &str) -> Result<Option<SshCredential>, sqlx::Error> {
        sqlx::query_as::<_, SshCredential>("SELECT * FROM ssh_credentials WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await
    }

    pub async fn list_by_target(pool: &PgPool, target_id: &str) -> Result<Vec<SshCredential>, sqlx::Error> {
        sqlx::query_as::<_, SshCredential>(
            "SELECT * FROM ssh_credentials WHERE target_id = $1 ORDER BY created_at DESC",
        )
        .bind(target_id)
        .fetch_all(pool)
        .await
    }

    pub async fn create(
        pool: &PgPool,
        id: &str,
        target_id: &str,
        username: &str,
        auth_type: &str,
        encrypted_credential: &[u8],
    ) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO ssh_credentials (id, target_id, username, auth_type, encrypted_credential) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(target_id)
        .bind(username)
        .bind(auth_type)
        .bind(encrypted_credential)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &PgPool, id: &str) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM ssh_credentials WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub fn to_response(&self) -> SshCredentialResponse {
        SshCredentialResponse {
            id: self.id.clone(),
            target_id: self.target_id.clone(),
            username: self.username.clone(),
            auth_type: self.auth_type.clone(),
            created_at: self.created_at,
        }
    }
}
