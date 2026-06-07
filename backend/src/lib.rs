pub mod ai;
pub mod app;
pub mod auth;
pub mod cache;
pub mod config;
pub mod constants;
pub mod db;
pub mod error;
pub mod modules;
pub mod pagination;
pub mod response;
pub mod routes;
pub mod state;
pub mod storage;
pub mod time;
pub mod validation;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing() {
    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| "backend=debug,tower_http=debug,sqlx=warn".into());

    tracing_subscriber::registry()
        .with(env_filter)
        .with(tracing_subscriber::fmt::layer())
        .init();
}
