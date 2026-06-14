use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::admin::{
        dto::{
            AdminAuditLogQuery, AdminAuthResponse, AdminDashboardResponse, AdminListQuery,
            AdminLoginRequest, AdminRoomListResponse, AdminRoomResponse, AdminUserListResponse,
            AdminUserResponse, AuditLogListResponse, UpdateRoomStatusRequest,
            UpdateUserStatusRequest,
        },
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

pub async fn list_users(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<AdminListQuery>,
) -> Result<Json<ApiResponse<AdminUserListResponse>>, AppError> {
    let response = service::list_users(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_user_status(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(user_id): Path<i64>,
    Json(payload): Json<UpdateUserStatusRequest>,
) -> Result<Json<ApiResponse<AdminUserResponse>>, AppError> {
    let response = service::update_user_status(&state, &current_user, user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn list_rooms(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<AdminListQuery>,
) -> Result<Json<ApiResponse<AdminRoomListResponse>>, AppError> {
    let response = service::list_rooms(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_room_status(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(room_id): Path<i64>,
    Json(payload): Json<UpdateRoomStatusRequest>,
) -> Result<Json<ApiResponse<AdminRoomResponse>>, AppError> {
    let response = service::update_room_status(&state, &current_user, room_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn dashboard(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<AdminDashboardResponse>>, AppError> {
    let response = service::dashboard(&state, &current_user).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn audit_logs(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<AdminAuditLogQuery>,
) -> Result<Json<ApiResponse<AuditLogListResponse>>, AppError> {
    let response = service::audit_logs(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}
