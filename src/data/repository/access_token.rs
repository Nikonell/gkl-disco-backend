use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr, EntityTrait, ColumnTrait, QueryFilter};
use crate::domain::entities::access_token::{Entity as AccessTokenEntity, Model, Column};

#[derive(Clone)]
pub struct AccessTokenRepository {
    connection: Arc<DatabaseConnection>
}

impl AccessTokenRepository {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        Self { connection }
    }

    pub async fn find_by_token(&self, token: &String) -> Result<Option<Model>, DbErr> {
        AccessTokenEntity::find()
           .filter(Column::Token.eq(token))
           .one(self.connection.as_ref())
           .await
    }
}