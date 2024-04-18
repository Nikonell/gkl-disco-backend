use axum::extract::Path;
use axum::{Extension, Json};
use http_api_problem::HttpApiProblem;
use reqwest::StatusCode;
use crate::domain::entities::forbidden_artist;
use crate::infrastructure::state::AppState;
use crate::error;

pub async fn get_all(Extension(state): Extension<AppState>) -> error::Result<Json<Vec<forbidden_artist::Model>>> {
    let forbidden_artists = state.forbidden_artist_service.get_all_forbidden_artists().await?;
    Ok(Json(forbidden_artists))
}

pub async fn find_first(Path(name): Path<String>, Extension(state): Extension<AppState>) -> error::Result<Json<forbidden_artist::Model>> {
    match state.forbidden_artist_service.get_first_forbidden_artist(&name).await {
        Ok(Some(forbidden_artist)) => Ok(Json(forbidden_artist)),
        Ok(None) => Err(error::Error::from(
            HttpApiProblem::new(StatusCode::NOT_FOUND).title("Forbidden artist not found"))
        ),
        Err(e) => Err(e.into()),
    }
}