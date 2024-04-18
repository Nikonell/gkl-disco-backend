use axum::{Router, routing::get};
use crate::application::handlers::forbidden_artist_handlers::*;

pub fn forbidden_artist_routes() -> Router {
    Router::new()
        .route("/all", get(get_all))
        .route("/:name", get(find_first))
}