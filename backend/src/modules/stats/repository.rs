use chrono::{DateTime, NaiveDate, Utc};
use sqlx::MySqlPool;

#[derive(Debug, sqlx::FromRow)]
pub struct StudyTrendRow {
    pub date: NaiveDate,
    pub total_minutes: i64,
}

#[derive(Debug, sqlx::FromRow)]
pub struct EmotionTrendBucketRow {
    pub date: NaiveDate,
    pub emotion_tag: String,
    pub count: i64,
    pub last_created_at: DateTime<Utc>,
}

pub async fn today_study_minutes(
    pool: &MySqlPool,
    user_id: i64,
    today: NaiveDate,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, Option<i64>>(
        r#"
        SELECT CAST(COALESCE(SUM(total_minutes), 0) AS SIGNED)
        FROM checkin_records
        WHERE user_id = ?
          AND checkin_date = ?
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_one(pool)
    .await
    .map(|value| value.unwrap_or_default())
}

pub async fn streak_days_as_of(
    pool: &MySqlPool,
    user_id: i64,
    base_date: NaiveDate,
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
    .bind(base_date)
    .fetch_all(pool)
    .await?;

    Ok(calculate_streak_days(&dates, base_date))
}

fn calculate_streak_days(dates: &[NaiveDate], base_date: NaiveDate) -> i32 {
    let Some(latest_date) = dates.first().copied() else {
        return 0;
    };

    let yesterday = base_date.pred_opt();
    let Some(mut expected) = (if latest_date == base_date {
        Some(base_date)
    } else {
        yesterday.filter(|date| latest_date == *date)
    }) else {
        return 0;
    };

    let mut streak = 0;
    for &date in dates {
        if date == expected {
            streak += 1;
            expected = match expected.pred_opt() {
                Some(value) => value,
                None => break,
            };
        } else if date < expected {
            break;
        }
    }

    streak
}

pub async fn today_checkin_exists(
    pool: &MySqlPool,
    user_id: i64,
    today: NaiveDate,
) -> Result<bool, sqlx::Error> {
    let count = sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM checkin_records
        WHERE user_id = ?
          AND checkin_date = ?
        "#,
    )
    .bind(user_id)
    .bind(today)
    .fetch_one(pool)
    .await?;

    Ok(count > 0)
}

pub async fn latest_emotion_tag(
    pool: &MySqlPool,
    user_id: i64,
) -> Result<Option<String>, sqlx::Error> {
    sqlx::query_scalar::<_, String>(
        r#"
        SELECT er.emotion_tag
        FROM emotion_records er
        INNER JOIN study_sessions ss ON ss.id = er.session_id
        WHERE ss.user_id = ?
        ORDER BY er.created_at DESC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn valid_session_count(
    pool: &MySqlPool,
    user_id: i64,
    start_at: DateTime<Utc>,
    end_at: DateTime<Utc>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM study_sessions
        WHERE user_id = ?
          AND is_valid = 1
          AND status = 'ended'
          AND end_time >= ?
          AND end_time < ?
        "#,
    )
    .bind(user_id)
    .bind(start_at)
    .bind(end_at)
    .fetch_one(pool)
    .await
}

pub async fn study_trends(
    pool: &MySqlPool,
    user_id: i64,
    start_date: NaiveDate,
    end_date: NaiveDate,
) -> Result<Vec<StudyTrendRow>, sqlx::Error> {
    sqlx::query_as::<_, StudyTrendRow>(
        r#"
        SELECT checkin_date AS date,
               CAST(COALESCE(SUM(total_minutes), 0) AS SIGNED) AS total_minutes
        FROM checkin_records
        WHERE user_id = ?
          AND checkin_date >= ?
          AND checkin_date < ?
        GROUP BY checkin_date
        ORDER BY date ASC
        "#,
    )
    .bind(user_id)
    .bind(start_date)
    .bind(end_date)
    .fetch_all(pool)
    .await
}

pub async fn emotion_trend_buckets(
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
