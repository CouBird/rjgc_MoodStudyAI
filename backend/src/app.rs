use axum::{Router, extract::DefaultBodyLimit};
use tower_http::{cors::CorsLayer, services::ServeDir, trace::TraceLayer};

use crate::{routes, state::AppState};

pub fn router(state: AppState) -> Router {
    let avatar_dir = state.config.storage.avatar_dir.clone();
    let upload_limit = state.config.storage.max_avatar_bytes + 1024 * 1024;

    Router::new()
        .route("/health", axum::routing::get(routes::health::health))
        .nest_service("/storage/avatars", ServeDir::new(avatar_dir))
        .nest("/api/v1", routes::api_v1::router())
        .layer(DefaultBodyLimit::max(upload_limit))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
