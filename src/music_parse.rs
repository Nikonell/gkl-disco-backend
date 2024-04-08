use axum::extract::Query;
use axum::{Json, Router};
use axum::response::IntoResponse;
use axum::routing::get;
use serde::Deserialize;
use crate::error;
use crate::models::FoundTrack;

const YANDEX_API_BASE_URL: &'static str = "https://api.music.yandex.net:443";

pub fn music_parse_routes() -> Router {
    Router::new()
        .route("/search_tracks", get(search_track_handler))
}

#[derive(Deserialize)]
pub struct SearchTrackFilter {
    text: String,
    limit: Option<i32>
}

pub async fn search_track_handler(Query(filter): Query<SearchTrackFilter>) -> error::Result<impl IntoResponse> {
    let result = search_track(&filter.text, 10).await?;
    Ok(Json(result))
}

pub async fn search_track(text: &String, limit: i32) -> anyhow::Result<Vec<FoundTrack>> {
    let yandex_token = std::env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN not found in scope");
    let query = &[("page", "0"), ("type", "track"), ("text", text.as_str())];

    let client = reqwest::Client::new();
    let response = client.get(YANDEX_API_BASE_URL.to_owned() + "/search")
        .query(query)
        .bearer_auth(yandex_token)
        .send()
        .await?;
    let resp_text = response.text().await?;

    let resp_data: serde_json::Value = serde_json::from_str(resp_text.as_str())?;
    let tracks_list = resp_data.get("result")
        .and_then(|r| r.get("tracks"))
        .and_then(|r| r.get("results"))
        .and_then(|r| r.as_array())
        .unwrap_or(&Vec::new()).to_owned();

    let mapped_tracks: Vec<FoundTrack> = tracks_list.iter()
        .scan((), |_, track_val| Some(FoundTrack {
            id: track_val.get("id").and_then(|v| v.as_i64())?,
            title: track_val.get("title").and_then(|v| v.as_str())?.to_string(),
            artist_names: track_val.get("artists").and_then(|v| v.as_array())?.iter()
                .scan((), |_, artist_val| Some(artist_val.get("name")
                    .and_then(|v| v.as_str())?.to_string()))
                .collect(),
            cover_url: track_val.get("coverUri")?.as_str()?.to_string(),
            explicit: track_val.get("explicit")?.as_bool()?
        }))
        .collect();

    Ok(mapped_tracks)
}
