use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct StudyBreakRow {
    pub id: i64,
    pub session_id: i64,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: i32,
    pub is_extended: i8,
}
