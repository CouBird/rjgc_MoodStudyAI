use std::sync::Arc;

use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use crate::{config::AppConfig, error::AppError};

#[derive(Clone)]
pub struct AppState {
    pub config: Arc<AppConfig>,
    pub db: Option<MySqlPool>,
    pub redis: Option<redis::Client>,
}

impl AppState {
    pub async fn new(config: AppConfig) -> Result<Self, AppError> {
        let db = if config.database.enabled {
            let url = config.database.url.as_deref().ok_or_else(|| {
                AppError::config("DATABASE_URL is required when DATABASE_ENABLED=true")
            })?;

            Some(
                MySqlPoolOptions::new()
                    .max_connections(config.database.max_connections)
                    .connect(url)
                    .await?,
            )
        } else {
            None
        };

        let redis = if config.redis.enabled {
            let url =
                config.redis.url.as_deref().ok_or_else(|| {
                    AppError::config("REDIS_URL is required when REDIS_ENABLED=true")
                })?;

            Some(redis::Client::open(url)?)
        } else {
            None
        };

        Ok(Self {
            config: Arc::new(config),
            db,
            redis,
        })
    }

    pub async fn database_status(&self) -> &'static str {
        let Some(pool) = &self.db else {
            return "disabled";
        };

        match sqlx::query("SELECT 1").execute(pool).await {
            Ok(_) => "ok",
            Err(_) => "error",
        }
    }

    pub fn redis_status(&self) -> &'static str {
        if self.redis.is_some() {
            "configured"
        } else {
            "disabled"
        }
    }

    pub fn require_db(&self) -> Result<&MySqlPool, AppError> {
        self.db
            .as_ref()
            .ok_or_else(|| AppError::config("database is disabled"))
    }
}
