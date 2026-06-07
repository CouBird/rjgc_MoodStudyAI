use chrono::NaiveDate;
use sqlx::{MySql, MySqlPool, Transaction};

use crate::modules::checkins::model::{CheckinDetailRow, CheckinRecordRow};

pub struct CreateCheckinRecord<'a> {
    pub user_id: i64,
    pub checkin_date: NaiveDate,
    pub total_minutes: i32,
    pub is_makeup: bool,
    pub makeup_reason: Option<&'a str>,
    pub summary_note: Option<&'a str>,
}

pub async fn list_by_month(
    pool: &MySqlPool,
    user_id: i64,
    month_start: NaiveDate,
    next_month_start: NaiveDate,
) -> Result<Vec<CheckinRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, CheckinRecordRow>(
        r#"
        SELECT id, user_id, checkin_date, emotion_record_id, total_minutes,
               is_makeup, makeup_reason, summary_note, created_at
        FROM checkin_records
        WHERE user_id = ?
          AND checkin_date >= ?
          AND checkin_date < ?
        ORDER BY checkin_date ASC
        "#,
    )
    .bind(user_id)
    .bind(month_start)
    .bind(next_month_start)
    .fetch_all(pool)
    .await
}

pub async fn find_by_date(
    pool: &MySqlPool,
    user_id: i64,
    date: NaiveDate,
) -> Result<Option<CheckinRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, CheckinRecordRow>(
        r#"
        SELECT id, user_id, checkin_date, emotion_record_id, total_minutes,
               is_makeup, makeup_reason, summary_note, created_at
        FROM checkin_records
        WHERE user_id = ?
          AND checkin_date = ?
        "#,
    )
    .bind(user_id)
    .bind(date)
    .fetch_optional(pool)
    .await
}

pub async fn find_detail_by_date(
    pool: &MySqlPool,
    user_id: i64,
    date: NaiveDate,
) -> Result<Option<CheckinDetailRow>, sqlx::Error> {
    sqlx::query_as::<_, CheckinDetailRow>(
        r#"
        SELECT cr.id, cr.user_id, cr.checkin_date, cr.emotion_record_id, cr.total_minutes,
               cr.is_makeup, cr.makeup_reason, cr.summary_note, cr.created_at,
               er.emotion_tag, er.emotion_score, er.user_note, er.ai_feedback,
               er.created_at AS emotion_created_at
        FROM checkin_records cr
        LEFT JOIN emotion_records er ON er.id = cr.emotion_record_id
        WHERE cr.user_id = ?
          AND cr.checkin_date = ?
        "#,
    )
    .bind(user_id)
    .bind(date)
    .fetch_optional(pool)
    .await
}

pub async fn create_checkin_transaction(
    pool: &MySqlPool,
    input: CreateCheckinRecord<'_>,
    streak_base_date: NaiveDate,
) -> Result<CheckinRecordRow, sqlx::Error> {
    let mut tx = pool.begin().await?;

    let result = sqlx::query(
        r#"
        INSERT INTO checkin_records
            (user_id, checkin_date, total_minutes, is_makeup, makeup_reason, summary_note, created_at)
        VALUES
            (?, ?, ?, ?, ?, ?, NOW())
        "#,
    )
    .bind(input.user_id)
    .bind(input.checkin_date)
    .bind(input.total_minutes)
    .bind(if input.is_makeup { 1 } else { 0 })
    .bind(input.makeup_reason)
    .bind(input.summary_note)
    .execute(&mut *tx)
    .await?;

    recalculate_streak_days(&mut tx, input.user_id, streak_base_date).await?;
    tx.commit().await?;

    find_by_id(pool, result.last_insert_id() as i64)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn attach_emotion_to_checkin(
    pool: &MySqlPool,
    user_id: i64,
    checkin_date: NaiveDate,
    emotion_record_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE checkin_records
        SET emotion_record_id = ?
        WHERE user_id = ?
          AND checkin_date = ?
          AND emotion_record_id IS NULL
        "#,
    )
    .bind(emotion_record_id)
    .bind(user_id)
    .bind(checkin_date)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn attach_session_emotion_to_checkin(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
    checkin_date: NaiveDate,
    session_id: i64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE checkin_records cr
        SET cr.emotion_record_id = (
            SELECT er.id
            FROM emotion_records er
            WHERE er.session_id = ?
            ORDER BY er.created_at DESC
            LIMIT 1
        )
        WHERE cr.user_id = ?
          AND cr.checkin_date = ?
          AND cr.emotion_record_id IS NULL
          AND EXISTS (
              SELECT 1
              FROM emotion_records er
              WHERE er.session_id = ?
          )
        "#,
    )
    .bind(session_id)
    .bind(user_id)
    .bind(checkin_date)
    .bind(session_id)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn upsert_valid_study_checkin(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
    checkin_date: NaiveDate,
    minutes: i32,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO checkin_records
            (user_id, checkin_date, total_minutes, is_makeup, created_at)
        VALUES
            (?, ?, ?, 0, NOW())
        ON DUPLICATE KEY UPDATE
            total_minutes = total_minutes + VALUES(total_minutes)
        "#,
    )
    .bind(user_id)
    .bind(checkin_date)
    .bind(minutes)
    .execute(&mut **tx)
    .await?;

    Ok(())
}

pub async fn recalculate_streak_days(
    tx: &mut Transaction<'_, MySql>,
    user_id: i64,
    today: NaiveDate,
) -> Result<i32, sqlx::Error> {
    let dates = sqlx::query_scalar::<_, NaiveDate>(
        r#"
        SELECT checkin_date
        FROM checkin_records
        WHERE user_id = ?
          AND checkin_date <= ?
        ORDER BY checkin_date DESC
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_all(&mut **tx)
    .await?;

    let mut expected = today;
    let mut streak = 0;
    for date in dates {
        if date == expected {
            streak += 1;
            expected = expected.pred_opt().ok_or(sqlx::Error::RowNotFound)?;
        } else if date < expected {
            break;
        }
    }

    sqlx::query(
        r#"
        UPDATE users
        SET streak_days = ?, updated_at = NOW()
        WHERE id = ?
        "#,
    )
    .bind(streak)
    .bind(user_id)
    .execute(&mut **tx)
    .await?;

    Ok(streak)
}

async fn find_by_id(
    pool: &MySqlPool,
    checkin_id: i64,
) -> Result<Option<CheckinRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, CheckinRecordRow>(
        r#"
        SELECT id, user_id, checkin_date, emotion_record_id, total_minutes,
               is_makeup, makeup_reason, summary_note, created_at
        FROM checkin_records
        WHERE id = ?
        "#,
    )
    .bind(checkin_id)
    .fetch_optional(pool)
    .await
}
