use axum::{
    Router,
    routing::{get, post},
};

use crate::{modules::users::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(handler::register))
        .route("/auth/login", post(handler::login))
        .route("/users/me", get(handler::me))
}
