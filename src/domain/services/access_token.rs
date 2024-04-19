use std::sync::Arc;
use http_api_problem::HttpApiProblem;
use reqwest::StatusCode;
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

    pub async fn authorize(&self, token: String) -> Result<(), HttpApiProblem> {
        match self.repository.find_by_token(&token).await {
            Ok(Some(_)) => return Ok(()),
            _ => return Err(HttpApiProblem::new(StatusCode::UNAUTHORIZED).title("Unauthorized")),
        }
    }
}