use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct AdminUserRow {
    pub id: i64,
    pub admin_name: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, sqlx::FromRow)]
pub struct AuditLogRow {
    pub id: i64,
    pub admin_id: i64,
    pub action: String,
    pub target_type: String,
    pub target_id: i64,
    pub reason: Option<String>,
    pub created_at: DateTime<Utc>,
}
