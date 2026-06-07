use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::modules::admin::model::AdminUserRow;

#[derive(Debug, Deserialize)]
pub struct AdminLoginRequest {
    pub account: String,
    pub password: String,
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
