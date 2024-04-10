use axum::{Router, routing::get};
use crate::application::handlers::song_request_handlers::*;

pub fn song_request_routes() -> Router {
    Router::new()
        .route("/all", get(get_all))
        .route("/:id", get(get_by_id))
}