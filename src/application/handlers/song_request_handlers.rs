use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http_api_problem::HttpApiProblem;
use reqwest::StatusCode;
use crate::domain::entities::song_request;
use crate::error;
use crate::infrastructure::state::AppState;

pub async fn get_all(Extension(state): Extension<AppState>) -> error::Result<impl IntoResponse> {
    let song_requests = state.song_request_service.get_all_song_requests().await?;
    Ok(Json(song_requests))
}

pub async fn get_by_id(Path(id): Path<i32>, Extension(state): Extension<AppState>) -> error::Result<Json<song_request::Model>> {
    match state.song_request_service.get_song_request_by_id(id).await {
        Ok(Some(song_request)) => Ok(Json(song_request)),
        Ok(None) => Err(error::Error::from(
            HttpApiProblem::new(StatusCode::NOT_FOUND).title("Song request not found"))
        ),
        Err(e) => Err(e.into()),
    }
}