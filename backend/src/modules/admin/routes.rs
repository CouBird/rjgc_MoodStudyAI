use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::{modules::admin::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/admin/auth/login", post(handler::login))
        .route("/admin/users", get(handler::list_users))
        .route(
            "/admin/users/{user_id}/status",
            patch(handler::update_user_status),
        )
        .route("/admin/rooms", get(handler::list_rooms))
        .route(
            "/admin/rooms/{room_id}/status",
            patch(handler::update_room_status),
        )
        .route("/admin/dashboard", get(handler::dashboard))
        .route("/admin/audit-logs", get(handler::audit_logs))
}
