use crate::error::AppError;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
        .map_err(|source| AppError::Internal(format!("failed to hash password: {source}")))
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    bcrypt::verify(password, hash)
        .map_err(|source| AppError::Internal(format!("failed to verify password: {source}")))
}
