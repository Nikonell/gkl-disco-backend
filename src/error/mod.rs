use axum::{
    http::{StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use http_api_problem::HttpApiProblem;
use validator::ValidationErrors;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("an internal database error occurred")]
    DbErr(#[from] sea_orm::DbErr),
    #[error("an internal server error occurred")]
    Anyhow(#[from] anyhow::Error),
    #[error("validation error in request body")]
    InvalidEntity(#[from] ValidationErrors),
    #[error("http error")]
    HttpProblem(#[from] HttpApiProblem),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let payload = match self {
            Self::InvalidEntity(errors) => HttpApiProblem::new(StatusCode::UNPROCESSABLE_ENTITY)
                .title("Unprocessable entity in request body")
                .detail(errors.to_string()),
            Self::HttpProblem(problem) => problem,
            Self::Anyhow(error) => HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
                .title("Internal Server Error")
                .detail(error.to_string()),
            _ => HttpApiProblem::new(StatusCode::INTERNAL_SERVER_ERROR)
                .title("Internal Server Error")
        };
        (payload.status.unwrap(), Json(payload)).into_response()
    }
}