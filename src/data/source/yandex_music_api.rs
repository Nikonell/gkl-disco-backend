use reqwest;
use serde_json::Value;
use crate::domain::models::found_track::FoundTrack;
use std::env;

const YANDEX_API_BASE_URL: &str = "https://api.music.yandex.net:443";

pub async fn search_track(text: &String, limit: u16) -> anyhow::Result<Vec<FoundTrack>> {
    let yandex_token = env::var("YANDEX_MUSIC_TOKEN").expect("YANDEX_MUSIC_TOKEN not found in scope");
    let query = &[("page", "0"), ("type", "track"), ("text", text.as_str())];

    let client = reqwest::Client::new();
    let response = client.get(YANDEX_API_BASE_URL.to_owned() + "/search")
        .query(query)
        .bearer_auth(yandex_token)
        .send()
        .await?;
    let resp_text = response.text().await?;

    let resp_data: Value = serde_json::from_str(resp_text.as_str())?;
    let tracks_list = resp_data.get("result")
        .and_then(|r| r.get("tracks"))
        .and_then(|r| r.get("results"))
        .and_then(|r| r.as_array())
        .unwrap_or(&Vec::new()).to_owned();

    let mut mapped_tracks: Vec<FoundTrack> = tracks_list.iter()
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

    // Apply the limit to the number of tracks returned
    mapped_tracks.truncate(limit as usize);

    Ok(mapped_tracks)
}