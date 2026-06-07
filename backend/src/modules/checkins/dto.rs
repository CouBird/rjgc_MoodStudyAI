use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::checkins::model::{CheckinDetailRow, CheckinRecordRow};

#[derive(Debug, Deserialize)]
pub struct CheckinListQuery {
    pub month: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateCheckinRequest {
    pub date: String,
    #[serde(rename = "totalMinutes")]
    pub total_minutes: i32,
    #[serde(rename = "makeupReason")]
    pub makeup_reason: Option<String>,
    #[serde(rename = "summaryNote")]
    pub summary_note: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct CheckinResponse {
    #[serde(rename = "checkinId")]
    pub checkin_id: String,
    pub date: NaiveDate,
    #[serde(rename = "totalMinutes")]
    pub total_minutes: i32,
    #[serde(rename = "isMakeup")]
    pub is_makeup: bool,
    #[serde(rename = "makeupReason")]
    pub makeup_reason: Option<String>,
    #[serde(rename = "summaryNote")]
    pub summary_note: Option<String>,
    #[serde(rename = "emotionRecordId")]
    pub emotion_record_id: Option<String>,
    #[serde(rename = "emotionRecord")]
    pub emotion_record: Option<CheckinEmotionResponse>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct CheckinEmotionResponse {
    #[serde(rename = "emotionRecordId")]
    pub emotion_record_id: String,
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    #[serde(rename = "emotionScore")]
    pub emotion_score: i32,
    #[serde(rename = "userNote")]
    pub user_note: Option<String>,
    #[serde(rename = "aiFeedback")]
    pub ai_feedback: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CheckinCalendarDayResponse {
    pub date: NaiveDate,
    #[serde(rename = "checkedIn")]
    pub checked_in: bool,
    #[serde(rename = "totalMinutes")]
    pub total_minutes: i32,
    #[serde(rename = "isMakeup")]
    pub is_makeup: bool,
}

#[derive(Debug, Serialize)]
pub struct CheckinCalendarResponse {
    pub month: String,
    pub days: Vec<CheckinCalendarDayResponse>,
}

impl From<CheckinRecordRow> for CheckinResponse {
    fn from(row: CheckinRecordRow) -> Self {
        Self {
            checkin_id: row.id.to_string(),
            date: row.checkin_date,
            total_minutes: row.total_minutes,
            is_makeup: row.is_makeup != 0,
            makeup_reason: row.makeup_reason,
            summary_note: row.summary_note,
            emotion_record_id: row.emotion_record_id.map(|id| id.to_string()),
            emotion_record: None,
            created_at: row.created_at,
        }
    }
}

impl From<CheckinDetailRow> for CheckinResponse {
    fn from(row: CheckinDetailRow) -> Self {
        let emotion_record = match (row.emotion_record_id, row.emotion_tag, row.emotion_score) {
            (Some(id), Some(tag), Some(score)) => Some(CheckinEmotionResponse {
                emotion_record_id: id.to_string(),
                emotion_tag: tag,
                emotion_score: score,
                user_note: row.user_note,
                ai_feedback: row.ai_feedback,
                created_at: row.emotion_created_at,
            }),
            _ => None,
        };

        Self {
            checkin_id: row.id.to_string(),
            date: row.checkin_date,
            total_minutes: row.total_minutes,
            is_makeup: row.is_makeup != 0,
            makeup_reason: row.makeup_reason,
            summary_note: row.summary_note,
            emotion_record_id: row.emotion_record_id.map(|id| id.to_string()),
            emotion_record,
            created_at: row.created_at,
        }
    }
}
