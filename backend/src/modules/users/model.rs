use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct UserRow {
    pub id: i64,
    pub nickname: String,
    pub password_hash: String,
    pub phone: String,
    pub avatar_url: Option<String>,
    pub profile: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub streak_days: i32,
}
