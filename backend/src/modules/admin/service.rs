use crate::{
    auth::{jwt::issue_token, password},
    constants::roles,
    error::AppError,
    modules::admin::{
        audit,
        dto::{AdminAuthResponse, AdminLoginRequest, AdminResponse},
        repository,
    },
    state::AppState,
};

pub async fn login(
    state: &AppState,
    payload: AdminLoginRequest,
) -> Result<AdminAuthResponse, AppError> {
    if payload.account.trim().is_empty() || payload.password.is_empty() {
        return Err(AppError::Validation("账号和密码不能为空".to_string()));
    }

    let pool = state.require_db()?;
    let admin = repository::find_by_account(pool, &payload.account)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if !password::verify_password(&payload.password, &admin.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    repository::insert_audit_log(
        pool,
        admin.id,
        audit::ACTION_ADMIN_LOGIN,
        "admin",
        admin.id,
        None,
    )
    .await?;

    let token = issue_token(
        admin.id,
        roles::ADMIN,
        &state.config.jwt.secret,
        state.config.jwt.admin_expire_hours,
    )?;

    Ok(AdminAuthResponse {
        admin: AdminResponse::from_row(admin),
        admin_token: token,
    })
}
