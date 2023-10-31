use std::sync::Arc;

use axum::{routing::get, Router};

use crate::{
    handler::{get_all_chats_handler, health_checker_handler},
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/api/healthchecker", get(health_checker_handler))
        .route("/api/chats", get(get_all_chats_handler))
        .with_state(app_state)
}
