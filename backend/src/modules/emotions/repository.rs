use chrono::{DateTime, Utc};
use sqlx::MySqlPool;

use crate::modules::{
    emotions::model::{
        EmotionRecordRow, EmotionTagCountRow, EmotionTrendBucketRow, EmotionTrendRow,
    },
    study_sessions::model::StudySessionRow,
};

pub struct CreateEmotionRecord<'a> {
    pub session_id: i64,
    pub emotion_tag: &'a str,
    pub emotion_score: i32,
    pub user_note: Option<&'a str>,
    pub ai_feedback: &'a str,
}

pub async fn find_session(
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

pub async fn find_by_session(
    pool: &MySqlPool,
    session_id: i64,
) -> Result<Option<EmotionRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, EmotionRecordRow>(
        r#"
        SELECT id, session_id, emotion_tag, emotion_score, user_note,
               ai_feedback, created_at
        FROM emotion_records
        WHERE session_id = ?
        "#,
    )
    .bind(session_id)
    .fetch_optional(pool)
    .await
}

pub async fn create_record(
    pool: &MySqlPool,
    input: CreateEmotionRecord<'_>,
) -> Result<EmotionRecordRow, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO emotion_records
            (session_id, emotion_tag, emotion_score, user_note, ai_feedback, created_at)
        VALUES
            (?, ?, ?, ?, ?, NOW())
        "#,
    )
    .bind(input.session_id)
    .bind(input.emotion_tag)
    .bind(input.emotion_score)
    .bind(input.user_note)
    .bind(input.ai_feedback)
    .execute(pool)
    .await?;

    find_by_id(pool, result.last_insert_id() as i64)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn list_trends(
    pool: &MySqlPool,
    user_id: i64,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> Result<Vec<EmotionTrendRow>, sqlx::Error> {
    sqlx::query_as::<_, EmotionTrendRow>(
        r#"
        SELECT
            DATE(er.created_at) AS date,
            CAST(AVG(er.emotion_score) AS DOUBLE) AS average_score,
            SUBSTRING_INDEX(GROUP_CONCAT(er.emotion_tag ORDER BY er.created_at DESC), ',', 1)
                AS dominant_emotion,
            COUNT(*) AS count
        FROM emotion_records er
        INNER JOIN study_sessions ss ON ss.id = er.session_id
        WHERE ss.user_id = ?
          AND er.created_at >= ?
          AND er.created_at < ?
        GROUP BY DATE(er.created_at)
        ORDER BY date ASC
        "#,
    )
    .bind(user_id)
    .bind(start_at)
    .bind(end_at)
    .fetch_all(pool)
    .await
}

pub async fn tag_distribution(
    pool: &MySqlPool,
    user_id: i64,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> Result<Vec<EmotionTagCountRow>, sqlx::Error> {
    sqlx::query_as::<_, EmotionTagCountRow>(
        r#"
        SELECT er.emotion_tag, COUNT(*) AS count
        FROM emotion_records er
        INNER JOIN study_sessions ss ON ss.id = er.session_id
        WHERE ss.user_id = ?
          AND er.created_at >= ?
          AND er.created_at < ?
        GROUP BY er.emotion_tag
        ORDER BY count DESC, er.emotion_tag ASC
        "#,
    )
    .bind(user_id)
    .bind(start_at)
    .bind(end_at)
    .fetch_all(pool)
    .await
}

pub async fn trend_buckets(
    pool: &MySqlPool,
    user_id: i64,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> Result<Vec<EmotionTrendBucketRow>, sqlx::Error> {
    sqlx::query_as::<_, EmotionTrendBucketRow>(
        r#"
        SELECT DATE(er.created_at) AS date,
               er.emotion_tag,
               COUNT(*) AS count,
               MAX(er.created_at) AS last_created_at
        FROM emotion_records er
        INNER JOIN study_sessions ss ON ss.id = er.session_id
        WHERE ss.user_id = ?
          AND er.created_at >= ?
          AND er.created_at < ?
        GROUP BY DATE(er.created_at), er.emotion_tag
        ORDER BY date ASC, count DESC, last_created_at DESC, er.emotion_tag ASC
        "#,
    )
    .bind(user_id)
    .bind(start_at)
    .bind(end_at)
    .fetch_all(pool)
    .await
}

async fn find_by_id(
    pool: &MySqlPool,
    emotion_record_id: i64,
) -> Result<Option<EmotionRecordRow>, sqlx::Error> {
    sqlx::query_as::<_, EmotionRecordRow>(
        r#"
        SELECT id, session_id, emotion_tag, emotion_score, user_note,
               ai_feedback, created_at
        FROM emotion_records
        WHERE id = ?
        "#,
    )
    .bind(emotion_record_id)
    .fetch_optional(pool)
    .await
}
