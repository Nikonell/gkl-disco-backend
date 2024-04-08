use axum::http::{StatusCode};
use axum::Json;
use axum::response::{IntoResponse, Response};
use http_api_problem::HttpApiProblem;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Internal server error")]
    Anyhow(#[from] anyhow::Error),
    #[error("Request body validation error")]
    InvalidEntity(#[from] validator::ValidationError)
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let payload = match self {
            Self::InvalidEntity(errors) => HttpApiProblem::new(StatusCode::UNPROCESSABLE_ENTITY)
                .title("Unprocessable entity in request body")
                .detail(errors.to_string()),
            Self::Anyhow(errors) => HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
                .title("Internal Server Error")
                .detail(errors.to_string()),
        };
        (payload.status.unwrap(), Json(payload)).into_response()
    }
}
