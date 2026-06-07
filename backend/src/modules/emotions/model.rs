use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct EmotionRecordRow {
    pub id: i64,
    pub session_id: i64,
    pub emotion_tag: String,
    pub emotion_score: i32,
    pub user_note: Option<String>,
    pub ai_feedback: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EmotionTrendRow {
    pub date: chrono::NaiveDate,
    pub average_score: f64,
    pub dominant_emotion: String,
    pub count: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EmotionTagCountRow {
    pub emotion_tag: String,
    pub count: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EmotionTrendBucketRow {
    pub date: chrono::NaiveDate,
    pub emotion_tag: String,
    pub count: i64,
    pub last_created_at: DateTime<Utc>,
}
