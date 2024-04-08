mod music_parse;
mod models;
mod error;

use std::env;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use axum::Router;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    let port: u16 = env::var("PORT").unwrap_or("3000".to_string()).parse().unwrap();

    let app = Router::new()
        .merge(music_parse::music_parse_routes());

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port as u16);
    println!("Running on http://{}", addr.to_string());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
