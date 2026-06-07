use chrono::{DateTime, Duration, Utc};

use crate::{
    auth::extractor::CurrentUser,
    constants::{limits, roles, statuses},
    error::AppError,
    modules::{
        rooms::{
            dto::{
                CreateRoomRequest, RoomDetailResponse, RoomListQuery, RoomListResponse,
                RoomResponse, SeatResponse, UserBriefResponse,
            },
            repository,
        },
        study_sessions,
    },
    pagination::PageResult,
    state::AppState,
};

pub async fn list_rooms(
    state: &AppState,
    current_user: &CurrentUser,
    query: RoomListQuery,
) -> Result<RoomListResponse, AppError> {
    require_user(current_user)?;
    validate_status_filter(query.status.as_deref())?;

    let pool = state.require_db()?;
    study_sessions::repository::cleanup_inactive_sessions(pool).await?;
    let page = query.page();
    let page_size = query.page_size();
    let (rows, total) = repository::list_rooms(pool, &query).await?;
    let items = rows.into_iter().map(RoomResponse::from_summary).collect();

    Ok(PageResult {
        items,
        total,
        page,
        page_size,
    })
}

pub async fn create_room(
    state: &AppState,
    current_user: &CurrentUser,
    payload: CreateRoomRequest,
) -> Result<RoomResponse, AppError> {
    require_user(current_user)?;
    validate_create_room_request(&payload)?;

    let pool = state.require_db()?;
    let name = payload.name.trim();

    if repository::find_by_name(pool, name).await?.is_some() {
        return Err(AppError::Conflict("房间名称已存在".to_string()));
    }

    let close_at = parse_close_at(&payload.close_at)?;
    let row = repository::create_room_with_seats(
        pool,
        repository::CreateRoomRecord {
            name,
            description: payload
                .description
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty()),
            capacity: payload.capacity,
            is_private: payload.is_private,
            password: payload
                .password
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty()),
            creator_id: current_user.user_id,
            close_at,
        },
    )
    .await?;

    let detail = repository::find_room_detail(pool, row.id)
        .await?
        .ok_or_else(|| AppError::NotFound("自习室不存在".to_string()))?;

    Ok(RoomResponse::from_detail(detail, 0))
}

pub async fn get_room_detail(
    state: &AppState,
    current_user: &CurrentUser,
    room_id: i64,
) -> Result<RoomDetailResponse, AppError> {
    require_user(current_user)?;

    let pool = state.require_db()?;
    study_sessions::repository::cleanup_inactive_sessions(pool).await?;
    let room = repository::find_room_detail(pool, room_id)
        .await?
        .ok_or_else(|| AppError::NotFound("自习室不存在".to_string()))?;
    let seats = repository::list_seats(pool, room_id).await?;
    let members = repository::list_members(pool, room_id).await?;
    let current_members = members.len() as i64;

    Ok(RoomDetailResponse {
        room: RoomResponse::from_detail(room, current_members),
        seats: seats.into_iter().map(SeatResponse::from_row).collect(),
        members: members.into_iter().map(UserBriefResponse::from).collect(),
    })
}

pub async fn list_room_seats(
    state: &AppState,
    current_user: &CurrentUser,
    room_id: i64,
) -> Result<Vec<SeatResponse>, AppError> {
    require_user(current_user)?;

    let pool = state.require_db()?;
    study_sessions::repository::cleanup_inactive_sessions(pool).await?;
    if repository::find_room_by_id(pool, room_id).await?.is_none() {
        return Err(AppError::NotFound("自习室不存在".to_string()));
    }

    let seats = repository::list_seats(pool, room_id).await?;
    Ok(seats.into_iter().map(SeatResponse::from_row).collect())
}

fn require_user(current_user: &CurrentUser) -> Result<(), AppError> {
    if current_user.role == roles::USER {
        Ok(())
    } else {
        Err(AppError::Forbidden)
    }
}

fn validate_status_filter(status: Option<&str>) -> Result<(), AppError> {
    match status {
        None | Some("") => Ok(()),
        Some(status) if status == statuses::room::OPEN || status == statuses::room::CLOSED => {
            Ok(())
        }
        Some(_) => Err(AppError::Validation(
            "房间状态仅支持 open 或 closed".to_string(),
        )),
    }
}

fn validate_create_room_request(payload: &CreateRoomRequest) -> Result<(), AppError> {
    let name = payload.name.trim();

    if name.is_empty() || name.chars().count() > 20 {
        return Err(AppError::Validation(
            "房间名称不能为空且不能超过20个字符".to_string(),
        ));
    }

    if payload.capacity < 1 || payload.capacity > limits::MAX_ROOM_CAPACITY {
        return Err(AppError::Unprocessable("容量必须在1-50之间".to_string()));
    }

    if let Some(description) = &payload.description
        && description.chars().count() > 255
    {
        return Err(AppError::Validation(
            "房间描述不能超过255个字符".to_string(),
        ));
    }

    let close_at = parse_close_at(&payload.close_at)?;
    if close_at < Utc::now() + Duration::hours(1) {
        return Err(AppError::Unprocessable(
            "开放截止时间不得早于当前时间后1小时".to_string(),
        ));
    }

    Ok(())
}

fn parse_close_at(value: &str) -> Result<DateTime<Utc>, AppError> {
    DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|_| AppError::Validation("closeAt 必须为 RFC3339 时间".to_string()))
}
