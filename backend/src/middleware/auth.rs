use actix_web::{dev::Payload, FromRequest, HttpRequest};
use futures::future::Ready;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use crate::errors::AppError;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,       // user ID
    pub username: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct AuthenticatedUser(pub Claims);

impl FromRequest for AuthenticatedUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "));

        let token = match auth_header {
            Some(t) => t,
            None => {
                return futures::future::ready(Err(AppError::Unauthorized(
                    "Missing authorization token".to_string(),
                )))
            }
        };

        let jwt_secret = req
            .app_data::<actix_web::web::Data<crate::AppState>>()
            .map(|state| state.config.jwt_secret.clone())
            .unwrap_or_default();

        match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &Validation::default(),
        ) {
            Ok(data) => futures::future::ready(Ok(AuthenticatedUser(data.claims))),
            Err(e) => futures::future::ready(Err(AppError::Unauthorized(format!(
                "Invalid token: {}",
                e
            )))),
        }
    }
}

pub struct AdminUser(pub Claims);

impl FromRequest for AdminUser {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, payload: &mut Payload) -> Self::Future {
        match AuthenticatedUser::from_request(req, payload).into_inner() {
            Ok(AuthenticatedUser(claims)) => {
                if claims.role == "admin" {
                    futures::future::ready(Ok(AdminUser(claims)))
                } else {
                    futures::future::ready(Err(AppError::Forbidden))
                }
            }
            Err(e) => futures::future::ready(Err(e)),
        }
    }
}
