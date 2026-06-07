use chrono::{DateTime, Utc};
use sqlx::{MySql, MySqlPool, QueryBuilder};

use crate::modules::rooms::{
    dto::RoomListQuery,
    model::{RoomDetailRow, RoomMemberRow, RoomSeatRow, RoomSummaryRow, StudyRoomRow},
    seat_code,
};

const ACTIVE_SESSION_STATUSES: [&str; 3] = ["studying", "paused", "resting"];

pub struct CreateRoomRecord<'a> {
    pub name: &'a str,
    pub description: Option<&'a str>,
    pub capacity: i32,
    pub is_private: bool,
    pub password: Option<&'a str>,
    pub creator_id: i64,
    pub close_at: DateTime<Utc>,
}

pub async fn find_by_name(
    pool: &MySqlPool,
    name: &str,
) -> Result<Option<StudyRoomRow>, sqlx::Error> {
    sqlx::query_as::<_, StudyRoomRow>(
        r#"
        SELECT id, name, description, capacity, is_private, password, status,
               creator_id, created_at, open_at, close_at
        FROM study_rooms
        WHERE name = ?
        "#,
    )
    .bind(name)
    .fetch_optional(pool)
    .await
}

pub async fn list_rooms(
    pool: &MySqlPool,
    query: &RoomListQuery,
) -> Result<(Vec<RoomSummaryRow>, u64), sqlx::Error> {
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

    let rooms = list_builder
        .build_query_as::<RoomSummaryRow>()
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

    Ok((rooms, total))
}

pub async fn create_room_with_seats(
    pool: &MySqlPool,
    input: CreateRoomRecord<'_>,
) -> Result<StudyRoomRow, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        r#"
        INSERT INTO study_rooms
            (name, description, capacity, is_private, password, status, creator_id,
             created_at, open_at, close_at)
        VALUES
            (?, ?, ?, ?, ?, 'open', ?, NOW(), NOW(), ?)
        "#,
    )
    .bind(input.name)
    .bind(input.description)
    .bind(input.capacity)
    .bind(if input.is_private { 1 } else { 0 })
    .bind(input.password)
    .bind(input.creator_id)
    .bind(input.close_at)
    .execute(&mut *tx)
    .await?;

    let room_id = result.last_insert_id() as i64;

    for index in 1..=input.capacity {
        sqlx::query(
            r#"
            INSERT INTO study_room_seats (room_id, seat_code, status)
            VALUES (?, ?, 'available')
            "#,
        )
        .bind(room_id)
        .bind(seat_code::seat_code(index))
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;

    find_room_by_id(pool, room_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn find_room_by_id(
    pool: &MySqlPool,
    room_id: i64,
) -> Result<Option<StudyRoomRow>, sqlx::Error> {
    sqlx::query_as::<_, StudyRoomRow>(
        r#"
        SELECT id, name, description, capacity, is_private, password, status,
               creator_id, created_at, open_at, close_at
        FROM study_rooms
        WHERE id = ?
        "#,
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_room_detail(
    pool: &MySqlPool,
    room_id: i64,
) -> Result<Option<RoomDetailRow>, sqlx::Error> {
    sqlx::query_as::<_, RoomDetailRow>(
        r#"
        SELECT r.id, r.name, r.description, r.capacity, r.is_private, r.status,
               r.creator_id, u.nickname AS creator_nickname, u.avatar_url AS creator_avatar_url,
               r.created_at, r.open_at, r.close_at
        FROM study_rooms r
        INNER JOIN users u ON u.id = r.creator_id
        WHERE r.id = ?
        "#,
    )
    .bind(room_id)
    .fetch_optional(pool)
    .await
}

pub async fn list_seats(pool: &MySqlPool, room_id: i64) -> Result<Vec<RoomSeatRow>, sqlx::Error> {
    sqlx::query_as::<_, RoomSeatRow>(
        r#"
        SELECT s.id, s.seat_code, s.status,
               u.id AS occupied_user_id,
               u.nickname AS occupied_nickname,
               u.avatar_url AS occupied_avatar_url
        FROM study_room_seats s
        LEFT JOIN study_sessions ss
          ON ss.seat_id = s.id
         AND ss.status IN ('studying', 'paused', 'resting')
        LEFT JOIN users u ON u.id = ss.user_id
        WHERE s.room_id = ?
        ORDER BY s.id ASC
        "#,
    )
    .bind(room_id)
    .fetch_all(pool)
    .await
}

pub async fn list_members(
    pool: &MySqlPool,
    room_id: i64,
) -> Result<Vec<RoomMemberRow>, sqlx::Error> {
    sqlx::query_as::<_, RoomMemberRow>(
        r#"
        SELECT DISTINCT u.id, u.nickname, u.avatar_url
        FROM study_sessions ss
        INNER JOIN users u ON u.id = ss.user_id
        WHERE ss.room_id = ?
          AND ss.status IN ('studying', 'paused', 'resting')
        ORDER BY u.id ASC
        "#,
    )
    .bind(room_id)
    .fetch_all(pool)
    .await
}

fn push_room_filters(builder: &mut QueryBuilder<MySql>, query: &RoomListQuery) {
    if let Some(keyword) = query
        .keyword
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        builder.push(" AND r.name LIKE ");
        builder.push_bind(format!("%{keyword}%"));
    }

    let status = query
        .status
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty());

    match status {
        Some("open") => {
            builder.push(" AND r.status = 'open' AND r.close_at > NOW()");
        }
        Some(status) => {
            builder.push(" AND r.status = ");
            builder.push_bind(status);
        }
        None => {
            builder.push(" AND r.status = 'open' AND r.close_at > NOW()");
        }
    }
}

pub fn active_session_statuses() -> &'static [&'static str] {
    &ACTIVE_SESSION_STATUSES
}
