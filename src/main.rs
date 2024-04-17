use axum::{Extension, Router};
use std::net::SocketAddr;
use std::sync::Arc;
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;
use crate::infrastructure::state::AppState;
use crate::presentation::routes::external_music_routes::external_music_routes;
use crate::presentation::routes::song_request_routes::song_request_routes;
use crate::presentation::routes::forbidden_artist_routes::forbidden_artist_routes;

mod domain;
mod data;
mod application;
mod presentation;
mod config;
mod error;
mod infrastructure;

#[tokio::main]
async fn main() {
    let db_connection = Arc::new(infrastructure::database::establish_connection().await);
    Migrator::up(db_connection.as_ref(), None).await.unwrap();

    let state = AppState::new(db_connection).await;

    let app = Router::new()
        .merge(external_music_routes())
        .nest("/song_requests", song_request_routes())
        .nest("/forbiden_artists", forbidden_artist_routes())
        .layer(Extension(state));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}