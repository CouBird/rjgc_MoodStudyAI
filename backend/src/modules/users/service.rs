use crate::{
    auth::{jwt::issue_token, password},
    constants::{roles, statuses},
    error::AppError,
    modules::users::{
        dto::{AuthResponse, LoginRequest, RegisterRequest, UserResponse},
        repository,
    },
    state::AppState,
    validation::{is_valid_password, is_valid_phone},
};

pub async fn register(
    state: &AppState,
    payload: RegisterRequest,
) -> Result<AuthResponse, AppError> {
    validate_register_request(&payload)?;

    let pool = state.require_db()?;

    if repository::find_by_phone(pool, &payload.phone)
        .await?
        .is_some()
    {
        return Err(AppError::Conflict("手机号已注册".to_string()));
    }

    let password_hash = password::hash_password(&payload.password)?;
    let user =
        repository::create_user(pool, &payload.phone, &payload.nickname, &password_hash).await?;
    let token = issue_token(
        user.id,
        roles::USER,
        &state.config.jwt.secret,
        state.config.jwt.user_expire_hours,
    )?;

    Ok(AuthResponse {
        user: UserResponse::from_row(user),
        token,
    })
}

pub async fn login(state: &AppState, payload: LoginRequest) -> Result<AuthResponse, AppError> {
    if !is_valid_phone(&payload.phone) || payload.password.is_empty() {
        return Err(AppError::Validation("手机号或密码格式错误".to_string()));
    }

    let pool = state.require_db()?;
    let user = repository::find_by_phone(pool, &payload.phone)
        .await?
        .ok_or(AppError::Unauthorized)?;

    if user.status != statuses::user::ACTIVE {
        return Err(AppError::Locked("账号已被禁用".to_string()));
    }

    if !password::verify_password(&payload.password, &user.password_hash)? {
        return Err(AppError::Unauthorized);
    }

    let token = issue_token(
        user.id,
        roles::USER,
        &state.config.jwt.secret,
        state.config.jwt.user_expire_hours,
    )?;

    Ok(AuthResponse {
        user: UserResponse::from_row(user),
        token,
    })
}

pub async fn get_current_user(state: &AppState, user_id: i64) -> Result<UserResponse, AppError> {
    let pool = state.require_db()?;
    let user = repository::find_by_id(pool, user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("用户不存在".to_string()))?;

    Ok(UserResponse::from_row(user))
}

fn validate_register_request(payload: &RegisterRequest) -> Result<(), AppError> {
    if !payload.agree_terms || !payload.agree_privacy {
        return Err(AppError::Validation(
            "必须同意用户协议和隐私协议".to_string(),
        ));
    }

    if !is_valid_phone(&payload.phone) {
        return Err(AppError::Validation("手机号必须为11位数字".to_string()));
    }

    if payload.nickname.trim().is_empty() || payload.nickname.chars().count() > 20 {
        return Err(AppError::Validation(
            "昵称不能为空且不能超过20个字符".to_string(),
        ));
    }

    if !is_valid_password(&payload.password) {
        return Err(AppError::Unprocessable(
            "密码需不少于8位，且包含字母和数字".to_string(),
        ));
    }

    if payload.password != payload.confirm_password {
        return Err(AppError::Unprocessable("两次密码不一致".to_string()));
    }

    Ok(())
}
