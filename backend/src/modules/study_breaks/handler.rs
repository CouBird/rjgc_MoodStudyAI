use axum::{
    Json,
    extract::{Path, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::study_breaks::{
        dto::{CreateBreakRequest, ExtendBreakRequest, StudyBreakResponse},
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn create_break(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(session_id): Path<i64>,
    Json(payload): Json<CreateBreakRequest>,
) -> Result<Json<ApiResponse<StudyBreakResponse>>, AppError> {
    let response = service::create_break(&state, &current_user, session_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn extend_break(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(break_id): Path<i64>,
    Json(payload): Json<ExtendBreakRequest>,
) -> Result<Json<ApiResponse<StudyBreakResponse>>, AppError> {
    let response = service::extend_break(&state, &current_user, break_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
