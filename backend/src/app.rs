use axum::Router;
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{routes, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", axum::routing::get(routes::health::health))
        .nest("/api/v1", routes::api_v1::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
