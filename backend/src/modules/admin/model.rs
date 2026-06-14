use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: i64,
    pub admin_name: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AdminUserListRow {
    pub id: i64,
    pub nickname: String,
    pub phone: String,
    pub avatar_url: Option<String>,
    pub profile: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub streak_days: i32,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AdminRoomListRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub is_private: i8,
    pub status: String,
    pub creator_id: i64,
    pub creator_nickname: String,
    pub creator_avatar_url: Option<String>,
    pub current_members: i64,
    pub created_at: DateTime<Utc>,
    pub open_at: DateTime<Utc>,
    pub close_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuditLogRow {
    pub id: i64,
    pub admin_id: i64,
    pub action: String,
    pub target_type: String,
    pub target_id: i64,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuditLogListRow {
    pub id: i64,
    pub admin_id: i64,
    pub admin_name: String,
    pub action: String,
    pub target_type: String,
    pub target_id: i64,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EmotionDistributionRow {
    pub emotion_tag: String,
    pub count: i64,
}
