use axum::{Json, extract::State};

use crate::{
    error::AppError,
    modules::admin::{
        dto::{AdminAuthResponse, AdminLoginRequest},
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<AdminLoginRequest>,
) -> Result<Json<ApiResponse<AdminAuthResponse>>, AppError> {
    let response = service::login(&state, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}
