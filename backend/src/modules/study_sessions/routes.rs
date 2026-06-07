use axum::{Router, routing::get};

use crate::{modules::study_sessions::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/study-sessions",
            get(handler::active_session).post(handler::start_session),
        )
        .route("/study-sessions/active", get(handler::active_session))
        .route(
            "/study-sessions/{session_id}",
            axum::routing::patch(handler::update_session),
        )
        .route(
            "/study-sessions/{session_id}/heartbeats",
            axum::routing::post(handler::heartbeat),
        )
}
