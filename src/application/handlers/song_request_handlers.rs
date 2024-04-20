use axum::http::HeaderMap;
use axum::{Extension, Json};
use axum::extract::Path;
use axum::response::IntoResponse;
use http_api_problem::HttpApiProblem;
use reqwest::StatusCode;
use sea_orm::ActiveValue::Set;
use serde::Deserialize;
use crate::data::source::yandex_music_api::get_track_by_id;
use crate::domain::entities::song_request;
use crate::error;
use crate::infrastructure::state::AppState;

pub async fn get_all(headers: HeaderMap, Extension(state): Extension<AppState>) -> error::Result<Json<Vec<song_request::Model>>> {
    if let Err(unauthorized) = state.access_token_service.authorize(&headers).await { return Err(unauthorized); }
    let song_requests = state.song_request_service.get_all_song_requests().await?;
    Ok(Json(song_requests))
}

pub async fn get_by_id(headers: HeaderMap, Path(id): Path<i32>, Extension(state): Extension<AppState>) -> error::Result<Json<song_request::Model>> {
    if let Err(unauthorized) = state.access_token_service.authorize(&headers).await { return Err(unauthorized); }
    match state.song_request_service.get_song_request_by_id(id).await {
        Ok(Some(song_request)) => Ok(Json(song_request)),
        Ok(None) => Err(error::Error::from(
            HttpApiProblem::new(StatusCode::NOT_FOUND).title("Song request not found"))
        ),
        Err(e) => Err(e.into()),
    }
}

#[derive(Deserialize)]
pub struct CreateSongRequest {
    yandex_id: i32,
    say_hello: bool,
    hello_from: Option<String>,
    hello_to: Option<String>,
    hello_text: Option<String>,
}

pub async fn create(Extension(state): Extension<AppState>, Json(model): Json<CreateSongRequest>) -> error::Result<Json<song_request::Model>> {
    match state.song_request_service.get_song_request_by_song_id(model.yandex_id).await {
        Ok(Some(_)) => return Err(error::Error::from(HttpApiProblem::new(StatusCode::CONFLICT).title("This song already requested"))),
        Err(err) => return Err(error::Error::from(err)),
        _ => ()
    }
    let track = get_track_by_id(model.yandex_id as i64).await?;
    let mut artist_forbidden = false;
    for artist in &track.artist_names {
        match state.forbidden_artist_service.get_first_forbidden_artist(artist).await {
            Ok(Some(_)) => artist_forbidden = true,
            Err(err) => return Err(error::Error::from(err)),
            _ => ()
        }
    }
    let song_request = song_request::ActiveModel {
        yandex_id: Set(model.yandex_id),
        song_name: Set(track.title),
        artist_names: Set(track.artist_names.join(", ")),
        say_hello: Set(model.say_hello),
        hello_from: Set(model.hello_from),
        hello_to: Set(model.hello_to),
        hello_text: Set(model.hello_text),
        explicit_correct: Set(!track.explicit),
        artist_correct: Set(!artist_forbidden),
        ..Default::default()
    };
    Ok(Json(state.song_request_service.create_song_request(song_request).await?))
}

#[derive(Deserialize)]
pub struct UpdateSongRequest {
    yandex_id: Option<i32>,
    song_name: Option<String>,
    artist_names: Option<String>,
    say_hello: Option<bool>,
    hello_from: Option<String>,
    hello_to: Option<String>,
    hello_text: Option<String>,
    explicit_correct: Option<bool>,
    artist_correct: Option<bool>,
    expert_mark: Option<bool>
}

pub async fn update(headers: HeaderMap, Path(id): Path<i32>, Extension(state): Extension<AppState>, Json(model): Json<UpdateSongRequest>) -> error::Result<Json<song_request::Model>> {
    if let Err(unauthorized) = state.access_token_service.authorize(&headers).await { return Err(unauthorized); }
    let mut song_request: song_request::ActiveModel = match state.song_request_service.get_song_request_by_id(id).await {
        Ok(Some(song_request)) => song_request.into(),
        Ok(None) => return Err(error::Error::from(
            HttpApiProblem::new(StatusCode::NOT_FOUND).title("Song request not found"))
        ),
        Err(e) => return Err(e.into())
    };
    if let Some(yandex_id) = model.yandex_id { song_request.yandex_id = Set(yandex_id); }
    if let Some(song_name) = model.song_name { song_request.song_name = Set(song_name); }
    if let Some(artist_names) = model.artist_names { song_request.artist_names = Set(artist_names); }
    if let Some(say_hello) = model.say_hello { song_request.say_hello = Set(say_hello); }
    if let Some(hello_from) = model.hello_from { song_request.hello_from = Set(Some(hello_from)); }
    if let Some(hello_to) = model.hello_to { song_request.hello_to = Set(Some(hello_to)); }
    if let Some(hello_text) = model.hello_text { song_request.hello_text = Set(Some(hello_text)); }
    if let Some(explicit_correct) = model.explicit_correct { song_request.explicit_correct = Set(explicit_correct); }
    if let Some(artist_correct) = model.artist_correct { song_request.artist_correct = Set(artist_correct); }
    if let Some(expert_mark) = model.expert_mark { song_request.expert_mark = Set(Some(expert_mark)); }
    Ok(Json(state.song_request_service.update_song_request(song_request).await?))
}

pub async fn delete(headers: HeaderMap, Path(id): Path<i32>, Extension(state): Extension<AppState>) -> error::Result<impl IntoResponse> {
    if let Err(unauthorized) = state.access_token_service.authorize(&headers).await { return Err(unauthorized); }
    match state.song_request_service.delete_song_request(id).await {
        Ok(_) => Ok(StatusCode::NO_CONTENT),
        Err(e) => Err(e.into())
    }
}