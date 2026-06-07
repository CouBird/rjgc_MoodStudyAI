use std::collections::{BTreeMap, HashMap};

use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc};

use crate::{
    auth::extractor::CurrentUser,
    constants::roles,
    error::AppError,
    modules::stats::{
        dto::{
            EmotionTrendPointResponse, PeriodQuery, StudyTrendPointResponse, TodayStatsResponse,
            UserStatsResponse,
        },
        repository::{self, EmotionTrendBucketRow, StudyTrendRow},
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

pub async fn today_stats(
    state: &AppState,
    current_user: &CurrentUser,
) -> Result<TodayStatsResponse, AppError> {
    require_user(current_user)?;

    let pool = state.require_db()?;
    let today = Utc::now().date_naive();
    let today_minutes = repository::today_study_minutes(pool, current_user.user_id, today).await?;
    let streak_days = repository::streak_days_as_of(pool, current_user.user_id, today).await?;
    let today_checkin = repository::today_checkin_exists(pool, current_user.user_id, today).await?;
    let latest_emotion = repository::latest_emotion_tag(pool, current_user.user_id).await?;

    Ok(TodayStatsResponse {
        today_minutes,
        today_hours: minutes_to_hours(today_minutes),
        streak_days,
        today_checkin,
        valid_checkin: today_checkin,
        mood: latest_emotion.clone(),
        latest_emotion,
    })
}

pub async fn user_stats(
    state: &AppState,
    current_user: &CurrentUser,
    query: PeriodQuery,
) -> Result<UserStatsResponse, AppError> {
    require_user(current_user)?;

    let period = resolve_period(query)?;
    let pool = state.require_db()?;
    let trend_rows = repository::study_trends(
        pool,
        current_user.user_id,
        period.start_date,
        period.end_date_exclusive,
    )
    .await?;
    let total_minutes: i64 = trend_rows.iter().map(|row| row.total_minutes).sum();
    let checkin_count = trend_rows.len() as i64;
    let study_days = trend_rows
        .iter()
        .filter(|row| row.total_minutes > 0)
        .count() as i64;
    let valid_session_count =
        repository::valid_session_count(pool, current_user.user_id, period.start_at, period.end_at)
            .await?;
    let streak_days =
        repository::streak_days_as_of(pool, current_user.user_id, period.base_date).await?;
    let study_trend = build_study_trend(period.start_date, period.end_date_exclusive, trend_rows)?;
    let emotion_rows = repository::emotion_trend_buckets(
        pool,
        current_user.user_id,
        period.start_at,
        period.end_at,
    )
    .await?;
    let (emotion_trend, main_emotion, summary) = build_emotion_trend(emotion_rows);
    let average_daily_minutes = if period.day_count > 0 {
        total_minutes / period.day_count
    } else {
        0
    };
    let previous_period = previous_period(&period)?;
    let previous_trend_rows = repository::study_trends(
        pool,
        current_user.user_id,
        previous_period.start_date,
        previous_period.end_date_exclusive,
    )
    .await?;
    let previous_total_minutes: i64 = previous_trend_rows
        .iter()
        .map(|row| row.total_minutes)
        .sum();
    let previous_checkin_count = previous_trend_rows.len() as i64;
    let previous_study_days = previous_trend_rows
        .iter()
        .filter(|row| row.total_minutes > 0)
        .count() as i64;
    let previous_valid_session_count = repository::valid_session_count(
        pool,
        current_user.user_id,
        previous_period.start_at,
        previous_period.end_at,
    )
    .await?;
    let previous_average_daily_minutes = if previous_period.day_count > 0 {
        previous_total_minutes / previous_period.day_count
    } else {
        0
    };
    let trends = study_trend.clone();

    Ok(UserStatsResponse {
        period: period.name,
        date: period.base_date,
        start_date: period.start_date,
        end_date: period.end_date,
        total_minutes,
        total_hours: minutes_to_hours(total_minutes),
        previous_total_minutes,
        previous_total_hours: minutes_to_hours(previous_total_minutes),
        total_minutes_change: total_minutes - previous_total_minutes,
        total_hours_growth_percent: growth_percent(total_minutes, previous_total_minutes),
        valid_session_count,
        previous_valid_session_count,
        valid_session_count_change: valid_session_count - previous_valid_session_count,
        valid_session_count_growth_percent: growth_percent(
            valid_session_count,
            previous_valid_session_count,
        ),
        checkin_count,
        previous_checkin_count,
        checkin_count_change: checkin_count - previous_checkin_count,
        checkin_count_growth_percent: growth_percent(checkin_count, previous_checkin_count),
        study_days,
        previous_study_days,
        study_days_change: study_days - previous_study_days,
        study_days_growth_percent: growth_percent(study_days, previous_study_days),
        streak_days,
        average_minutes: average_daily_minutes,
        average_daily_minutes,
        previous_average_daily_minutes,
        average_daily_minutes_change: average_daily_minutes - previous_average_daily_minutes,
        average_daily_hours: minutes_to_hours(average_daily_minutes),
        previous_average_daily_hours: minutes_to_hours(previous_average_daily_minutes),
        average_daily_hours_growth_percent: growth_percent(
            average_daily_minutes,
            previous_average_daily_minutes,
        ),
        trends,
        study_trend,
        emotion_map: emotion_map(),
        emotion_trend,
        main_emotion,
        summary,
    })
}

fn require_user(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::USER {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

struct StatsPeriod {
    name: String,
    base_date: NaiveDate,
    start_date: NaiveDate,
    end_date: NaiveDate,
    end_date_exclusive: NaiveDate,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    day_count: i64,
}

struct ComparisonPeriod {
    start_date: NaiveDate,
    end_date_exclusive: NaiveDate,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
    day_count: i64,
}

fn resolve_period(query: PeriodQuery) -> Result<StatsPeriod, AppError> {
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

    let end_date = end_date_exclusive
        .pred_opt()
        .ok_or_else(|| AppError::Internal("date overflow".to_string()))?;
    let start_at = day_start_utc(start_date)?;
    let end_at = day_start_utc(end_date_exclusive)?;
    let day_count = end_date_exclusive
        .signed_duration_since(start_date)
        .num_days();

    Ok(StatsPeriod {
        name,
        base_date,
        start_date,
        end_date,
        end_date_exclusive,
        start_at,
        end_at,
        day_count,
    })
}

fn previous_period(period: &StatsPeriod) -> Result<ComparisonPeriod, AppError> {
    let (start_date, end_date_exclusive) = match period.name.as_str() {
        "week" => (period.start_date - Duration::days(7), period.start_date),
        "month" => {
            let previous_month_date = period
                .start_date
                .pred_opt()
                .ok_or_else(|| AppError::Internal("date overflow".to_string()))?;
            month_bounds(previous_month_date)?
        }
        "year" => (
            ymd(period.start_date.year() - 1, 1, 1)?,
            ymd(period.start_date.year(), 1, 1)?,
        ),
        _ => {
            return Err(AppError::Validation(
                "period 仅支持 week、month、year".to_string(),
            ));
        }
    };
    let day_count = end_date_exclusive
        .signed_duration_since(start_date)
        .num_days();

    Ok(ComparisonPeriod {
        start_date,
        end_date_exclusive,
        start_at: day_start_utc(start_date)?,
        end_at: day_start_utc(end_date_exclusive)?,
        day_count,
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

fn build_study_trend(
    start_date: NaiveDate,
    end_date_exclusive: NaiveDate,
    rows: Vec<StudyTrendRow>,
) -> Result<Vec<StudyTrendPointResponse>, AppError> {
    let totals = rows
        .into_iter()
        .map(|row| (row.date, row.total_minutes))
        .collect::<HashMap<_, _>>();
    let mut points = Vec::new();
    let mut cursor = start_date;

    while cursor < end_date_exclusive {
        let total_minutes = totals.get(&cursor).copied().unwrap_or_default();
        points.push(StudyTrendPointResponse {
            date: cursor,
            hours: minutes_to_hours(total_minutes),
            total_minutes,
        });
        cursor = cursor
            .succ_opt()
            .ok_or_else(|| AppError::Internal("date overflow".to_string()))?;
    }

    Ok(points)
}

fn build_emotion_trend(
    rows: Vec<EmotionTrendBucketRow>,
) -> (Vec<EmotionTrendPointResponse>, Option<String>, String) {
    let mut daily = BTreeMap::<NaiveDate, EmotionTrendPointResponse>::new();
    let mut distribution = HashMap::<String, i64>::new();

    for row in rows {
        *distribution.entry(row.emotion_tag.clone()).or_default() += row.count;
        daily
            .entry(row.date)
            .or_insert_with(|| EmotionTrendPointResponse {
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

fn minutes_to_hours(minutes: i64) -> f64 {
    ((minutes as f64 / 60.0) * 10.0).round() / 10.0
}

fn growth_percent(current: i64, previous: i64) -> f64 {
    if previous == 0 {
        return if current == 0 { 0.0 } else { 100.0 };
    }

    ((((current - previous) as f64 / previous as f64) * 100.0) * 10.0).round() / 10.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn week_period_uses_monday_to_sunday() {
        let period = resolve_period(PeriodQuery {
            period: Some("week".to_string()),
            date: Some("2026-06-07".to_string()),
        })
        .unwrap();

        assert_eq!(period.start_date, ymd(2026, 6, 1).unwrap());
        assert_eq!(period.end_date, ymd(2026, 6, 7).unwrap());
        assert_eq!(period.day_count, 7);
    }
}
