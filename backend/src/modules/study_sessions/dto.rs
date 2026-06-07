use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, Serialize};

use crate::modules::study_sessions::model::StudySessionDetailRow;

#[derive(Debug, Deserialize)]
pub struct StartStudySessionRequest {
    #[serde(rename = "roomId")]
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub room_id: i64,
    #[serde(rename = "seatId")]
    #[serde(deserialize_with = "deserialize_i64_from_string_or_number")]
    pub seat_id: i64,
    pub mode: String,
    #[serde(rename = "studyContent")]
    pub study_content: Option<String>,
}

fn deserialize_i64_from_string_or_number<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::Number(number) => number
            .as_i64()
            .ok_or_else(|| serde::de::Error::custom("expected signed integer id")),
        serde_json::Value::String(value) => value
            .parse::<i64>()
            .map_err(|_| serde::de::Error::custom("expected numeric string id")),
        _ => Err(serde::de::Error::custom("expected string or number id")),
    }
}

#[derive(Debug, Deserialize)]
pub struct UpdateStudySessionRequest {
    pub status: String,
    #[serde(rename = "studyContent")]
    pub study_content: Option<String>,
    #[serde(rename = "endedAt")]
    pub ended_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct StudyHeartbeatRequest {
    #[serde(rename = "clientTime")]
    pub client_time: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct StudySessionResponse {
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "roomId")]
    pub room_id: String,
    #[serde(rename = "roomName")]
    pub room_name: String,
    #[serde(rename = "seatId")]
    pub seat_id: String,
    #[serde(rename = "seatCode")]
    pub seat_code: String,
    pub status: String,
    pub mode: String,
    #[serde(rename = "studyContent")]
    pub study_content: Option<String>,
    #[serde(rename = "startTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "endTime")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i32,
    #[serde(rename = "isValid")]
    pub is_valid: bool,
    #[serde(rename = "lastHeartbeatAt")]
    pub last_heartbeat_at: Option<DateTime<Utc>>,
}

impl From<StudySessionDetailRow> for StudySessionResponse {
    fn from(row: StudySessionDetailRow) -> Self {
        Self {
            session_id: row.id.to_string(),
            room_id: row.room_id.to_string(),
            room_name: row.room_name,
            seat_id: row.seat_id.to_string(),
            seat_code: row.seat_code,
            status: row.status,
            mode: row.mode,
            study_content: row.study_content,
            start_time: row.start_time,
            end_time: row.end_time,
            duration_minutes: row.duration_minutes,
            is_valid: row.is_valid != 0,
            last_heartbeat_at: row.last_heartbeat_at,
        }
    }
}
