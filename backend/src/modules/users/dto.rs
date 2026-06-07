use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::users::model::UserRow;

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub phone: String,
    pub nickname: String,
    pub password: String,
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
    #[serde(rename = "agreeTerms")]
    pub agree_terms: bool,
    #[serde(rename = "agreePrivacy")]
    pub agree_privacy: bool,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub phone: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub phone: String,
    pub nickname: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub role: String,
    pub status: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub user: UserResponse,
    pub token: String,
}

impl UserResponse {
    pub fn from_row(row: UserRow) -> Self {
        Self {
            user_id: row.id.to_string(),
            phone: row.phone,
            nickname: row.nickname,
            avatar_url: row.avatar_url,
            role: "user".to_string(),
            status: row.status,
            created_at: row.created_at,
        }
    }
}
