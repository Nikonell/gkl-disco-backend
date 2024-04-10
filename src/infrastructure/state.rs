use std::ops::Deref;
use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::domain::services::song_request::SongRequestService;

#[derive(Clone)]
pub struct AppState {
    pub song_request_service: SongRequestService,
}

impl AppState {
    pub async fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        AppState {
            song_request_service: SongRequestService::new(db_connection.clone())
        }
    }
}