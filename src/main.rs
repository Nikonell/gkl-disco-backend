use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::presentation::routes::external_music_routes::external_music_routes;

mod domain;
mod data;
mod application;
mod presentation;
mod config;
mod error;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .merge(external_music_routes());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}