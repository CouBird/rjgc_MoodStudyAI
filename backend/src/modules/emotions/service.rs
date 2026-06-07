use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

use crate::{
    ai::feedback,
    auth::extractor::CurrentUser,
    constants::roles,
    error::AppError,
    modules::{
        checkins,
        emotions::{
            dto::{
                CreateEmotionRecordRequest, CreateEmotionRecordResponse, EmotionRecordResponse,
                EmotionTagDistributionResponse, EmotionTrendItemResponse,
                EmotionTrendPointResponse, EmotionTrendQuery, EmotionTrendResponse,
            },
            model::EmotionTrendBucketRow,
            repository,
        },
    },
    state::AppState,
};

const EMOTION_VALUES: [(&str, i32); 8] = [
    ("平静", 0),
    ("自豪", 1),
    ("满足", 2),
    ("快乐", 3),
    ("焦虑", 4),
    ("疲惫", 5),
    ("难过", 6),
    ("自定义", 7),
];

const ALLOWED_EMOTIONS: [&str; 8] = [
    "平静",
    "自豪",
    "满足",
    "快乐",
    "疲惫",
    "焦虑",
    "难过",
    "自定义",
];

pub async fn create_emotion_record(
    state: &AppState,
    current_user: &CurrentUser,
    session_id: i64,
    payload: CreateEmotionRecordRequest,
) -> Result<CreateEmotionRecordResponse, AppError> {
    require_user(current_user)?;
    validate_payload(&payload)?;

    let pool = state.require_db()?;
    let session = repository::find_session(pool, session_id)
        .await?
        .ok_or_else(|| AppError::NotFound("学习会话不存在".to_string()))?;

    if session.user_id != current_user.user_id {
        return Err(AppError::Forbidden);
    }

    if repository::find_by_session(pool, session_id)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("该学习会话已提交情绪记录".to_string()));
    }

    let ai_feedback = feedback::generate_template_feedback(payload.emotion_tag.trim());
    let row = repository::create_record(
        pool,
        repository::CreateEmotionRecord {
            session_id,
            emotion_tag: payload.emotion_tag.trim(),
            emotion_score: payload.emotion_score,
            user_note: payload
                .user_note
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty()),
            ai_feedback: ai_feedback.as_str(),
        },
    )
    .await
    .map_err(|error| match error {
        sqlx::Error::Database(db_error) if db_error.is_unique_violation() => {
            AppError::Conflict("该学习会话已提交情绪记录".to_string())
        }
        error => AppError::Database(error),
    })?;

    let checkin_date = session.end_time.unwrap_or(row.created_at).date_naive();
    checkins::repository::attach_emotion_to_checkin(pool, session.user_id, checkin_date, row.id)
        .await?;

    let response = EmotionRecordResponse::from(row);
    Ok(CreateEmotionRecordResponse {
        emotion_record: response,
        ai_feedback,
    })
}

pub async fn emotion_trends(
    state: &AppState,
    current_user: &CurrentUser,
    query: EmotionTrendQuery,
) -> Result<EmotionTrendResponse, AppError> {
    require_user(current_user)?;

    let period = resolve_period(query)?;
    let pool = state.require_db()?;
    let trends =
        repository::list_trends(pool, current_user.user_id, period.start_at, period.end_at).await?;
    let distribution =
        repository::tag_distribution(pool, current_user.user_id, period.start_at, period.end_at)
            .await?;
    let buckets =
        repository::trend_buckets(pool, current_user.user_id, period.start_at, period.end_at)
            .await?;
    let (items, main_emotion, summary) = build_documented_trend(buckets);

    Ok(EmotionTrendResponse {
        period: period.name,
        emotion_map: emotion_map(),
        items,
        main_emotion,
        summary,
        trends: trends
            .into_iter()
            .map(|row| EmotionTrendPointResponse {
                date: row.date,
                average_score: (row.average_score * 10.0).round() / 10.0,
                dominant_emotion: row.dominant_emotion,
                count: row.count,
            })
            .collect(),
        tag_distribution: distribution
            .into_iter()
            .map(|row| EmotionTagDistributionResponse {
                emotion_tag: row.emotion_tag,
                count: row.count,
            })
            .collect(),
    })
}

fn require_user(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::USER {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn validate_payload(payload: &CreateEmotionRecordRequest) -> Result<(), AppError> {
    let tag = payload.emotion_tag.trim();
    if !ALLOWED_EMOTIONS.contains(&tag) {
        return Err(AppError::Validation(
            "情绪标签仅支持平静、自豪、满足、快乐、疲惫、焦虑、难过、自定义".to_string(),
        ));
    }

    if !(1..=10).contains(&payload.emotion_score) {
        return Err(AppError::Validation("情绪评分必须在1-10之间".to_string()));
    }

    if let Some(note) = &payload.user_note
        && note.chars().count() > 1000
    {
        return Err(AppError::Validation(
            "情绪备注不能超过1000个字符".to_string(),
        ));
    }

    Ok(())
}

struct TrendPeriod {
    name: String,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
}

fn resolve_period(query: EmotionTrendQuery) -> Result<TrendPeriod, AppError> {
    let name = query
        .period
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .unwrap_or("week")
        .to_string();
    let base_date = match query.date.as_deref().map(str::trim) {
        Some(value) if !value.is_empty() => parse_date(value)?,
        _ => Utc::now().date_naive(),
    };

    let (start_date, end_date_exclusive) = match name.as_str() {
        "week" => week_bounds(base_date),
        "month" => month_bounds(base_date)?,
        "year" => year_bounds(base_date)?,
        _ => {
            return Err(AppError::Validation(
                "period 仅支持 week、month、year".to_string(),
            ));
        }
    };

    Ok(TrendPeriod {
        name,
        start_at: day_start_utc(start_date)?,
        end_at: day_start_utc(end_date_exclusive)?,
    })
}

fn parse_date(value: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("date 格式必须为 YYYY-MM-DD".to_string()))
}

fn week_bounds(base_date: NaiveDate) -> (NaiveDate, NaiveDate) {
    let offset = i64::from(base_date.weekday().num_days_from_monday());
    let start = base_date - Duration::days(offset);
    (start, start + Duration::days(7))
}

fn month_bounds(base_date: NaiveDate) -> Result<(NaiveDate, NaiveDate), AppError> {
    let start = ymd(base_date.year(), base_date.month(), 1)?;
    let end = if base_date.month() == 12 {
        ymd(base_date.year() + 1, 1, 1)?
    } else {
        ymd(base_date.year(), base_date.month() + 1, 1)?
    };

    Ok((start, end))
}

fn year_bounds(base_date: NaiveDate) -> Result<(NaiveDate, NaiveDate), AppError> {
    Ok((
        ymd(base_date.year(), 1, 1)?,
        ymd(base_date.year() + 1, 1, 1)?,
    ))
}

fn ymd(year: i32, month: u32, day: u32) -> Result<NaiveDate, AppError> {
    NaiveDate::from_ymd_opt(year, month, day)
        .ok_or_else(|| AppError::Validation("date 无效".to_string()))
}

fn day_start_utc(date: NaiveDate) -> Result<DateTime<Utc>, AppError> {
    date.and_hms_opt(0, 0, 0)
        .map(|value| value.and_utc())
        .ok_or_else(|| AppError::Internal("date overflow".to_string()))
}

fn build_documented_trend(
    rows: Vec<EmotionTrendBucketRow>,
) -> (Vec<EmotionTrendItemResponse>, Option<String>, String) {
    let mut daily = BTreeMap::<NaiveDate, EmotionTrendItemResponse>::new();
    let mut distribution = HashMap::<String, i64>::new();

    for row in rows {
        *distribution.entry(row.emotion_tag.clone()).or_default() += row.count;
        daily
            .entry(row.date)
            .or_insert_with(|| EmotionTrendItemResponse {
                date: row.date,
                emotion_value: emotion_value(&row.emotion_tag),
                emotion_tag: row.emotion_tag,
                count: row.count,
            });
    }

    let main_emotion = dominant_emotion(distribution);
    let summary = match &main_emotion {
        Some(tag) => format!("这段时间内你常感到{tag}"),
        None => "这段时间还没有情绪记录".to_string(),
    };

    (daily.into_values().collect(), main_emotion, summary)
}

fn dominant_emotion(distribution: HashMap<String, i64>) -> Option<String> {
    let mut selected: Option<(String, i64)> = None;

    for (tag, count) in distribution {
        let should_replace = match &selected {
            None => true,
            Some((selected_tag, selected_count)) => {
                count > *selected_count
                    || (count == *selected_count
                        && emotion_value(&tag) < emotion_value(selected_tag))
            }
        };

        if should_replace {
            selected = Some((tag, count));
        }
    }

    selected.map(|(tag, _)| tag)
}

fn emotion_map() -> BTreeMap<String, i32> {
    EMOTION_VALUES
        .iter()
        .map(|(tag, value)| ((*tag).to_string(), *value))
        .collect()
}

fn emotion_value(tag: &str) -> i32 {
    EMOTION_VALUES
        .iter()
        .find_map(|(name, value)| (*name == tag).then_some(*value))
        .unwrap_or(7)
}
