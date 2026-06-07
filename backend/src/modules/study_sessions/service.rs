use crate::{
    auth::extractor::CurrentUser,
    constants::{roles, statuses},
    error::AppError,
    modules::study_sessions::{
        dto::{
            StartStudySessionRequest, StudyHeartbeatRequest, StudySessionResponse,
            UpdateStudySessionRequest,
        },
        repository::{self, HeartbeatError, StartStudySessionError, UpdateStudySessionError},
    },
    state::AppState,
    time,
};

pub async fn start_session(
    state: &AppState,
    current_user: &CurrentUser,
    payload: StartStudySessionRequest,
) -> Result<StudySessionResponse, AppError> {
    require_user(current_user)?;
    validate_mode(&payload.mode)?;
    validate_study_content(payload.study_content.as_deref())?;

    let pool = state.require_db()?;
    repository::cleanup_inactive_sessions(pool).await?;
    let study_content = payload
        .study_content
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let row = repository::create_session_transaction(
        pool,
        repository::CreateStudySessionRecord {
            room_id: payload.room_id,
            seat_id: payload.seat_id,
            user_id: current_user.user_id,
            mode: payload.mode.as_str(),
            study_content,
        },
    )
    .await
    .map_err(map_start_error)?;

    Ok(row.into())
}

pub async fn get_active_session(
    state: &AppState,
    current_user: &CurrentUser,
) -> Result<Option<StudySessionResponse>, AppError> {
    require_user(current_user)?;

    let pool = state.require_db()?;
    repository::cleanup_inactive_sessions(pool).await?;
    let row = repository::find_active_by_user(pool, current_user.user_id).await?;

    Ok(row.map(StudySessionResponse::from))
}

pub async fn update_session(
    state: &AppState,
    current_user: &CurrentUser,
    session_id: i64,
    payload: UpdateStudySessionRequest,
) -> Result<StudySessionResponse, AppError> {
    require_user(current_user)?;
    validate_next_status(&payload.status)?;
    validate_study_content(payload.study_content.as_deref())?;

    let pool = state.require_db()?;
    repository::cleanup_inactive_sessions(pool).await?;
    let study_content = payload
        .study_content
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty());

    let row = repository::update_status_transaction(
        pool,
        session_id,
        current_user.user_id,
        payload.status.as_str(),
        study_content,
        payload.ended_at,
    )
    .await
    .map_err(map_update_error)?;

    Ok(row.into())
}

pub async fn heartbeat(
    state: &AppState,
    current_user: &CurrentUser,
    session_id: i64,
    payload: StudyHeartbeatRequest,
) -> Result<StudySessionResponse, AppError> {
    require_user(current_user)?;

    let pool = state.require_db()?;
    repository::cleanup_inactive_sessions(pool).await?;
    let at = payload.client_time.unwrap_or_else(time::now);
    let row = repository::touch_heartbeat(pool, session_id, current_user.user_id, at)
        .await
        .map_err(map_heartbeat_error)?;

    Ok(row.into())
}

fn require_user(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::USER {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn validate_mode(mode: &str) -> Result<(), AppError> {
    match mode {
        "normal" | "pomodoro" => Ok(()),
        _ => Err(AppError::Validation(
            "学习模式仅支持 normal 或 pomodoro".to_string(),
        )),
    }
}

fn validate_next_status(status: &str) -> Result<(), AppError> {
    match status {
        statuses::study_session::STUDYING
        | statuses::study_session::PAUSED
        | statuses::study_session::ENDED => Ok(()),
        statuses::study_session::RESTING => Err(AppError::Validation(
            "进入休息状态请调用创建休息接口".to_string(),
        )),
        _ => Err(AppError::Validation(
            "学习会话状态仅支持 studying、paused、ended".to_string(),
        )),
    }
}

pub fn validate_study_content(value: Option<&str>) -> Result<(), AppError> {
    if let Some(value) = value
        && value.chars().count() > 255
    {
        return Err(AppError::Validation(
            "学习内容不能超过255个字符".to_string(),
        ));
    }

    Ok(())
}

fn map_start_error(error: StartStudySessionError) -> AppError {
    match error {
        StartStudySessionError::ActiveSession => {
            AppError::Conflict("当前已有进行中的学习会话".to_string())
        }
        StartStudySessionError::RoomNotFound => AppError::NotFound("自习室不存在".to_string()),
        StartStudySessionError::RoomClosed => {
            AppError::Unprocessable("自习室已关闭，不能开始学习".to_string())
        }
        StartStudySessionError::RoomExpired => {
            AppError::Unprocessable("自习室已超过开放截止时间".to_string())
        }
        StartStudySessionError::RoomFull => AppError::Conflict("自习室人数已满".to_string()),
        StartStudySessionError::SeatNotFound => AppError::NotFound("座位不存在".to_string()),
        StartStudySessionError::SeatRoomMismatch => {
            AppError::Validation("座位不属于当前自习室".to_string())
        }
        StartStudySessionError::SeatOccupied => AppError::Conflict("座位已被占用".to_string()),
        StartStudySessionError::Database(error) => AppError::Database(error),
    }
}

fn map_update_error(error: UpdateStudySessionError) -> AppError {
    match error {
        UpdateStudySessionError::SessionNotFound => {
            AppError::NotFound("学习会话不存在".to_string())
        }
        UpdateStudySessionError::Forbidden => AppError::Forbidden,
        UpdateStudySessionError::AlreadyEnded => {
            AppError::Conflict("学习会话已结束，不能继续更新".to_string())
        }
        UpdateStudySessionError::InvalidTransition { from, to } => {
            AppError::Unprocessable(format!("不允许从 {from} 切换到 {to}"))
        }
        UpdateStudySessionError::Database(error) => AppError::Database(error),
    }
}

fn map_heartbeat_error(error: HeartbeatError) -> AppError {
    match error {
        HeartbeatError::SessionNotFound => AppError::NotFound("学习会话不存在".to_string()),
        HeartbeatError::Forbidden => AppError::Forbidden,
        HeartbeatError::AlreadyEnded => {
            AppError::Conflict("学习会话已结束，不能继续心跳".to_string())
        }
        HeartbeatError::Database(error) => AppError::Database(error),
    }
}
