use std::sync::Arc;
use axum::http::HeaderMap;
use http_api_problem::HttpApiProblem;
use reqwest::StatusCode;
use sea_orm::DatabaseConnection;
use crate::{data::repository::access_token::AccessTokenRepository, error};

#[derive(Clone)]
pub struct AccessTokenService {
    repository: AccessTokenRepository,
}

impl AccessTokenService {
    pub fn new(connection: Arc<DatabaseConnection>) -> Self {
        let repository = AccessTokenRepository::new(connection);
        Self { repository }
    }

    pub async fn authorize(&self, headers: &HeaderMap) -> error::Result<()> {
        let err = error::Error::from(HttpApiProblem::new(StatusCode::UNAUTHORIZED).title("Unauthorized"));
        let Some(token) = headers.get("Authorization")
            .and_then(|v| v.to_str().ok()) else { return Err(err); };
        match self.repository.find_by_token(&token.to_string()).await {
            Ok(Some(_)) => return Ok(()),
            _ => return Err(err),
        }
    }
}