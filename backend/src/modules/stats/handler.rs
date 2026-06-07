use axum::{
    Json,
    extract::{Query, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::stats::{
        dto::{PeriodQuery, TodayStatsResponse, UserStatsResponse},
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn today_stats(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<TodayStatsResponse>>, AppError> {
    let response = service::today_stats(&state, &current_user).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn user_stats(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<PeriodQuery>,
) -> Result<Json<ApiResponse<UserStatsResponse>>, AppError> {
    let response = service::user_stats(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}
