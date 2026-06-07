use axum::{Router, routing::get};

use crate::{modules::checkins::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/checkins",
            get(handler::list_checkins).post(handler::create_checkin),
        )
        .route("/checkins/{date}", get(handler::get_checkin_detail))
}
