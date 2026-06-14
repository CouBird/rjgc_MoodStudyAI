use axum::{
    Router,
    routing::{get, patch, post},
};

use crate::{modules::users::handler, state::AppState};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/register", post(handler::register))
        .route("/auth/login", post(handler::login))
        .route("/users/me", get(handler::me).patch(handler::update_me))
        .route("/users/me/avatar", post(handler::upload_avatar))
        .route("/users/me/password", patch(handler::change_password))
}
