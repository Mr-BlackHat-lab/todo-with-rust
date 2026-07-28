use crate::{error::AppError, state::AppState};
use axum::extract::State;
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};
use jsonwebtoken::{DecodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32,   // User ID
    pub exp: usize, // Expiration
}

pub struct AuthUser {
    pub user_id: i32,
}
#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Get Authorization header
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .ok_or(AppError::Unauthorized("Missing token".into()))?;

        // 2. Strip "Bearer " prefix
        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized("Invalid token format".into()))?;

        // 3. Decode and verify JWT
        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".into()))?;

        // 4. Return the extracted user data
        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}
