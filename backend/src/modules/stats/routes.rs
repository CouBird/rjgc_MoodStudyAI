use axum::{Router, routing::get};

use crate::{modules::stats::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/users/me/stats/today", get(handler::today_stats))
        .route("/users/me/stats", get(handler::user_stats))
}
