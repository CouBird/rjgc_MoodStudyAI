use crate::{
    auth::extractor::CurrentUser,
    constants::roles,
    error::AppError,
    modules::{
        study_breaks::{
            dto::{CreateBreakRequest, ExtendBreakRequest, StudyBreakResponse},
            repository::{self, CreateBreakError, ExtendBreakError},
        },
        study_sessions,
    },
    state::AppState,
};

pub async fn create_break(
    state: &AppState,
    current_user: &CurrentUser,
    session_id: i64,
    payload: CreateBreakRequest,
) -> Result<StudyBreakResponse, AppError> {
    require_user(current_user)?;
    validate_duration(payload.duration_minutes)?;

    let pool = state.require_db()?;
    study_sessions::repository::cleanup_inactive_sessions(pool).await?;
    let row = repository::create_break_transaction(
        pool,
        session_id,
        current_user.user_id,
        payload.duration_minutes,
    )
    .await
    .map_err(map_create_error)?;

    Ok(row.into())
}

pub async fn extend_break(
    state: &AppState,
    current_user: &CurrentUser,
    break_id: i64,
    payload: ExtendBreakRequest,
) -> Result<StudyBreakResponse, AppError> {
    require_user(current_user)?;
    validate_duration(payload.extend_minutes)?;

    let pool = state.require_db()?;
    study_sessions::repository::cleanup_inactive_sessions(pool).await?;
    let row = repository::extend_break_transaction(
        pool,
        break_id,
        current_user.user_id,
        payload.extend_minutes,
    )
    .await
    .map_err(map_extend_error)?;

    Ok(row.into())
}

fn require_user(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::USER {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn validate_duration(value: i32) -> Result<(), AppError> {
    if !(1..=180).contains(&value) {
        return Err(AppError::Validation(
            "休息时长必须在1-180分钟之间".to_string(),
        ));
    }

    Ok(())
}

fn map_create_error(error: CreateBreakError) -> AppError {
    match error {
        CreateBreakError::SessionNotFound => AppError::NotFound("学习会话不存在".to_string()),
        CreateBreakError::Forbidden => AppError::Forbidden,
        CreateBreakError::AlreadyEnded => {
            AppError::Conflict("学习会话已结束，不能休息".to_string())
        }
        CreateBreakError::NotStudying => {
            AppError::Unprocessable("只有学习中状态可以进入休息".to_string())
        }
        CreateBreakError::Database(error) => AppError::Database(error),
    }
}

fn map_extend_error(error: ExtendBreakError) -> AppError {
    match error {
        ExtendBreakError::BreakNotFound => AppError::NotFound("休息记录不存在".to_string()),
        ExtendBreakError::Forbidden => AppError::Forbidden,
        ExtendBreakError::AlreadyEnded => AppError::Conflict("休息已结束，不能延长".to_string()),
        ExtendBreakError::Database(error) => AppError::Database(error),
    }
}
