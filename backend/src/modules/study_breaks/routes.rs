use axum::{Router, routing::patch};

use crate::{modules::study_breaks::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/study-sessions/{session_id}/breaks",
            axum::routing::post(handler::create_break),
        )
        .route("/study-breaks/{break_id}", patch(handler::extend_break))
}
