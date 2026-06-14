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
    pub profile: Option<String>,
    pub role: String,
    pub status: String,
    #[serde(rename = "streakDays")]
    pub streak_days: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateProfileRequest {
    pub nickname: Option<String>,
    pub profile: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ChangePasswordRequest {
    #[serde(rename = "currentPassword")]
    pub current_password: String,
    #[serde(rename = "newPassword")]
    pub new_password: String,
    #[serde(rename = "confirmPassword")]
    pub confirm_password: String,
}

#[derive(Debug, Serialize)]
pub struct AvatarResponse {
    #[serde(rename = "avatarUrl")]
    pub avatar_url: String,
    pub user: UserResponse,
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
            profile: row.profile,
            role: "user".to_string(),
            status: row.status,
            streak_days: row.streak_days,
            created_at: row.created_at,
        }
    }
}
