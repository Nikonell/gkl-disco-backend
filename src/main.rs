use axum::{Extension, Router};
use std::net::SocketAddr;
use migration::{Migrator, MigratorTrait};
use tokio::net::TcpListener;
use crate::presentation::routes::external_music_routes::external_music_routes;

mod domain;
mod data;
mod application;
mod presentation;
mod config;
mod error;
mod infrastructure;

#[tokio::main]
async fn main() {
    let db_connection = infrastructure::database::establish_connection().await;
    Migrator::up(&db_connection, None).await.unwrap();

    let app = Router::new()
        .merge(external_music_routes())
        .layer(Extension(db_connection));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}