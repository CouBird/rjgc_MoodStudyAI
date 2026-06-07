use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::checkins::{
        dto::{CheckinCalendarResponse, CheckinListQuery, CheckinResponse, CreateCheckinRequest},
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn list_checkins(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<CheckinListQuery>,
) -> Result<Json<ApiResponse<CheckinCalendarResponse>>, AppError> {
    let response = service::list_checkins(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn get_checkin_detail(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(date): Path<String>,
) -> Result<Json<ApiResponse<CheckinResponse>>, AppError> {
    let response = service::get_checkin_detail(&state, &current_user, date).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create_checkin(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<CreateCheckinRequest>,
) -> Result<Json<ApiResponse<CheckinResponse>>, AppError> {
    let response = service::create_makeup_checkin(&state, &current_user, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
