use axum::{
    Json,
    extract::{Path, Query, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::rooms::{
        dto::{
            CreateRoomRequest, RoomDetailResponse, RoomListQuery, RoomListResponse, RoomResponse,
        },
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn list_rooms(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Query(query): Query<RoomListQuery>,
) -> Result<Json<ApiResponse<RoomListResponse>>, AppError> {
    let response = service::list_rooms(&state, &current_user, query).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn create_room(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<CreateRoomRequest>,
) -> Result<Json<ApiResponse<RoomResponse>>, AppError> {
    let response = service::create_room(&state, &current_user, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn get_room_detail(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(room_id): Path<i64>,
) -> Result<Json<ApiResponse<RoomDetailResponse>>, AppError> {
    let response = service::get_room_detail(&state, &current_user, room_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn list_room_seats(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(room_id): Path<i64>,
) -> Result<Json<ApiResponse<Vec<crate::modules::rooms::dto::SeatResponse>>>, AppError> {
    let response = service::list_room_seats(&state, &current_user, room_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}
