use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct StudyRoomRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub is_private: i8,
    pub password: Option<String>,
    pub status: String,
    pub creator_id: i64,
    pub created_at: DateTime<Utc>,
    pub open_at: DateTime<Utc>,
    pub close_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RoomSummaryRow {
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
pub struct RoomDetailRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    pub is_private: i8,
    pub status: String,
    pub creator_id: i64,
    pub creator_nickname: String,
    pub creator_avatar_url: Option<String>,
    pub created_at: DateTime<Utc>,
    pub open_at: DateTime<Utc>,
    pub close_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RoomSeatRow {
    pub id: i64,
    pub seat_code: String,
    pub status: String,
    pub occupied_user_id: Option<i64>,
    pub occupied_nickname: Option<String>,
    pub occupied_avatar_url: Option<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RoomMemberRow {
    pub id: i64,
    pub nickname: String,
    pub avatar_url: Option<String>,
}
