// Google OAuth2 Helpers

use serde::{Deserialize,Serialize};
use crate::{config::GoogleOAuthConfig , error::{AppError,AppResult}};

#[derive(Debug,Deserialize)]
pub struct GoogleTokenResponse{
    pub access_token:String,
}

#[derive(Debug,Deserialize,Serialize)]
pub struct GoogleUserInfo {
    pub sub :String,
    pub email:String,
    pub email_verified:bool,
    pub name:<Options<String>,

}


pub async fn exchange_code(code: &str, config: &GoogleOAuthConfig, http: &reqwest::Client) -> AppResult<GoogleTokenResponse> {
    let params = [
        ("code", code), ("client_id", &config.client_id),
        ("client_secret", config.client_secret.expose()),
        ("redirect_uri", &config.redirect_uri), ("grant_type", "authorization_code"),
    ];
    let resp = http.post("https://oauth2.googleapis.com/token").form(&params).send().await
        .map_err(|e| AppError::ExchangeError(e.to_string()))?;
    if !resp.status().is_success() {
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::ExchangeError(format!("Google token error: {body}")));
    }
    resp.json::<GoogleTokenResponse>().await.map_err(|e| AppError::ExchangeError(e.to_string()))
}

pub async fn fetch_user_info(access_token: &str, http: &reqwest::Client) -> AppResult<GoogleUserInfo> {
    http.get("https://www.googleapis.com/oauth2/v3/userinfo")
        .bearer_auth(access_token).send().await
        .map_err(|e| AppError::ExchangeError(e.to_string()))?
        .json::<GoogleUserInfo>().await.map_err(|e| AppError::ExchangeError(e.to_string()))
}

pub fn authorization_url(config: &GoogleOAuthConfig) -> String {
    format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile",
        config.client_id, config.redirect_uri
    )
}
