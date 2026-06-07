use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct StudySessionRow {
    pub id: i64,
    pub room_id: i64,
    pub user_id: i64,
    pub seat_id: i64,
    pub mode: String,
    pub study_content: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_minutes: i32,
    pub is_valid: i8,
    pub status: String,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct StudySessionDetailRow {
    pub id: i64,
    pub room_id: i64,
    pub room_name: String,
    pub user_id: i64,
    pub seat_id: i64,
    pub seat_code: String,
    pub mode: String,
    pub study_content: Option<String>,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration_minutes: i32,
    pub is_valid: i8,
    pub status: String,
    pub last_heartbeat_at: Option<DateTime<Utc>>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct RoomLockRow {
    pub id: i64,
    pub status: String,
    pub capacity: i32,
    pub close_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct SeatLockRow {
    pub id: i64,
    pub room_id: i64,
    pub status: String,
}
