//! Axum request extractor for authenticated routes.

use axum::{async_trait, extract::FromRequestParts, http::{header::AUTHORIZATION, request::Parts}};
use uuid::Uuid;
use crate::{auth::jwt::validate_token, error::AppError, state::SharedState};

/// Extracts and validates the Bearer JWT from the Authorization header.
/// If invalid → handler never runs, 401 is returned automatically.
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: Uuid,
}

#[async_trait]
impl FromRequestParts<SharedState> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &SharedState) -> Result<Self, Self::Rejection> {
        let header = parts.headers.get(AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Missing Authorization header".into()))?;

        let token = header.strip_prefix("Bearer ")
            .ok_or_else(|| AppError::Unauthorized("Expected 'Bearer <token>'".into()))?.trim();

        let claims = validate_token(token, &state.config.jwt)?;
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|_| AppError::Unauthorized("Invalid user ID in token".into()))?;

        Ok(Self { user_id })
    }
}
