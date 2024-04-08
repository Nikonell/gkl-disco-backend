use serde::Serialize;

#[derive(Serialize)]
pub struct FoundTrack {
    pub id: i64,
    pub title: String,
    pub artist_names: Vec<String>,
    pub cover_url: String,
    pub explicit: bool
}