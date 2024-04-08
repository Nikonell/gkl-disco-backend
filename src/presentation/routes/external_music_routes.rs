use axum::{Router, routing::get};
use crate::application::handlers::external_track_handlers::search_track_handler;

pub fn external_music_routes() -> Router {
    Router::new()
        .route("/search_tracks", get(search_track_handler))
}