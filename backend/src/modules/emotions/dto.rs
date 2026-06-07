use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::emotions::model::EmotionRecordRow;

#[derive(Debug, Deserialize)]
pub struct EmotionTrendQuery {
    pub period: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateEmotionRecordRequest {
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    #[serde(rename = "emotionScore")]
    pub emotion_score: i32,
    #[serde(rename = "userNote")]
    pub user_note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct EmotionRecordResponse {
    #[serde(rename = "emotionRecordId")]
    pub emotion_record_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    #[serde(rename = "emotionScore")]
    pub emotion_score: i32,
    #[serde(rename = "userNote")]
    pub user_note: Option<String>,
    #[serde(rename = "aiFeedback")]
    pub ai_feedback: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CreateEmotionRecordResponse {
    #[serde(rename = "emotionRecord")]
    pub emotion_record: EmotionRecordResponse,
    #[serde(rename = "aiFeedback")]
    pub ai_feedback: String,
}

#[derive(Debug, Serialize)]
pub struct EmotionTrendPointResponse {
    pub date: NaiveDate,
    #[serde(rename = "averageScore")]
    pub average_score: f64,
    #[serde(rename = "dominantEmotion")]
    pub dominant_emotion: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct EmotionTagDistributionResponse {
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct EmotionTrendItemResponse {
    pub date: NaiveDate,
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    #[serde(rename = "emotionValue")]
    pub emotion_value: i32,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct EmotionTrendResponse {
    pub period: String,
    #[serde(rename = "emotionMap")]
    pub emotion_map: std::collections::BTreeMap<String, i32>,
    pub items: Vec<EmotionTrendItemResponse>,
    #[serde(rename = "mainEmotion")]
    pub main_emotion: Option<String>,
    pub summary: String,
    pub trends: Vec<EmotionTrendPointResponse>,
    #[serde(rename = "tagDistribution")]
    pub tag_distribution: Vec<EmotionTagDistributionResponse>,
}

impl From<EmotionRecordRow> for EmotionRecordResponse {
    fn from(row: EmotionRecordRow) -> Self {
        Self {
            emotion_record_id: row.id.to_string(),
            session_id: row.session_id.to_string(),
            emotion_tag: row.emotion_tag,
            emotion_score: row.emotion_score,
            user_note: row.user_note,
            ai_feedback: row.ai_feedback.unwrap_or_default(),
            created_at: row.created_at,
        }
    }
}
