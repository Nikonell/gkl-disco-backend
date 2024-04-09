use sea_orm::{DatabaseConnection, DbErr};
use crate::data::repository::song_request::SongRequestRepository;
use crate::domain::entities::song_request::{Model as SongRequestModel, ActiveModel as SongRequestActiveModel};

pub struct SongRequestService<'a> {
    repository: SongRequestRepository<'a>,
}

impl<'a> SongRequestService<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        let repository = SongRequestRepository::new(connection);
        Self { repository }
    }

    pub async fn get_all_song_requests(&self) -> Result<Vec<SongRequestModel>, DbErr> {
        self.repository.find_all().await
    }

    pub async fn get_song_request_by_id(&self, id: i32) -> Result<Option<SongRequestModel>, DbErr> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_song_request(&self, model: SongRequestActiveModel) -> Result<SongRequestModel, DbErr> {
        self.repository.create(model).await
    }

    pub async fn delete_song_request(&self, id: i32) -> Result<u64, DbErr> {
        self.repository.delete(id).await
    }
}