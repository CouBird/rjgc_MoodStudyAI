use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct PeriodQuery {
    pub period: Option<String>,
    pub date: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TodayStatsResponse {
    #[serde(rename = "todayMinutes")]
    pub today_minutes: i64,
    #[serde(rename = "todayHours")]
    pub today_hours: f64,
    #[serde(rename = "streakDays")]
    pub streak_days: i32,
    #[serde(rename = "todayCheckin")]
    pub today_checkin: bool,
    #[serde(rename = "validCheckin")]
    pub valid_checkin: bool,
    #[serde(rename = "latestEmotion")]
    pub latest_emotion: Option<String>,
    pub mood: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct StudyTrendPointResponse {
    pub date: chrono::NaiveDate,
    pub hours: f64,
    #[serde(rename = "totalMinutes")]
    pub total_minutes: i64,
}

#[derive(Clone, Debug, Serialize)]
pub struct EmotionTrendPointResponse {
    pub date: chrono::NaiveDate,
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    #[serde(rename = "emotionValue")]
    pub emotion_value: i32,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct UserStatsResponse {
    pub period: String,
    pub date: chrono::NaiveDate,
    #[serde(rename = "startDate")]
    pub start_date: chrono::NaiveDate,
    #[serde(rename = "endDate")]
    pub end_date: chrono::NaiveDate,
    #[serde(rename = "totalMinutes")]
    pub total_minutes: i64,
    #[serde(rename = "totalHours")]
    pub total_hours: f64,
    #[serde(rename = "previousTotalMinutes")]
    pub previous_total_minutes: i64,
    #[serde(rename = "previousTotalHours")]
    pub previous_total_hours: f64,
    #[serde(rename = "totalMinutesChange")]
    pub total_minutes_change: i64,
    #[serde(rename = "totalHoursGrowthPercent")]
    pub total_hours_growth_percent: f64,
    #[serde(rename = "validSessionCount")]
    pub valid_session_count: i64,
    #[serde(rename = "previousValidSessionCount")]
    pub previous_valid_session_count: i64,
    #[serde(rename = "validSessionCountChange")]
    pub valid_session_count_change: i64,
    #[serde(rename = "validSessionCountGrowthPercent")]
    pub valid_session_count_growth_percent: f64,
    #[serde(rename = "checkinCount")]
    pub checkin_count: i64,
    #[serde(rename = "previousCheckinCount")]
    pub previous_checkin_count: i64,
    #[serde(rename = "checkinCountChange")]
    pub checkin_count_change: i64,
    #[serde(rename = "checkinCountGrowthPercent")]
    pub checkin_count_growth_percent: f64,
    #[serde(rename = "studyDays")]
    pub study_days: i64,
    #[serde(rename = "previousStudyDays")]
    pub previous_study_days: i64,
    #[serde(rename = "studyDaysChange")]
    pub study_days_change: i64,
    #[serde(rename = "studyDaysGrowthPercent")]
    pub study_days_growth_percent: f64,
    #[serde(rename = "streakDays")]
    pub streak_days: i32,
    #[serde(rename = "averageMinutes")]
    pub average_minutes: i64,
    #[serde(rename = "averageDailyMinutes")]
    pub average_daily_minutes: i64,
    #[serde(rename = "previousAverageDailyMinutes")]
    pub previous_average_daily_minutes: i64,
    #[serde(rename = "averageDailyMinutesChange")]
    pub average_daily_minutes_change: i64,
    #[serde(rename = "averageDailyHours")]
    pub average_daily_hours: f64,
    #[serde(rename = "previousAverageDailyHours")]
    pub previous_average_daily_hours: f64,
    #[serde(rename = "averageDailyHoursGrowthPercent")]
    pub average_daily_hours_growth_percent: f64,
    pub trends: Vec<StudyTrendPointResponse>,
    #[serde(rename = "studyTrend")]
    pub study_trend: Vec<StudyTrendPointResponse>,
    #[serde(rename = "emotionMap")]
    pub emotion_map: std::collections::BTreeMap<String, i32>,
    #[serde(rename = "emotionTrend")]
    pub emotion_trend: Vec<EmotionTrendPointResponse>,
    #[serde(rename = "mainEmotion")]
    pub main_emotion: Option<String>,
    pub summary: String,
}
