use std::collections::HashMap;

use chrono::{Datelike, Duration, NaiveDate, Utc};

use crate::{
    auth::extractor::CurrentUser,
    constants::roles,
    error::AppError,
    modules::checkins::{
        dto::{
            CheckinCalendarDayResponse, CheckinCalendarResponse, CheckinListQuery, CheckinResponse,
            CreateCheckinRequest,
        },
        repository,
    },
    state::AppState,
};

pub async fn list_checkins(
    state: &AppState,
    current_user: &CurrentUser,
    query: CheckinListQuery,
) -> Result<CheckinCalendarResponse, AppError> {
    require_user(current_user)?;

    let (month_start, next_month_start) = parse_month(&query.month)?;
    let pool = state.require_db()?;
    let rows = repository::list_by_month(pool, current_user.user_id, month_start, next_month_start)
        .await?;

    let by_date = rows
        .into_iter()
        .map(|row| (row.checkin_date, row))
        .collect::<HashMap<_, _>>();

    let mut days = Vec::new();
    let mut cursor = month_start;
    while cursor < next_month_start {
        if let Some(row) = by_date.get(&cursor) {
            days.push(CheckinCalendarDayResponse {
                date: cursor,
                checked_in: true,
                total_minutes: row.total_minutes,
                is_makeup: row.is_makeup != 0,
            });
        } else {
            days.push(CheckinCalendarDayResponse {
                date: cursor,
                checked_in: false,
                total_minutes: 0,
                is_makeup: false,
            });
        }

        cursor = cursor
            .succ_opt()
            .ok_or_else(|| AppError::Internal("date overflow".to_string()))?;
    }

    Ok(CheckinCalendarResponse {
        month: query.month,
        days,
    })
}

pub async fn get_checkin_detail(
    state: &AppState,
    current_user: &CurrentUser,
    date: String,
) -> Result<CheckinResponse, AppError> {
    require_user(current_user)?;

    let date = parse_date(&date)?;
    let pool = state.require_db()?;
    let row = repository::find_detail_by_date(pool, current_user.user_id, date)
        .await?
        .ok_or_else(|| AppError::NotFound("当日不存在打卡记录".to_string()))?;

    Ok(row.into())
}

pub async fn create_makeup_checkin(
    state: &AppState,
    current_user: &CurrentUser,
    payload: CreateCheckinRequest,
) -> Result<CheckinResponse, AppError> {
    require_user(current_user)?;
    validate_text(payload.makeup_reason.as_deref(), "补卡原因", 255)?;
    validate_text(payload.summary_note.as_deref(), "学习总结", 1000)?;

    let date = parse_date(&payload.date)?;
    let today = Utc::now().date_naive();
    if date > today {
        return Err(AppError::Unprocessable("不能补未来日期".to_string()));
    }

    if date < today - Duration::days(7) {
        return Err(AppError::Unprocessable("只能补7日内的打卡".to_string()));
    }

    if payload.total_minutes <= 0 {
        return Err(AppError::Validation("补卡学习时长必须大于0".to_string()));
    }

    let pool = state.require_db()?;
    if repository::find_by_date(pool, current_user.user_id, date)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("该日期已存在打卡记录".to_string()));
    }

    let row = repository::create_checkin_transaction(
        pool,
        repository::CreateCheckinRecord {
            user_id: current_user.user_id,
            checkin_date: date,
            total_minutes: payload.total_minutes,
            is_makeup: true,
            makeup_reason: payload
                .makeup_reason
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty()),
            summary_note: payload
                .summary_note
                .as_deref()
                .map(str::trim)
                .filter(|value| !value.is_empty()),
        },
        today,
    )
    .await
    .map_err(|error| match error {
        sqlx::Error::Database(db_error) if db_error.is_unique_violation() => {
            AppError::Conflict("该日期已存在打卡记录".to_string())
        }
        error => AppError::Database(error),
    })?;

    Ok(row.into())
}

fn require_user(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::USER {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn parse_month(value: &str) -> Result<(NaiveDate, NaiveDate), AppError> {
    let date = NaiveDate::parse_from_str(&format!("{value}-01"), "%Y-%m-%d")
        .map_err(|_| AppError::Validation("month 格式必须为 YYYY-MM".to_string()))?;

    let next_month = if date.month() == 12 {
        NaiveDate::from_ymd_opt(date.year() + 1, 1, 1)
    } else {
        NaiveDate::from_ymd_opt(date.year(), date.month() + 1, 1)
    }
    .ok_or_else(|| AppError::Validation("month 无效".to_string()))?;

    Ok((date, next_month))
}

fn parse_date(value: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("date 格式必须为 YYYY-MM-DD".to_string()))
}

fn validate_text(value: Option<&str>, field: &str, max_chars: usize) -> Result<(), AppError> {
    if let Some(value) = value
        && value.chars().count() > max_chars
    {
        return Err(AppError::Validation(format!(
            "{field}不能超过{max_chars}个字符"
        )));
    }

    Ok(())
}
