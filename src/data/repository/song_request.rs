use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use crate::domain::entities::song_request::{Entity as SongRequestEntity, Model, ActiveModel};

pub struct SongRequestRepository<'a> {
    connection: &'a DatabaseConnection
}

impl<'a> SongRequestRepository<'a> {
    pub fn new(connection: &'a DatabaseConnection) -> Self {
        Self { connection }
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        SongRequestEntity::find().all(self.connection).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        SongRequestEntity::find_by_id(id).one(self.connection).await
    }

    pub async fn create(&self, model: ActiveModel) -> Result<Model, DbErr> {
        model.insert(self.connection).await
    }

    pub async fn update(&self, model: ActiveModel) -> Result<Model, DbErr> {
        model.update(self.connection).await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        SongRequestEntity::delete_by_id(id).exec(self.connection).await.map(|res| res.rows_affected)
    }
}