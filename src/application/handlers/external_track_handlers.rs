use axum::{Json, extract::Query, debug_handler};
use crate::data::source::yandex_music_api::search_track;
use crate::error::Result;
use serde::Deserialize;
use crate::domain::models::found_track::FoundTrack;

#[derive(Deserialize)]
pub struct SearchTrackFilter {
    text: String,
    limit: Option<u16>,
}

#[debug_handler]
pub async fn search_track_handler(Query(filter): Query<SearchTrackFilter>) -> Result<Json<Vec<FoundTrack>>> {
    let result = search_track(&filter.text, filter.limit.unwrap_or(10)).await?;
    Ok(Json(result))
}