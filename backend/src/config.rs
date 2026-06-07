use std::{env, net::SocketAddr};

use crate::error::AppError;

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub jwt: JwtConfig,
    pub storage: StorageConfig,
}

#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub enabled: bool,
    pub url: Option<String>,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct RedisConfig {
    pub enabled: bool,
    pub url: Option<String>,
}

#[derive(Debug, Clone)]
pub struct JwtConfig {
    pub secret: String,
    pub user_expire_hours: i64,
    pub admin_expire_hours: i64,
}

#[derive(Debug, Clone)]
pub struct StorageConfig {
    pub avatar_dir: String,
    pub max_avatar_bytes: usize,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        dotenvy::dotenv().ok();

        Ok(Self {
            server: ServerConfig {
                host: env_or("APP_HOST", "127.0.0.1"),
                port: parse_env("APP_PORT", 8080)?,
            },
            database: DatabaseConfig {
                enabled: parse_env("DATABASE_ENABLED", false)?,
                url: env::var("DATABASE_URL").ok(),
                max_connections: parse_env("DATABASE_MAX_CONNECTIONS", 5)?,
            },
            redis: RedisConfig {
                enabled: parse_env("REDIS_ENABLED", false)?,
                url: env::var("REDIS_URL").ok(),
            },
            jwt: JwtConfig {
                secret: env_or("JWT_SECRET", "dev-only-change-me"),
                user_expire_hours: parse_env("JWT_USER_EXPIRE_HOURS", 24)?,
                admin_expire_hours: parse_env("JWT_ADMIN_EXPIRE_HOURS", 8)?,
            },
            storage: StorageConfig {
                avatar_dir: env_or("AVATAR_DIR", "storage/avatars"),
                max_avatar_bytes: parse_env("MAX_AVATAR_BYTES", 3 * 1024 * 1024)?,
            },
        })
    }
}

impl ServerConfig {
    pub fn address(&self) -> Result<SocketAddr, AppError> {
        format!("{}:{}", self.host, self.port)
            .parse()
            .map_err(|source| AppError::config(format!("invalid server address: {source}")))
    }
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn parse_env<T>(key: &str, default: T) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env::var(key) {
        Ok(value) => value
            .parse::<T>()
            .map_err(|source| AppError::config(format!("invalid {key}: {source}"))),
        Err(_) => Ok(default),
    }
}
