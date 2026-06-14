use chrono::{Duration, NaiveDate, Utc};

use crate::{
    auth::{extractor::CurrentUser, jwt::issue_token, password},
    constants::{roles, statuses},
    error::AppError,
    modules::admin::{
        audit,
        dto::{
            AdminAuditLogQuery, AdminAuthResponse, AdminDashboardResponse, AdminListQuery,
            AdminLoginRequest, AdminResponse, AdminRoomListResponse, AdminRoomResponse,
            AdminUserListResponse, AdminUserResponse, AuditLogListResponse,
            EmotionDistributionResponse, UpdateRoomStatusRequest, UpdateUserStatusRequest,
        },
        repository,
    },
    pagination::PageResult,
    state::AppState,
};

pub async fn login(
    state: &AppState,
    payload: AdminLoginRequest,
) -> Result<AdminAuthResponse, AppError> {
    if payload.account.trim().is_empty() || payload.password.is_empty() {
        return Err(AppError::Validation("账号和密码不能为空".to_string()));
    }

    let pool = state.require_db()?;
    let admin = repository::find_by_account(pool, &payload.account)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify_password(&payload.password, &admin.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    repository::insert_audit_log(
        pool,
        admin.id,
        audit::ACTION_ADMIN_LOGIN,
        "admin",
        admin.id,
        None,
    )
    .await?;

    let token = issue_token(
        admin.id,
        roles::ADMIN,
        &state.config.jwt.secret,
        state.config.jwt.admin_expire_hours,
    )?;

    Ok(AdminAuthResponse {
        admin: AdminResponse::from_row(admin),
        admin_token: token,
    })
}

pub async fn list_users(
    state: &AppState,
    current_user: &CurrentUser,
    query: AdminListQuery,
) -> Result<AdminUserListResponse, AppError> {
    require_admin(current_user)?;
    validate_optional_user_status(query.status.as_deref())?;

    let pool = state.require_db()?;
    let page = query.page();
    let page_size = query.page_size();
    let (rows, total) = repository::list_users(pool, &query).await?;

    Ok(PageResult {
        items: rows.into_iter().map(AdminUserResponse::from).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn update_user_status(
    state: &AppState,
    current_user: &CurrentUser,
    user_id: i64,
    payload: UpdateUserStatusRequest,
) -> Result<AdminUserResponse, AppError> {
    require_admin(current_user)?;
    validate_user_status(&payload.status)?;

    let pool = state.require_db()?;
    let status = payload.status.trim();
    let reason = clean_reason(payload.reason.as_deref());
    let row = repository::update_user_status(pool, user_id, status)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    let action = if status == statuses::user::DISABLED {
        audit::ACTION_USER_DISABLE
    } else {
        audit::ACTION_USER_ENABLE
    };
    repository::insert_audit_log(pool, current_user.user_id, action, "user", user_id, reason)
        .await?;

    Ok(AdminUserResponse::from(row))
}

pub async fn list_rooms(
    state: &AppState,
    current_user: &CurrentUser,
    query: AdminListQuery,
) -> Result<AdminRoomListResponse, AppError> {
    require_admin(current_user)?;
    validate_optional_room_status(query.status.as_deref())?;

    let pool = state.require_db()?;
    let page = query.page();
    let page_size = query.page_size();
    let (rows, total) = repository::list_rooms(pool, &query).await?;

    Ok(PageResult {
        items: rows.into_iter().map(AdminRoomResponse::from).collect(),
        total,
        page,
        page_size,
    })
}

pub async fn update_room_status(
    state: &AppState,
    current_user: &CurrentUser,
    room_id: i64,
    payload: UpdateRoomStatusRequest,
) -> Result<AdminRoomResponse, AppError> {
    require_admin(current_user)?;
    validate_room_status(&payload.status)?;

    let pool = state.require_db()?;
    let status = payload.status.trim();
    let reason = clean_reason(payload.reason.as_deref());
    let row = repository::update_room_status(pool, room_id, status)
        .await?
        .ok_or_else(|| AppError::NotFound("自习室不存在".to_string()))?;

    let action = if status == statuses::room::CLOSED {
        audit::ACTION_ROOM_CLOSE
    } else {
        audit::ACTION_ROOM_OPEN
    };
    repository::insert_audit_log(pool, current_user.user_id, action, "room", room_id, reason)
        .await?;

    Ok(AdminRoomResponse::from(row))
}

pub async fn dashboard(
    state: &AppState,
    current_user: &CurrentUser,
) -> Result<AdminDashboardResponse, AppError> {
    require_admin(current_user)?;

    let pool = state.require_db()?;
    let today = Utc::now().date_naive();
    let tomorrow = today + Duration::days(1);
    let start_at = day_start_utc(today)?;
    let end_at = day_start_utc(tomorrow)?;
    let today_study_minutes = repository::today_study_minutes(pool, today).await?;

    Ok(AdminDashboardResponse {
        total_users: repository::total_users(pool).await?,
        active_users: repository::users_by_status(pool, statuses::user::ACTIVE).await?,
        disabled_users: repository::users_by_status(pool, statuses::user::DISABLED).await?,
        total_rooms: repository::total_rooms(pool).await?,
        open_rooms: repository::rooms_by_status(pool, statuses::room::OPEN).await?,
        closed_rooms: repository::rooms_by_status(pool, statuses::room::CLOSED).await?,
        current_online_users: repository::current_online_users(pool).await?,
        today_study_minutes,
        today_study_hours: minutes_to_hours(today_study_minutes),
        today_checkins: repository::today_checkins(pool, today).await?,
        emotion_distribution: repository::today_emotion_distribution(pool, start_at, end_at)
            .await?
            .into_iter()
            .map(EmotionDistributionResponse::from)
            .collect(),
    })
}

pub async fn audit_logs(
    state: &AppState,
    current_user: &CurrentUser,
    query: AdminAuditLogQuery,
) -> Result<AuditLogListResponse, AppError> {
    require_admin(current_user)?;

    let action = normalize_audit_type(query.action_type.as_deref())?;
    let start_at = match query.start_date.as_deref().map(str::trim) {
        Some(value) if !value.is_empty() => Some(day_start_utc(parse_date(value)?)?),
        _ => None,
    };
    let end_at = match query.end_date.as_deref().map(str::trim) {
        Some(value) if !value.is_empty() => {
            Some(day_start_utc(parse_date(value)? + Duration::days(1))?)
        }
        _ => None,
    };

    let pool = state.require_db()?;
    let page = query.page();
    let page_size = query.page_size();
    let (rows, total) =
        repository::list_audit_logs(pool, &query, action.as_deref(), start_at, end_at).await?;

    Ok(PageResult {
        items: rows.into_iter().map(Into::into).collect(),
        total,
        page,
        page_size,
    })
}

fn require_admin(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::ADMIN {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn validate_optional_user_status(status: Option<&str>) -> Result<(), AppError> {
    match status.map(str::trim).filter(|value| !value.is_empty()) {
        None => Ok(()),
        Some(status) => validate_user_status(status),
    }
}

fn validate_user_status(status: &str) -> Result<(), AppError> {
    match status.trim() {
        statuses::user::ACTIVE | statuses::user::DISABLED => Ok(()),
        _ => Err(AppError::Validation(
            "用户状态仅支持 active 或 disabled".to_string(),
        )),
    }
}

fn validate_optional_room_status(status: Option<&str>) -> Result<(), AppError> {
    match status.map(str::trim).filter(|value| !value.is_empty()) {
        None => Ok(()),
        Some(status) => validate_room_status(status),
    }
}

fn validate_room_status(status: &str) -> Result<(), AppError> {
    match status.trim() {
        statuses::room::OPEN | statuses::room::CLOSED => Ok(()),
        _ => Err(AppError::Validation(
            "自习室状态仅支持 open 或 closed".to_string(),
        )),
    }
}

fn clean_reason(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn normalize_audit_type(value: Option<&str>) -> Result<Option<String>, AppError> {
    let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) else {
        return Ok(None);
    };

    let action = match value {
        "login" | audit::ACTION_ADMIN_LOGIN => audit::ACTION_ADMIN_LOGIN,
        "disable" | "user_disable" => audit::ACTION_USER_DISABLE,
        "enable" | "user_enable" => audit::ACTION_USER_ENABLE,
        audit::ACTION_ROOM_CLOSE => audit::ACTION_ROOM_CLOSE,
        audit::ACTION_ROOM_OPEN => audit::ACTION_ROOM_OPEN,
        _ => {
            return Err(AppError::Validation(
                "日志类型仅支持 login、disable、enable、room_close、room_open".to_string(),
            ));
        }
    };

    Ok(Some(action.to_string()))
}

fn parse_date(value: &str) -> Result<NaiveDate, AppError> {
    NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| AppError::Validation("日期格式必须为 YYYY-MM-DD".to_string()))
}

fn day_start_utc(date: NaiveDate) -> Result<chrono::DateTime<Utc>, AppError> {
    date.and_hms_opt(0, 0, 0)
        .map(|value| value.and_utc())
        .ok_or_else(|| AppError::Internal("date overflow".to_string()))
}

fn minutes_to_hours(minutes: i64) -> f64 {
    ((minutes as f64 / 60.0) * 10.0).round() / 10.0
}
