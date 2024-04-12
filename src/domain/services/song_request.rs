use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr};
use crate::data::repository::song_request::SongRequestRepository;
use crate::domain::entities::prelude::SongRequestEntity;
use crate::domain::entities::song_request::{Model as SongRequestModel, ActiveModel as SongRequestActiveModel, Model, Column};

#[derive(Clone)]
pub struct SongRequestService {
    repository: SongRequestRepository,
}

impl SongRequestService {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        let repository = SongRequestRepository::new(connection);
        Self { repository }
    }

    pub async fn get_all_song_requests(&self) -> Result<Vec<SongRequestModel>, DbErr> {
        self.repository.find_all().await
    }

    pub async fn get_song_request_by_id(&self, id: i32) -> Result<Option<SongRequestModel>, DbErr> {
        self.repository.find_by_id(id).await
    }

    pub async fn get_song_request_by_song_id(&self, yandex_id: i32) -> Result<Option<Model>, DbErr> {
        self.repository.find_by_song_id(yandex_id).await
    }

    pub async fn create_song_request(&self, model: SongRequestActiveModel) -> Result<SongRequestModel, DbErr> {
        self.repository.create(model).await
    }

    pub async fn update_song_request(&self, model: SongRequestActiveModel) -> Result<SongRequestModel, DbErr> {
        self.repository.update(model).await
    }

    pub async fn delete_song_request(&self, id: i32) -> Result<u64, DbErr> {
        self.repository.delete(id).await
    }
}