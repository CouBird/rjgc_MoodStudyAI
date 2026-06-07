use axum::{Json, extract::State};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::{error::AppError, response::ApiResponse, state::AppState};

#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub service: &'static str,
    pub status: &'static str,
    pub database: &'static str,
    pub redis: &'static str,
    pub timestamp: DateTime<Utc>,
}

pub async fn health(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<HealthResponse>>, AppError> {
    let database = state.database_status().await;
    let redis = state.redis_status();

    Ok(Json(ApiResponse::ok(HealthResponse {
        service: "ai-study-room-backend",
        status: "ok",
        database,
        redis,
        timestamp: Utc::now(),
    })))
}
