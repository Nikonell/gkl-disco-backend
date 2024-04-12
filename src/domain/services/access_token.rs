use std::sync::Arc;
use sea_orm::{DatabaseConnection, DbErr};
use crate::data::repository::access_token::AccessTokenRepository;
use crate::domain::entities::access_token::{Model as AccessTokenModel, ActiveModel as AccessTokenActiveModel};

#[derive(Clone)]
pub struct AccessTokenService {
    repository: AccessTokenRepository,
}

impl AccessTokenService {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        let repository = AccessTokenRepository::new(connection);
        Self { repository }
    }

    pub async fn get_all_access_tokens(&self) -> Result<Vec<AccessTokenModel>, DbErr> {
        self.repository.find_all().await
    }

    pub async fn get_access_token_by_id(&self, id: i32) -> Result<Option<AccessTokenModel>, DbErr> {
        self.repository.find_by_id(id).await
    }

    pub async fn create_access_token(&self, model: AccessTokenActiveModel) -> Result<AccessTokenModel, DbErr> {
        self.repository.create(model).await
    }

    pub async fn update_access_token(&self, model: AccessTokenActiveModel) -> Result<AccessTokenModel, DbErr> {
        self.repository.update(model).await
    }

    pub async fn delete_access_token(&self, id: i32) -> Result<u64, DbErr> {
        self.repository.delete(id).await
    }
}