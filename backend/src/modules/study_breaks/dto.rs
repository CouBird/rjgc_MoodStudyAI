use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::study_breaks::model::StudyBreakRow;

#[derive(Debug, Deserialize)]
pub struct CreateBreakRequest {
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i32,
}

#[derive(Debug, Deserialize)]
pub struct ExtendBreakRequest {
    #[serde(rename = "extendMinutes")]
    pub extend_minutes: i32,
}

#[derive(Debug, Serialize)]
pub struct StudyBreakResponse {
    #[serde(rename = "breakId")]
    pub break_id: String,
    #[serde(rename = "sessionId")]
    pub session_id: String,
    #[serde(rename = "startTime")]
    pub start_time: DateTime<Utc>,
    #[serde(rename = "endTime")]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(rename = "durationMinutes")]
    pub duration_minutes: i32,
    #[serde(rename = "isExtended")]
    pub is_extended: bool,
}

impl From<StudyBreakRow> for StudyBreakResponse {
    fn from(row: StudyBreakRow) -> Self {
        Self {
            break_id: row.id.to_string(),
            session_id: row.session_id.to_string(),
            start_time: row.start_time,
            end_time: row.end_time,
            duration_minutes: row.duration,
            is_extended: row.is_extended != 0,
        }
    }
}
