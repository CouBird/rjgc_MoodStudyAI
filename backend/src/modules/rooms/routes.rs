use axum::{Router, routing::get};

use crate::{modules::rooms::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route(
            "/rooms",
            get(handler::list_rooms).post(handler::create_room),
        )
        .route("/rooms/{room_id}", get(handler::get_room_detail))
        .route("/rooms/{room_id}/seats", get(handler::list_room_seats))
}
