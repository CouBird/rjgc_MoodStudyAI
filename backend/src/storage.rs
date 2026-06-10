use std::path::Path;

use tokio::fs;
use uuid::Uuid;

use crate::error::AppError;

pub async fn save_avatar(
    avatar_dir: &str,
    max_avatar_bytes: usize,
    content_type: Option<&str>,
    bytes: &[u8],
) -> Result<String, AppError> {
    if bytes.is_empty() {
        return Err(AppError::Validation("头像文件不能为空".to_string()));
    }

    if bytes.len() > max_avatar_bytes {
        return Err(AppError::PayloadTooLarge("头像文件不能超过3MB".to_string()));
    }

    let extension = avatar_extension(content_type, bytes)?;
    fs::create_dir_all(avatar_dir).await?;

    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let path = Path::new(avatar_dir).join(&filename);
    fs::write(path, bytes).await?;

    Ok(format!("/storage/avatars/{filename}"))
}

fn avatar_extension(content_type: Option<&str>, bytes: &[u8]) -> Result<&'static str, AppError> {
    match content_type {
        Some("image/jpeg") | Some("image/jpg") if is_jpeg(bytes) => Ok("jpg"),
        Some("image/png") if is_png(bytes) => Ok("png"),
        _ => Err(AppError::Validation("头像仅支持 JPG 或 PNG".to_string())),
    }
}

fn is_jpeg(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0xFF, 0xD8, 0xFF])
}

fn is_png(bytes: &[u8]) -> bool {
    bytes.starts_with(&[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A])
}
