use std::sync::Arc;
use sea_orm::DatabaseConnection;
use crate::domain::services::access_token::AccessTokenService;
use crate::domain::services::forbidden_artist::ForbiddenArtistService;
use crate::domain::services::song_request::SongRequestService;

#[derive(Clone)]
pub struct AppState {
    pub song_request_service: SongRequestService,
    pub access_token_service: AccessTokenService,
    pub forbidden_artist_service: ForbiddenArtistService,
}

impl AppState {
    pub async fn new(db_connection: Arc<DatabaseConnection>) -> Self {
        AppState {
            song_request_service: SongRequestService::new(db_connection.clone()),
            access_token_service: AccessTokenService::new(db_connection.clone()),
            forbidden_artist_service: ForbiddenArtistService::new(db_connection.clone()),
        }
    }
}