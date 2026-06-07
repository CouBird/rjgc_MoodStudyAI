use axum::{Json, extract::State};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::users::{
        dto::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let response = service::register(&state, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let response = service::login(&state, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn me(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let response = service::get_current_user(&state, current_user.user_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
