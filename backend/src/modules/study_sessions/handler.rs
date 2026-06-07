use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::study_sessions::{
        dto::{
            StartStudySessionRequest, StudyHeartbeatRequest, StudySessionResponse,
            UpdateStudySessionRequest,
        },
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn start_session(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<StartStudySessionRequest>,
) -> Result<Json<ApiResponse<StudySessionResponse>>, AppError> {
    let response = service::start_session(&state, &current_user, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn active_session(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<Option<StudySessionResponse>>>, AppError> {
    let response = service::get_active_session(&state, &current_user).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_session(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(session_id): Path<i64>,
    Json(payload): Json<UpdateStudySessionRequest>,
) -> Result<Json<ApiResponse<StudySessionResponse>>, AppError> {
    let response = service::update_session(&state, &current_user, session_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn heartbeat(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(session_id): Path<i64>,
    Json(payload): Json<StudyHeartbeatRequest>,
) -> Result<Json<ApiResponse<StudySessionResponse>>, AppError> {
    let response = service::heartbeat(&state, &current_user, session_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
