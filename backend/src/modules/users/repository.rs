use sqlx::MySqlPool;

use crate::modules::users::model::UserRow;

pub async fn find_by_id(pool: &MySqlPool, user_id: i64) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, nickname, password_hash, phone, avatar_url, profile, status,
               created_at, updated_at, streak_days
        FROM users
        WHERE id = ?
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_phone(pool: &MySqlPool, phone: &str) -> Result<Option<UserRow>, sqlx::Error> {
    sqlx::query_as::<_, UserRow>(
        r#"
        SELECT id, nickname, password_hash, phone, avatar_url, profile, status,
               created_at, updated_at, streak_days
        FROM users
        WHERE phone = ?
        "#,
    )
    .bind(phone)
    .fetch_optional(pool)
    .await
}

pub async fn create_user(
    pool: &MySqlPool,
    phone: &str,
    nickname: &str,
    password_hash: &str,
) -> Result<UserRow, sqlx::Error> {
    let result = sqlx::query(
        r#"
        INSERT INTO users
            (nickname, password_hash, phone, status, created_at, updated_at, streak_days)
        VALUES
            (?, ?, ?, 'active', NOW(), NOW(), 0)
        "#,
    )
    .bind(nickname)
    .bind(password_hash)
    .bind(phone)
    .execute(pool)
    .await?;

    let user_id = result.last_insert_id() as i64;
    find_by_id(pool, user_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_profile(
    pool: &MySqlPool,
    user_id: i64,
    nickname: &str,
    profile: Option<&str>,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET nickname = ?, profile = ?, updated_at = NOW()
        WHERE id = ?
        "#,
    )
    .bind(nickname)
    .bind(profile)
    .bind(user_id)
    .execute(pool)
    .await?;

    find_by_id(pool, user_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_avatar_url(
    pool: &MySqlPool,
    user_id: i64,
    avatar_url: &str,
) -> Result<UserRow, sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET avatar_url = ?, updated_at = NOW()
        WHERE id = ?
        "#,
    )
    .bind(avatar_url)
    .bind(user_id)
    .execute(pool)
    .await?;

    find_by_id(pool, user_id)
        .await?
        .ok_or(sqlx::Error::RowNotFound)
}

pub async fn update_password_hash(
    pool: &MySqlPool,
    user_id: i64,
    password_hash: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE users
        SET password_hash = ?, updated_at = NOW()
        WHERE id = ?
        "#,
    )
    .bind(password_hash)
    .bind(user_id)
    .execute(pool)
    .await?;

    Ok(())
}
