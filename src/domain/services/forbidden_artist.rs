use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr};
use crate::data::repository::forbidden_artist::ForbiddenArtistRepository;
use crate::domain::entities::forbidden_artist::{Model as ForbiddenArtistModel, ActiveModel as ForbiddenArtistActiveModel};

#[derive(Clone)]
pub struct ForbiddenArtistService {
    repository: ForbiddenArtistRepository,
}

impl ForbiddenArtistService {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        let repository = ForbiddenArtistRepository::new(connection);
        Self { repository }
    }

    pub async fn get_all_forbidden_artists(&self) -> Result<Vec<ForbiddenArtistModel>, DbErr> {
        self.repository.find_all().await
    }

    pub async fn get_first_forbidden_artist(&self, name: &String) -> Result<Option<ForbiddenArtistModel>, DbErr> {
        self.repository.find_first(name).await
    }   
}