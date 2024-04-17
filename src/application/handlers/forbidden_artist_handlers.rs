use axum::{Extension, Json};
use crate::domain::entities::forbidden_artist;
use crate::infrastructure::state::AppState;
use crate::error;

pub async fn get_all(Extension(state): Extension<AppState>) -> error::Result<Json<Vec<forbidden_artist::Model>>> {
    let forbidden_artists = state.forbidden_artist_service.get_all_forbidden_artists().await?;
    Ok(Json(forbidden_artists))
}   