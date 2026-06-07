use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::emotions::{
        dto::{
            CreateEmotionRecordRequest, CreateEmotionRecordResponse, EmotionTrendQuery,
            EmotionTrendResponse,
        },
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn create_emotion_record(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(session_id): Path<i64>,
    Json(payload): Json<CreateEmotionRecordRequest>,
) -> Result<Json<ApiResponse<CreateEmotionRecordResponse>>, AppError> {
    let response =
        service::create_emotion_record(&state, &current_user, session_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn emotion_trends(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<EmotionTrendQuery>,
) -> Result<Json<ApiResponse<EmotionTrendResponse>>, AppError> {
    let response = service::emotion_trends(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}
