use std::sync::Arc;
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait};
use crate::domain::entities::access_token::{Entity as AccessTokenEntity, Model, ActiveModel};

#[derive(Clone)]
pub struct AccessTokenRepository {
    connection: Arc<DatabaseConnection>
}

impl AccessTokenRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn find_all(&self) -> Result<Vec<Model>, DbErr> {
        AccessTokenEntity::find().all(self.connection.as_ref()).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<Model>, DbErr> {
        AccessTokenEntity::find_by_id(id).one(self.connection.as_ref()).await
    }

    pub async fn create(&self, model: ActiveModel) -> Result<Model, DbErr> {
        model.insert(self.connection.as_ref()).await
    }

    pub async fn update(&self, model: ActiveModel) -> Result<Model, DbErr> {
        model.update(self.connection.as_ref()).await
    }

    pub async fn delete(&self, id: i32) -> Result<u64, DbErr> {
        AccessTokenEntity::delete_by_id(id).exec(self.connection.as_ref()).await.map(|res| res.rows_affected)
    }
}