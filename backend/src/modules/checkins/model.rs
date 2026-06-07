use chrono::{DateTime, NaiveDate, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct CheckinRecordRow {
    pub id: i64,
    pub user_id: i64,
    pub checkin_date: NaiveDate,
    pub emotion_record_id: Option<i64>,
    pub total_minutes: i32,
    pub is_makeup: i8,
    pub makeup_reason: Option<String>,
    pub summary_note: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct CheckinDetailRow {
    pub id: i64,
    pub user_id: i64,
    pub checkin_date: NaiveDate,
    pub emotion_record_id: Option<i64>,
    pub total_minutes: i32,
    pub is_makeup: i8,
    pub makeup_reason: Option<String>,
    pub summary_note: Option<String>,
    pub created_at: DateTime<Utc>,
    pub emotion_tag: Option<String>,
    pub emotion_score: Option<i32>,
    pub user_note: Option<String>,
    pub ai_feedback: Option<String>,
    pub emotion_created_at: Option<DateTime<Utc>>,
}
