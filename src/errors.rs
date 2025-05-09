use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::env;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Errors {
    #[error("initialize configuration failed because: {0}")]
    ConifgError(#[from] env::VarError),
    #[error("error from vestige: {0}")]
    VestigeError(#[from] vestige::Errors),
    #[error("error from database: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Unknown Error")]
    Unknown,
}

impl IntoResponse for Errors {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unknown Internal Server Error",
            ),
        };
        (status, error_message).into_response()
    }
}

pub type Result<T> = std::result::Result<T, Errors>;
