use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub database_url: String,
    pub jwt_secret: String,
    pub jwt_expiration_hours: i64,
    pub encryption_key: Vec<u8>,
    pub listen_address: String,
    pub recordings_dir: String,
}

impl AppConfig {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
            jwt_expiration_hours: env::var("JWT_EXPIRATION_HOURS")
                .unwrap_or_else(|_| "24".to_string())
                .parse()
                .expect("JWT_EXPIRATION_HOURS must be a number"),
            encryption_key: {
                let key_b64 =
                    env::var("CREDENTIAL_ENCRYPTION_KEY").expect("CREDENTIAL_ENCRYPTION_KEY must be set");
                use base64::Engine;
                base64::engine::general_purpose::STANDARD
                    .decode(&key_b64)
                    .expect("CREDENTIAL_ENCRYPTION_KEY must be valid base64")
            },
            listen_address: env::var("LISTEN_ADDRESS").unwrap_or_else(|_| "0.0.0.0:8080".to_string()),
            recordings_dir: env::var("RECORDINGS_DIR").unwrap_or_else(|_| "./recordings".to_string()),
        }
    }
}
