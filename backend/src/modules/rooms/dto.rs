use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    modules::rooms::model::{RoomDetailRow, RoomMemberRow, RoomSeatRow, RoomSummaryRow},
    pagination::PageResult,
};

#[derive(Debug, Deserialize)]
pub struct RoomListQuery {
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

impl RoomListQuery {
    pub fn page(&self) -> u32 {
        self.page.unwrap_or(1).max(1)
    }

    pub fn page_size(&self) -> u32 {
        self.page_size.unwrap_or(10).clamp(1, 100)
    }

    pub fn offset(&self) -> u64 {
        ((self.page() - 1) * self.page_size()) as u64
    }
}

#[derive(Debug, Deserialize)]
pub struct CreateRoomRequest {
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub password: Option<String>,
    #[serde(rename = "closeAt")]
    pub close_at: String,
}

#[derive(Debug, Serialize)]
pub struct RoomResponse {
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub name: String,
    pub description: Option<String>,
    pub status: String,
    pub capacity: i32,
    #[serde(rename = "currentMembers")]
    pub current_members: i64,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub creator: UserBriefResponse,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "openAt")]
    pub open_at: DateTime<Utc>,
    #[serde(rename = "closeAt")]
    pub close_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct UserBriefResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub nickname: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct SeatResponse {
    #[serde(rename = "seatId")]
    pub seat_id: String,
    #[serde(rename = "seatCode")]
    pub seat_code: String,
    pub status: String,
    #[serde(rename = "occupiedBy")]
    pub occupied_by: Option<UserBriefResponse>,
}

#[derive(Debug, Serialize)]
pub struct RoomDetailResponse {
    pub room: RoomResponse,
    pub seats: Vec<SeatResponse>,
    pub members: Vec<UserBriefResponse>,
}

pub type RoomListResponse = PageResult<RoomResponse>;

impl RoomResponse {
    pub fn from_summary(row: RoomSummaryRow) -> Self {
        Self {
            room_id: row.id.to_string(),
            name: row.name,
            description: row.description,
            status: row.status,
            capacity: row.capacity,
            current_members: row.current_members,
            is_private: row.is_private != 0,
            creator: UserBriefResponse {
                user_id: row.creator_id.to_string(),
                nickname: row.creator_nickname,
                avatar_url: row.creator_avatar_url,
            },
            created_at: row.created_at,
            open_at: row.open_at,
            close_at: row.close_at,
        }
    }

    pub fn from_detail(row: RoomDetailRow, current_members: i64) -> Self {
        Self {
            room_id: row.id.to_string(),
            name: row.name,
            description: row.description,
            status: row.status,
            capacity: row.capacity,
            current_members,
            is_private: row.is_private != 0,
            creator: UserBriefResponse {
                user_id: row.creator_id.to_string(),
                nickname: row.creator_nickname,
                avatar_url: row.creator_avatar_url,
            },
            created_at: row.created_at,
            open_at: row.open_at,
            close_at: row.close_at,
        }
    }
}

impl SeatResponse {
    pub fn from_row(row: RoomSeatRow) -> Self {
        Self {
            seat_id: row.id.to_string(),
            seat_code: row.seat_code,
            status: row.status,
            occupied_by: row.occupied_user_id.map(|user_id| UserBriefResponse {
                user_id: user_id.to_string(),
                nickname: row.occupied_nickname.unwrap_or_default(),
                avatar_url: row.occupied_avatar_url,
            }),
        }
    }
}

impl From<RoomMemberRow> for UserBriefResponse {
    fn from(row: RoomMemberRow) -> Self {
        Self {
            user_id: row.id.to_string(),
            nickname: row.nickname,
            avatar_url: row.avatar_url,
        }
    }
}
