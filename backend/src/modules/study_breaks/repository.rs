use sqlx::{MySql, MySqlPool, Transaction};

use crate::modules::{
    study_breaks::model::StudyBreakRow, study_sessions::repository as session_repository,
};

#[derive(Debug)]
pub enum CreateBreakError {
    SessionNotFound,
    Forbidden,
    AlreadyEnded,
    NotStudying,
    Database(sqlx::Error),
}

#[derive(Debug)]
pub enum ExtendBreakError {
    BreakNotFound,
    Forbidden,
    AlreadyEnded,
    Database(sqlx::Error),
}

pub async fn create_break_transaction(
    pool: &MySqlPool,
    session_id: i64,
    user_id: i64,
    duration_minutes: i32,
) -> Result<StudyBreakRow, CreateBreakError> {
    let mut tx = pool.begin().await.map_err(CreateBreakError::Database)?;
    let session = session_repository::find_session_for_update(&mut tx, session_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => CreateBreakError::SessionNotFound,
            err => CreateBreakError::Database(err),
        })?;

    if session.user_id != user_id {
        return Err(CreateBreakError::Forbidden);
    }

    if session.status == "ended" {
        return Err(CreateBreakError::AlreadyEnded);
    }

    if session.status != "studying" {
        return Err(CreateBreakError::NotStudying);
    }

    sqlx::query(
        r#"
        UPDATE study_sessions
        SET status = 'resting'
        WHERE id = ?
        "#,
    )
    .bind(session_id)
    .execute(&mut *tx)
    .await
    .map_err(CreateBreakError::Database)?;

    let result = sqlx::query(
        r#"
        INSERT INTO study_breaks (session_id, start_time, duration, is_extended)
        VALUES (?, NOW(), ?, 0)
        "#,
    )
    .bind(session_id)
    .bind(duration_minutes)
    .execute(&mut *tx)
    .await
    .map_err(CreateBreakError::Database)?;

    let break_id = result.last_insert_id() as i64;
    let row = find_break_for_update(&mut tx, break_id)
        .await
        .map_err(CreateBreakError::Database)?;
    tx.commit().await.map_err(CreateBreakError::Database)?;

    Ok(row)
}

pub async fn extend_break_transaction(
    pool: &MySqlPool,
    break_id: i64,
    user_id: i64,
    extend_minutes: i32,
) -> Result<StudyBreakRow, ExtendBreakError> {
    let mut tx = pool.begin().await.map_err(ExtendBreakError::Database)?;
    let row = find_break_for_update(&mut tx, break_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ExtendBreakError::BreakNotFound,
            err => ExtendBreakError::Database(err),
        })?;

    let session = session_repository::find_session_for_update(&mut tx, row.session_id)
        .await
        .map_err(|err| match err {
            sqlx::Error::RowNotFound => ExtendBreakError::Database(sqlx::Error::RowNotFound),
            err => ExtendBreakError::Database(err),
        })?;

    if session.user_id != user_id {
        return Err(ExtendBreakError::Forbidden);
    }

    if row.end_time.is_some() {
        return Err(ExtendBreakError::AlreadyEnded);
    }

    sqlx::query(
        r#"
        UPDATE study_breaks
        SET duration = duration + ?,
            is_extended = 1
        WHERE id = ?
        "#,
    )
    .bind(extend_minutes)
    .bind(break_id)
    .execute(&mut *tx)
    .await
    .map_err(ExtendBreakError::Database)?;

    let updated = find_break_for_update(&mut tx, break_id)
        .await
        .map_err(ExtendBreakError::Database)?;
    tx.commit().await.map_err(ExtendBreakError::Database)?;

    Ok(updated)
}

async fn find_break_for_update(
    tx: &mut Transaction<'_, MySql>,
    break_id: i64,
) -> Result<StudyBreakRow, sqlx::Error> {
    sqlx::query_as::<_, StudyBreakRow>(
        r#"
        SELECT id, session_id, start_time, end_time, duration, is_extended
        FROM study_breaks
        WHERE id = ?
        FOR UPDATE
        "#,
    )
    .bind(break_id)
    .fetch_one(&mut **tx)
    .await
}
