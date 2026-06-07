use axum::{Router, routing::post};

use crate::{modules::admin::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new().route("/admin/auth/login", post(handler::login))
}
