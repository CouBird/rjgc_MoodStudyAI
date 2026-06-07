use axum::{Router, routing::get};

use crate::{modules, routes::health, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health::health))
        .merge(modules::users::routes::router())
        .merge(modules::rooms::routes::router())
        .merge(modules::study_sessions::routes::router())
        .merge(modules::study_breaks::routes::router())
        .merge(modules::checkins::routes::router())
        .merge(modules::emotions::routes::router())
        .merge(modules::stats::routes::router())
        .merge(modules::admin::routes::router())
}
