//! JWT token issuance and validation.

use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{config::JwtConfig, error::{AppError, AppResult}};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub token_type: String,
}

#[derive(Debug, Serialize)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u64,
}

impl TokenPair {
    pub fn new(user_id: Uuid, config: &JwtConfig) -> AppResult<Self> {
        Ok(Self {
            access_token: issue_access_token(user_id, config)?,
            refresh_token: issue_refresh_token(user_id, config)?,
            expires_in: config.access_token_lifetime_secs,
        })
    }
}

pub fn issue_access_token(user_id: Uuid, config: &JwtConfig) -> AppResult<String> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims { sub: user_id.to_string(), exp: now + config.access_token_lifetime_secs as usize, iat: now, token_type: "access".into() };
    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(config.access_secret.expose().as_bytes()))?)
}

pub fn issue_refresh_token(user_id: Uuid, config: &JwtConfig) -> AppResult<String> {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims { sub: user_id.to_string(), exp: now + config.refresh_token_lifetime_secs as usize, iat: now, token_type: "refresh".into() };
    Ok(encode(&Header::default(), &claims, &EncodingKey::from_secret(config.refresh_secret.expose().as_bytes()))?)
}

pub fn validate_token(token: &str, config: &JwtConfig) -> AppResult<Claims> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(config.access_secret.expose().as_bytes()), &Validation::default())
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {e}")))?;
    if data.claims.token_type != "access" { return Err(AppError::Unauthorized("Wrong token type".into())); }
    Ok(data.claims)
}

pub fn validate_refresh_token(token: &str, config: &JwtConfig) -> AppResult<Claims> {
    let data = decode::<Claims>(token, &DecodingKey::from_secret(config.refresh_secret.expose().as_bytes()), &Validation::default())
        .map_err(|e| AppError::Unauthorized(format!("Invalid refresh token: {e}")))?;
    if data.claims.token_type != "refresh" { return Err(AppError::Unauthorized("Wrong token type".into())); }
    Ok(data.claims)
}
