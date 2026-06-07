use sqlx::MySqlPool;

use crate::modules::admin::model::AdminUserRow;

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
