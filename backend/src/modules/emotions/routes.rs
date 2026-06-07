use axum::{Router, routing::get};

use crate::{modules::emotions::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/study-sessions/{session_id}/emotion-records",
            axum::routing::post(handler::create_emotion_record),
        )
        .route("/users/me/emotion-trends", get(handler::emotion_trends))
}
