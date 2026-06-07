use std::time::Duration as StdDuration;

use chrono::{DateTime, Duration, Utc};
use sqlx::{MySql, MySqlPool, Transaction};
use tokio::time::sleep;

use crate::{
    constants::limits,
    modules::{
        checkins,
        study_sessions::{
            duration,
            model::{RoomLockRow, SeatLockRow, StudySessionDetailRow, StudySessionRow},
            state_machine,
        },
    },
};

pub const ACTIVE_SESSION_STATUSES: [&str; 3] = ["studying", "paused", "resting"];

#[derive(Debug)]
pub enum StartStudySessionError {
    ActiveSession,
    RoomNotFound,
    RoomClosed,
    RoomExpired,
    RoomFull,
    SeatNotFound,
    SeatRoomMismatch,
    SeatOccupied,
    Database(sqlx::Error),
}

#[derive(Debug)]
pub enum UpdateStudySessionError {
    SessionNotFound,
    Forbidden,
    AlreadyEnded,
    InvalidTransition { from: String, to: String },
    Database(sqlx::Error),
}

#[derive(Debug)]
pub enum HeartbeatError {
    SessionNotFound,
    Forbidden,
    AlreadyEnded,
    Database(sqlx::Error),
}

pub struct CreateStudySessionRecord<'a> {
    pub room_id: i64,
    pub seat_id: i64,
    pub user_id: i64,
    pub mode: &'a str,
    pub study_content: Option<&'a str>,
}

pub async fn cleanup_inactive_sessions(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let mut attempts = 0;
    loop {
        attempts += 1;
        match cleanup_inactive_sessions_once(pool).await {
            Ok(()) => return Ok(()),
            Err(error) if attempts < 3 && is_lock_contention(&error) => {
                sleep(StdDuration::from_millis(20)).await;
            }
            Err(error) => return Err(error),
        }
    }
}

async fn cleanup_inactive_sessions_once(pool: &MySqlPool) -> Result<(), sqlx::Error> {
    let now = Utc::now();
    let timeout_before = now - Duration::minutes(limits::HEARTBEAT_TIMEOUT_MINUTES);
    let mut tx = pool.begin().await?;

    sqlx::query(
        r#"
        UPDATE study_breaks b
        INNER JOIN study_sessions ss ON ss.id = b.session_id
        SET b.end_time = DATE_ADD(b.start_time, INTERVAL b.duration MINUTE),
            ss.status = 'studying',
            ss.last_heartbeat_at = DATE_ADD(b.start_time, INTERVAL b.duration MINUTE)
        WHERE ss.status = 'resting'
          AND b.end_time IS NULL
          AND DATE_ADD(b.start_time, INTERVAL b.duration MINUTE) <= ?
        "#,
    )
    .bind(now)
    .execute(&mut *tx)
    .await?;

    let timed_out_sessions = sqlx::query_as::<_, StudySessionRow>(
        r#"
        SELECT id, room_id, user_id, seat_id, mode, study_content, start_time,
               end_time, duration_minutes, is_valid, status, last_heartbeat_at
        FROM study_sessions
        WHERE status IN ('studying', 'paused')
          AND COALESCE(last_heartbeat_at, start_time) <= ?
        FOR UPDATE SKIP LOCKED
        "#,
    )
    .bind(timeout_before)
    .fetch_all(&mut *tx)
    .await?;

    let today = now.date_naive();
    for session in timed_out_sessions {
        end_session_for_timeout(&mut tx, session, today).await?;
    }

    tx.commit().await?;
    Ok(())
}

fn is_lock_contention(error: &sqlx::Error) -> bool {
    let sqlx::Error::Database(db_error) = error else {
        return false;
    };

    matches!(db_error.code().as_deref(), Some("1213" | "1205"))
}

pub async fn find_active_by_user(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Option<StudySessionDetailRow>, sqlx::Error> {
    sqlx::query_as::<_, StudySessionDetailRow>(
        r#"
        SELECT ss.id, ss.room_id, r.name AS room_name, ss.user_id, ss.seat_id,
               s.seat_code, ss.mode, ss.study_content, ss.start_time, ss.end_time,
               ss.duration_minutes, ss.is_valid, ss.status, ss.last_heartbeat_at
        FROM study_sessions ss
        INNER JOIN study_rooms r ON r.id = ss.room_id
        INNER JOIN study_room_seats s ON s.id = ss.seat_id
        WHERE ss.user_id = ?
          AND ss.status IN ('studying', 'paused', 'resting')
        ORDER BY ss.start_time DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

async fn end_session_for_timeout(
    tx: &mut Transaction<'_, MySql>,
    session: StudySessionRow,
    recalculate_base_date: chrono::NaiveDate,
) -> Result<(), sqlx::Error> {
    let end_at = session.last_heartbeat_at.unwrap_or(session.start_time);
    let duration_minutes = duration::whole_minutes_between(session.start_time, end_at) as i32;
    let is_valid = duration_minutes >= limits::MIN_VALID_STUDY_MINUTES;

    sqlx::query(
        r#"
        UPDATE study_sessions
        SET status = 'ended',
            end_time = ?,
            duration_minutes = ?,
            is_valid = ?
        WHERE id = ?
        "#,
    )
    .bind(end_at)
    .bind(duration_minutes)
    .bind(if is_valid { 1 } else { 0 })
    .bind(session.id)
    .execute(&mut **tx)
    .await?;

    sqlx::query(
        r#"
        UPDATE study_room_seats
        SET status = 'available'
        WHERE id = ?
        "#,
    )
    .bind(session.seat_id)
    .execute(&mut **tx)
    .await?;

    if is_valid {
        let checkin_date = end_at.date_naive();
        checkins::repository::upsert_valid_study_checkin(
            tx,
            session.user_id,
            checkin_date,
            duration_minutes,
        )
        .await?;
        checkins::repository::attach_session_emotion_to_checkin(
            tx,
            session.user_id,
            checkin_date,
            session.id,
        )
        .await?;
        checkins::repository::recalculate_streak_days(tx, session.user_id, recalculate_base_date)
            .await?;
    }

    Ok(())
}

pub async fn find_detail_by_id(
    pool: &MySqlPool,
    session_id: i64,
) -> Result<Option<StudySessionDetailRow>, sqlx::Error> {
    sqlx::query_as::<_, StudySessionDetailRow>(
        r#"
        SELECT ss.id, ss.room_id, r.name AS room_name, ss.user_id, ss.seat_id,
               s.seat_code, ss.mode, ss.study_content, ss.start_time, ss.end_time,
               ss.duration_minutes, ss.is_valid, ss.status, ss.last_heartbeat_at
        FROM study_sessions ss
        INNER JOIN study_rooms r ON r.id = ss.room_id
        INNER JOIN study_room_seats s ON s.id = ss.seat_id
        WHERE ss.id = ?
        "#,
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_session_transaction(
    pool: &MySqlPool,
    input: CreateStudySessionRecord<'_>,
) -> Result<StudySessionDetailRow, StartStudySessionError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(StartStudySessionError::Database)?;

    if find_active_by_user_for_update(&mut tx, input.user_id)
        .await
        .map_err(StartStudySessionError::Database)?
        .is_some()
    {
        return Err(StartStudySessionError::ActiveSession);
    }

    let room = find_room_for_update(&mut tx, input.room_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => StartStudySessionError::RoomNotFound,
            err => StartStudySessionError::Database(err),
        })?;

    if room.status != "open" {
        return Err(StartStudySessionError::RoomClosed);
    }

    if room.close_at <= Utc::now() {
        return Err(StartStudySessionError::RoomExpired);
    }

    let active_count = count_active_in_room(&mut tx, input.room_id)
        .await
        .map_err(StartStudySessionError::Database)?;

    if active_count >= i64::from(room.capacity) {
        return Err(StartStudySessionError::RoomFull);
    }

    let seat = find_seat_for_update(&mut tx, input.seat_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => StartStudySessionError::SeatNotFound,
            err => StartStudySessionError::Database(err),
        })?;

    if seat.room_id != room.id {
        return Err(StartStudySessionError::SeatRoomMismatch);
    }

    if seat.status != "available" {
        return Err(StartStudySessionError::SeatOccupied);
    }

    let result = sqlx::query(
        r#"
        INSERT INTO study_sessions
            (room_id, user_id, seat_id, mode, study_content, start_time,
             duration_minutes, is_valid, status, last_heartbeat_at)
        VALUES
            (?, ?, ?, ?, ?, NOW(), 0, 1, 'studying', NOW())
        "#,
    )
    .bind(input.room_id)
    .bind(input.user_id)
    .bind(input.seat_id)
    .bind(input.mode)
    .bind(input.study_content)
    .execute(&mut *tx)
    .await
    .map_err(StartStudySessionError::Database)?;

    sqlx::query(
        r#"
        UPDATE study_room_seats
        SET status = 'occupied'
        WHERE id = ?
        "#,
    )
    .bind(input.seat_id)
    .execute(&mut *tx)
    .await
    .map_err(StartStudySessionError::Database)?;

    let session_id = result.last_insert_id() as i64;
    tx.commit()
        .await
        .map_err(StartStudySessionError::Database)?;

    find_detail_by_id(pool, session_id)
        .await
        .map_err(StartStudySessionError::Database)?
        .ok_or(StartStudySessionError::Database(sqlx::Error::RowNotFound))
}

pub async fn update_status_transaction(
    pool: &MySqlPool,
    session_id: i64,
    user_id: i64,
    next_status: &str,
    study_content: Option<&str>,
    ended_at: Option<DateTime<Utc>>,
) -> Result<StudySessionDetailRow, UpdateStudySessionError> {
    let mut tx = pool
        .begin()
        .await
        .map_err(UpdateStudySessionError::Database)?;
    let session = find_session_for_update(&mut tx, session_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => UpdateStudySessionError::SessionNotFound,
            err => UpdateStudySessionError::Database(err),
        })?;

    if session.user_id != user_id {
        return Err(UpdateStudySessionError::Forbidden);
    }

    if session.status == "ended" {
        return Err(UpdateStudySessionError::AlreadyEnded);
    }

    if !state_machine::can_transition(&session.status, next_status) {
        return Err(UpdateStudySessionError::InvalidTransition {
            from: session.status,
            to: next_status.to_string(),
        });
    }

    if next_status == "ended" {
        let now = ended_at.unwrap_or_else(Utc::now);
        let duration_minutes = duration::whole_minutes_between(session.start_time, now) as i32;
        let is_valid = duration_minutes >= 10;

        sqlx::query(
            r#"
            UPDATE study_sessions
            SET status = 'ended',
                end_time = ?,
                duration_minutes = ?,
                is_valid = ?,
                study_content = COALESCE(?, study_content)
            WHERE id = ?
            "#,
        )
        .bind(now)
        .bind(duration_minutes)
        .bind(if is_valid { 1 } else { 0 })
        .bind(study_content)
        .bind(session_id)
        .execute(&mut *tx)
        .await
        .map_err(UpdateStudySessionError::Database)?;

        sqlx::query(
            r#"
            UPDATE study_room_seats
            SET status = 'available'
            WHERE id = ?
            "#,
        )
        .bind(session.seat_id)
        .execute(&mut *tx)
        .await
        .map_err(UpdateStudySessionError::Database)?;

        if is_valid {
            let checkin_date = now.date_naive();
            checkins::repository::upsert_valid_study_checkin(
                &mut tx,
                session.user_id,
                checkin_date,
                duration_minutes,
            )
            .await
            .map_err(UpdateStudySessionError::Database)?;
            checkins::repository::attach_session_emotion_to_checkin(
                &mut tx,
                session.user_id,
                checkin_date,
                session.id,
            )
            .await
            .map_err(UpdateStudySessionError::Database)?;
            checkins::repository::recalculate_streak_days(&mut tx, session.user_id, checkin_date)
                .await
                .map_err(UpdateStudySessionError::Database)?;
        }
    } else {
        if session.status == "resting" && next_status == "studying" {
            sqlx::query(
                r#"
                UPDATE study_breaks
                SET end_time = NOW()
                WHERE session_id = ?
                  AND end_time IS NULL
                "#,
            )
            .bind(session_id)
            .execute(&mut *tx)
            .await
            .map_err(UpdateStudySessionError::Database)?;
        }

        sqlx::query(
            r#"
            UPDATE study_sessions
            SET status = ?,
                study_content = COALESCE(?, study_content)
            WHERE id = ?
            "#,
        )
        .bind(next_status)
        .bind(study_content)
        .bind(session_id)
        .execute(&mut *tx)
        .await
        .map_err(UpdateStudySessionError::Database)?;
    }

    tx.commit()
        .await
        .map_err(UpdateStudySessionError::Database)?;

    find_detail_by_id(pool, session_id)
        .await
        .map_err(UpdateStudySessionError::Database)?
        .ok_or(UpdateStudySessionError::Database(sqlx::Error::RowNotFound))
}

pub async fn touch_heartbeat(
    pool: &MySqlPool,
    session_id: i64,
    user_id: i64,
    at: DateTime<Utc>,
) -> Result<StudySessionDetailRow, HeartbeatError> {
    let mut tx = pool.begin().await.map_err(HeartbeatError::Database)?;
    let session = find_session_for_update(&mut tx, session_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => HeartbeatError::SessionNotFound,
            err => HeartbeatError::Database(err),
        })?;

    if session.user_id != user_id {
        return Err(HeartbeatError::Forbidden);
    }

    if session.status == "ended" {
        return Err(HeartbeatError::AlreadyEnded);
    }

    sqlx::query(
        r#"
        UPDATE study_sessions
        SET last_heartbeat_at = ?
        WHERE id = ?
        "#,
    )
    .bind(at)
    .bind(session_id)
    .execute(&mut *tx)
    .await
    .map_err(HeartbeatError::Database)?;

    tx.commit().await.map_err(HeartbeatError::Database)?;

    find_detail_by_id(pool, session_id)
        .await
        .map_err(HeartbeatError::Database)?
        .ok_or(HeartbeatError::Database(sqlx::Error::RowNotFound))
}

pub async fn find_session_by_id(
    pool: &MySqlPool,
    session_id: i64,
) -> Result<Option<StudySessionRow>, sqlx::Error> {
    sqlx::query_as::<_, StudySessionRow>(
        r#"
        SELECT id, room_id, user_id, seat_id, mode, study_content, start_time,
               end_time, duration_minutes, is_valid, status, last_heartbeat_at
        FROM study_sessions
        WHERE id = ?
        "#,
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
}

async fn find_active_by_user_for_update(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
) -> Result<Option<StudySessionRow>, sqlx::Error> {
    sqlx::query_as::<_, StudySessionRow>(
        r#"
        SELECT id, room_id, user_id, seat_id, mode, study_content, start_time,
               end_time, duration_minutes, is_valid, status, last_heartbeat_at
        FROM study_sessions
        WHERE user_id = ?
          AND status IN ('studying', 'paused', 'resting')
        ORDER BY start_time DESC
        LIMIT 1
        FOR UPDATE
        "#,
    )
    .bind(user_id)
    .fetch_optional(&mut **tx)
    .await
}

async fn find_room_for_update(
    tx: &mut Transaction<'_, MySql>,
    room_id: i64,
) -> Result<RoomLockRow, sqlx::Error> {
    sqlx::query_as::<_, RoomLockRow>(
        r#"
        SELECT id, status, capacity, close_at
        FROM study_rooms
        WHERE id = ?
        FOR UPDATE
        "#,
    )
    .bind(room_id)
    .fetch_one(&mut **tx)
    .await
}

async fn find_seat_for_update(
    tx: &mut Transaction<'_, MySql>,
    seat_id: i64,
) -> Result<SeatLockRow, sqlx::Error> {
    sqlx::query_as::<_, SeatLockRow>(
        r#"
        SELECT id, room_id, status
        FROM study_room_seats
        WHERE id = ?
        FOR UPDATE
        "#,
    )
    .bind(seat_id)
    .fetch_one(&mut **tx)
    .await
}

async fn count_active_in_room(
    tx: &mut Transaction<'_, MySql>,
    room_id: i64,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM study_sessions
        WHERE room_id = ?
          AND status IN ('studying', 'paused', 'resting')
        "#,
    )
    .bind(room_id)
    .fetch_one(&mut **tx)
    .await
}

pub async fn find_session_for_update(
    tx: &mut Transaction<'_, MySql>,
    session_id: i64,
) -> Result<StudySessionRow, sqlx::Error> {
    sqlx::query_as::<_, StudySessionRow>(
        r#"
        SELECT id, room_id, user_id, seat_id, mode, study_content, start_time,
               end_time, duration_minutes, is_valid, status, last_heartbeat_at
        FROM study_sessions
        WHERE id = ?
        FOR UPDATE
        "#,
    )
    .bind(session_id)
    .fetch_one(&mut **tx)
    .await
}
