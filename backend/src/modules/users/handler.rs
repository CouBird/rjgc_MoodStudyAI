use axum::{
    Json,
    extract::{Multipart, State},
};

use crate::{
    auth::extractor::CurrentUser,
    error::AppError,
    modules::users::{
        dto::{
            AuthResponse, AvatarResponse, ChangePasswordRequest, LoginRequest, RegisterRequest,
            UpdateProfileRequest, UserResponse,
        },
        service,
    },
    response::ApiResponse,
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let response = service::register(&state, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiResponse<AuthResponse>>, AppError> {
    let response = service::login(&state, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn me(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let response = service::get_current_user(&state, current_user.user_id).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn update_me(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ApiResponse<UserResponse>>, AppError> {
    let response = service::update_current_user(&state, current_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(response)))
}

pub async fn upload_avatar(
    State(state): State<AppState>,
    current_user: CurrentUser,
    mut multipart: Multipart,
) -> Result<Json<ApiResponse<AvatarResponse>>, AppError> {
    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|_| AppError::Validation("头像上传格式错误".to_string()))?
    {
        if field.name() != Some("file") {
            continue;
        }

        let content_type = field.content_type().map(str::to_string);
        let bytes = field
            .bytes()
            .await
            .map_err(|_| AppError::Validation("头像上传格式错误".to_string()))?;
        let response = service::upload_avatar(
            &state,
            current_user.user_id,
            content_type.as_deref(),
            &bytes,
        )
        .await?;

        return Ok(Json(ApiResponse::ok(response)));
    }

    Err(AppError::Validation("请使用 file 字段上传头像".to_string()))
}

pub async fn change_password(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<ApiResponse<()>>, AppError> {
    service::change_password(&state, current_user.user_id, payload).await?;
    Ok(Json(ApiResponse::ok(())))
}
