use std::sync::Arc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait};
use crate::domain::entities::song_request::{Entity as SongRequestEntity, Model, ActiveModel, Column};

#[derive(Clone)]
pub struct SongRequestRepository {
    connection: Arc<DatabaseConnection>
}

impl SongRequestRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        SongRequestEntity::find().all(self.connection.as_ref()).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        SongRequestEntity::find_by_id(id).one(self.connection.as_ref()).await
    }

    pub async fn find_by_song_id(&self, yandex_id: i32) -> Result<Option<Model>, DbErr> {
        SongRequestEntity::find()
            .filter(Column::YandexId.eq(yandex_id))
            .one(self.connection.as_ref()).await
    }

    pub async fn create(&self, model: ActiveModel) -> Result<Model, DbErr> {
        model.insert(self.connection.as_ref()).await
    }

    pub async fn update(&self, model: ActiveModel) -> Result<Model, DbErr> {
        model.update(self.connection.as_ref()).await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        SongRequestEntity::delete_by_id(id).exec(self.connection.as_ref()).await.map(|res| res.rows_affected)
    }
}