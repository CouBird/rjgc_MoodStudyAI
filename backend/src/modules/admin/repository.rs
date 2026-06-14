use chrono::{DateTime, NaiveDate, Utc};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::modules::admin::{
    dto::{AdminAuditLogQuery, AdminListQuery},
    model::{
        AdminRoomListRow, AdminUserListRow, AdminUserRow, AuditLogListRow, EmotionDistributionRow,
    },
};

pub async fn find_by_account(
    pool: &MySqlPool,
    account: &str,
) -> Result<Option<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, admin_name, password_hash, role, created_at
        FROM admin_users
        WHERE admin_name = ?
        "#,
    )
    .bind(account)
    .fetch_optional(pool)
    .await
}

pub async fn list_users(
    pool: &MySqlPool,
    query: &AdminListQuery,
) -> Result<(Vec<AdminUserListRow>, u64), sqlx::Error> {
    let mut list_builder = QueryBuilder::<MySql>::new(
        r#"
        SELECT id, nickname, phone, avatar_url, profile, status,
               created_at, updated_at, streak_days
        FROM users
        WHERE 1 = 1
        "#,
    );
    push_user_filters(&mut list_builder, query);
    list_builder.push(" ORDER BY created_at DESC LIMIT ");
    list_builder.push_bind(query.page_size() as i64);
    list_builder.push(" OFFSET ");
    list_builder.push_bind(query.offset() as i64);

    let rows = list_builder
        .build_query_as::<AdminUserListRow>()
        .fetch_all(pool)
        .await?;

    let mut count_builder = QueryBuilder::<MySql>::new(
        r#"
        SELECT COUNT(*)
        FROM users
        WHERE 1 = 1
        "#,
    );
    push_user_filters(&mut count_builder, query);
    let total = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await?
        .max(0) as u64;

    Ok((rows, total))
}

pub async fn find_user_by_id(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Option<AdminUserListRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserListRow>(
        r#"
        SELECT id, nickname, phone, avatar_url, profile, status,
               created_at, updated_at, streak_days
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_user_status(
    pool: &MySqlPool,
    user_id: i64,
    status: &str,
) -> Result<Option<AdminUserListRow>, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE users
        SET status = ?, updated_at = NOW()
        WHERE id = ?
        "#,
    )
    .bind(status)
    .bind(user_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    find_user_by_id(pool, user_id).await
}

pub async fn list_rooms(
    pool: &MySqlPool,
    query: &AdminListQuery,
) -> Result<(Vec<AdminRoomListRow>, u64), sqlx::Error> {
    let mut list_builder = QueryBuilder::<MySql>::new(
        r#"
        SELECT r.id, r.name, r.description, r.capacity, r.is_private, r.status,
               r.creator_id, u.nickname AS creator_nickname, u.avatar_url AS creator_avatar_url,
               COALESCE(active.current_members, 0) AS current_members,
               r.created_at, r.open_at, r.close_at
        FROM study_rooms r
        INNER JOIN users u ON u.id = r.creator_id
        LEFT JOIN (
            SELECT room_id, COUNT(*) AS current_members
            FROM study_sessions
            WHERE status IN ('studying', 'paused', 'resting')
            GROUP BY room_id
        ) active ON active.room_id = r.id
        WHERE 1 = 1
        "#,
    );
    push_room_filters(&mut list_builder, query);
    list_builder.push(" ORDER BY r.created_at DESC LIMIT ");
    list_builder.push_bind(query.page_size() as i64);
    list_builder.push(" OFFSET ");
    list_builder.push_bind(query.offset() as i64);

    let rows = list_builder
        .build_query_as::<AdminRoomListRow>()
        .fetch_all(pool)
        .await?;

    let mut count_builder = QueryBuilder::<MySql>::new(
        r#"
        SELECT COUNT(*)
        FROM study_rooms r
        WHERE 1 = 1
        "#,
    );
    push_room_filters(&mut count_builder, query);
    let total = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await?
        .max(0) as u64;

    Ok((rows, total))
}

pub async fn find_room_by_id(
    pool: &MySqlPool,
    room_id: i64,
) -> Result<Option<AdminRoomListRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminRoomListRow>(
        r#"
        SELECT r.id, r.name, r.description, r.capacity, r.is_private, r.status,
               r.creator_id, u.nickname AS creator_nickname, u.avatar_url AS creator_avatar_url,
               COALESCE(active.current_members, 0) AS current_members,
               r.created_at, r.open_at, r.close_at
        FROM study_rooms r
        INNER JOIN users u ON u.id = r.creator_id
        LEFT JOIN (
            SELECT room_id, COUNT(*) AS current_members
            FROM study_sessions
            WHERE status IN ('studying', 'paused', 'resting')
            GROUP BY room_id
        ) active ON active.room_id = r.id
        WHERE r.id = ?
        "#,
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await
}

pub async fn update_room_status(
    pool: &MySqlPool,
    room_id: i64,
    status: &str,
) -> Result<Option<AdminRoomListRow>, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE study_rooms
        SET status = ?
        WHERE id = ?
        "#,
    )
    .bind(status)
    .bind(room_id)
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Ok(None);
    }

    find_room_by_id(pool, room_id).await
}

pub async fn total_users(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    count_scalar(pool, "SELECT COUNT(*) FROM users").await
}

pub async fn users_by_status(pool: &MySqlPool, status: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE status = ?")
        .bind(status)
        .fetch_one(pool)
        .await
}

pub async fn total_rooms(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    count_scalar(pool, "SELECT COUNT(*) FROM study_rooms").await
}

pub async fn rooms_by_status(pool: &MySqlPool, status: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM study_rooms WHERE status = ?")
        .bind(status)
        .fetch_one(pool)
        .await
}

pub async fn current_online_users(pool: &MySqlPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(DISTINCT user_id)
        FROM study_sessions
        WHERE status IN ('studying', 'paused', 'resting')
        "#,
    )
    .fetch_one(pool)
    .await
}

pub async fn today_study_minutes(pool: &MySqlPool, today: NaiveDate) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, Option<i64>>(
        r#"
        SELECT CAST(COALESCE(SUM(total_minutes), 0) AS SIGNED)
        FROM checkin_records
        WHERE checkin_date = ?
        "#,
    )
    .bind(today)
    .fetch_one(pool)
    .await
    .map(|value| value.unwrap_or_default())
}

pub async fn today_checkins(pool: &MySqlPool, today: NaiveDate) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM checkin_records
        WHERE checkin_date = ?
        "#,
    )
    .bind(today)
    .fetch_one(pool)
    .await
}

pub async fn today_emotion_distribution(
    pool: &MySqlPool,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> Result<Vec<EmotionDistributionRow>, sqlx::Error> {
    sqlx::query_as::<_, EmotionDistributionRow>(
        r#"
        SELECT er.emotion_tag, COUNT(*) AS count
        FROM emotion_records er
        WHERE er.created_at >= ?
          AND er.created_at < ?
        GROUP BY er.emotion_tag
        ORDER BY count DESC, er.emotion_tag ASC
        "#,
    )
    .bind(start_at)
    .bind(end_at)
    .fetch_all(pool)
    .await
}

pub async fn list_audit_logs(
    pool: &MySqlPool,
    query: &AdminAuditLogQuery,
    action: Option<&str>,
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
) -> Result<(Vec<AuditLogListRow>, u64), sqlx::Error> {
    let mut list_builder = QueryBuilder::<MySql>::new(
        r#"
        SELECT l.id, l.admin_id, a.admin_name, l.action, l.target_type,
               l.target_id, l.reason, l.created_at
        FROM audit_logs l
        INNER JOIN admin_users a ON a.id = l.admin_id
        WHERE 1 = 1
        "#,
    );
    push_audit_filters(&mut list_builder, action, start_at, end_at);
    list_builder.push(" ORDER BY l.created_at DESC LIMIT ");
    list_builder.push_bind(query.page_size() as i64);
    list_builder.push(" OFFSET ");
    list_builder.push_bind(query.offset() as i64);

    let rows = list_builder
        .build_query_as::<AuditLogListRow>()
        .fetch_all(pool)
        .await?;

    let mut count_builder = QueryBuilder::<MySql>::new(
        r#"
        SELECT COUNT(*)
        FROM audit_logs l
        WHERE 1 = 1
        "#,
    );
    push_audit_filters(&mut count_builder, action, start_at, end_at);
    let total = count_builder
        .build_query_scalar::<i64>()
        .fetch_one(pool)
        .await?
        .max(0) as u64;

    Ok((rows, total))
}

pub async fn insert_audit_log(
    pool: &MySqlPool,
    admin_id: i64,
    action: &str,
    target_type: &str,
    target_id: i64,
    reason: Option<&str>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO audit_logs
            (admin_id, action, target_type, target_id, reason, created_at)
        VALUES
            (?, ?, ?, ?, ?, NOW())
        "#,
    )
    .bind(admin_id)
    .bind(action)
    .bind(target_type)
    .bind(target_id)
    .bind(reason)
    .execute(pool)
    .await?;

    Ok(())
}

async fn count_scalar(pool: &MySqlPool, sql: &'static str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(sql).fetch_one(pool).await
}

fn push_user_filters(builder: &mut QueryBuilder<MySql>, query: &AdminListQuery) {
    if let Some(keyword) = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder.push(" AND (phone LIKE ");
        builder.push_bind(format!("%{keyword}%"));
        builder.push(" OR nickname LIKE ");
        builder.push_bind(format!("%{keyword}%"));
        builder.push(")");
    }

    if let Some(status) = query
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder.push(" AND status = ");
        builder.push_bind(status.to_string());
    }
}

fn push_room_filters(builder: &mut QueryBuilder<MySql>, query: &AdminListQuery) {
    if let Some(keyword) = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder.push(" AND r.name LIKE ");
        builder.push_bind(format!("%{keyword}%"));
    }

    if let Some(status) = query
        .status
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        builder.push(" AND r.status = ");
        builder.push_bind(status.to_string());
    }
}

fn push_audit_filters(
    builder: &mut QueryBuilder<MySql>,
    action: Option<&str>,
    start_at: Option<DateTime<Utc>>,
    end_at: Option<DateTime<Utc>>,
) {
    if let Some(action) = action {
        builder.push(" AND l.action = ");
        builder.push_bind(action.to_string());
    }

    if let Some(start_at) = start_at {
        builder.push(" AND l.created_at >= ");
        builder.push_bind(start_at);
    }

    if let Some(end_at) = end_at {
        builder.push(" AND l.created_at < ");
        builder.push_bind(end_at);
    }
}
