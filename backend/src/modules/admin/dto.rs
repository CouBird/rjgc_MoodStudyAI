use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::{
    modules::admin::model::{
        AdminRoomListRow, AdminUserListRow, AdminUserRow, AuditLogListRow, EmotionDistributionRow,
    },
    pagination::PageResult,
};

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub account: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminListQuery {
    pub keyword: Option<String>,
    pub status: Option<String>,
    pub page: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

impl AdminListQuery {
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
pub struct AdminAuditLogQuery {
    #[serde(rename = "type")]
    pub action_type: Option<String>,
    #[serde(rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(rename = "endDate")]
    pub end_date: Option<String>,
    pub page: Option<u32>,
    #[serde(rename = "pageSize")]
    pub page_size: Option<u32>,
}

impl AdminAuditLogQuery {
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
pub struct UpdateUserStatusRequest {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateRoomStatusRequest {
    pub status: String,
    pub reason: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminResponse {
    #[serde(rename = "adminId")]
    pub admin_id: String,
    #[serde(rename = "adminName")]
    pub admin_name: String,
    pub role: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminAuthResponse {
    pub admin: AdminResponse,
    #[serde(rename = "adminToken")]
    pub admin_token: String,
}

#[derive(Debug, Serialize)]
pub struct AdminUserResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub nickname: String,
    pub phone: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
    pub profile: Option<String>,
    pub status: String,
    #[serde(rename = "streakDays")]
    pub streak_days: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct AdminRoomCreatorResponse {
    #[serde(rename = "userId")]
    pub user_id: String,
    pub nickname: String,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminRoomResponse {
    #[serde(rename = "roomId")]
    pub room_id: String,
    pub name: String,
    pub description: Option<String>,
    pub capacity: i32,
    #[serde(rename = "isPrivate")]
    pub is_private: bool,
    pub status: String,
    #[serde(rename = "currentMembers")]
    pub current_members: i64,
    pub creator: AdminRoomCreatorResponse,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "openAt")]
    pub open_at: DateTime<Utc>,
    #[serde(rename = "closeAt")]
    pub close_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct EmotionDistributionResponse {
    #[serde(rename = "emotionTag")]
    pub emotion_tag: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct AdminDashboardResponse {
    #[serde(rename = "totalUsers")]
    pub total_users: i64,
    #[serde(rename = "activeUsers")]
    pub active_users: i64,
    #[serde(rename = "disabledUsers")]
    pub disabled_users: i64,
    #[serde(rename = "totalRooms")]
    pub total_rooms: i64,
    #[serde(rename = "openRooms")]
    pub open_rooms: i64,
    #[serde(rename = "closedRooms")]
    pub closed_rooms: i64,
    #[serde(rename = "currentOnlineUsers")]
    pub current_online_users: i64,
    #[serde(rename = "todayStudyMinutes")]
    pub today_study_minutes: i64,
    #[serde(rename = "todayStudyHours")]
    pub today_study_hours: f64,
    #[serde(rename = "todayCheckins")]
    pub today_checkins: i64,
    #[serde(rename = "emotionDistribution")]
    pub emotion_distribution: Vec<EmotionDistributionResponse>,
}

#[derive(Debug, Serialize)]
pub struct AuditLogResponse {
    #[serde(rename = "logId")]
    pub log_id: String,
    #[serde(rename = "adminId")]
    pub admin_id: String,
    #[serde(rename = "adminName")]
    pub admin_name: String,
    pub action: String,
    #[serde(rename = "targetType")]
    pub target_type: String,
    #[serde(rename = "targetId")]
    pub target_id: String,
    pub reason: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
}

pub type AdminUserListResponse = PageResult<AdminUserResponse>;
pub type AdminRoomListResponse = PageResult<AdminRoomResponse>;
pub type AuditLogListResponse = PageResult<AuditLogResponse>;

impl AdminResponse {
    pub fn from_row(row: AdminUserRow) -> Self {
        Self {
            admin_id: row.id.to_string(),
            admin_name: row.admin_name,
            role: row.role,
            created_at: row.created_at,
        }
    }
}

impl From<AdminUserListRow> for AdminUserResponse {
    fn from(row: AdminUserListRow) -> Self {
        Self {
            user_id: row.id.to_string(),
            nickname: row.nickname,
            phone: row.phone,
            avatar_url: row.avatar_url,
            profile: row.profile,
            status: row.status,
            streak_days: row.streak_days,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }
    }
}

impl From<AdminRoomListRow> for AdminRoomResponse {
    fn from(row: AdminRoomListRow) -> Self {
        Self {
            room_id: row.id.to_string(),
            name: row.name,
            description: row.description,
            capacity: row.capacity,
            is_private: row.is_private != 0,
            status: row.status,
            current_members: row.current_members,
            creator: AdminRoomCreatorResponse {
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

impl From<EmotionDistributionRow> for EmotionDistributionResponse {
    fn from(row: EmotionDistributionRow) -> Self {
        Self {
            emotion_tag: row.emotion_tag,
            count: row.count,
        }
    }
}

impl From<AuditLogListRow> for AuditLogResponse {
    fn from(row: AuditLogListRow) -> Self {
        Self {
            log_id: row.id.to_string(),
            admin_id: row.admin_id.to_string(),
            admin_name: row.admin_name,
            action: row.action,
            target_type: row.target_type,
            target_id: row.target_id.to_string(),
            reason: row.reason,
            created_at: row.created_at,
        }
    }
}
