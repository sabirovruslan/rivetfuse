use anyhow::Error as AnyhowError;
use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

pub type AppResult<T = ()> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    UnknownError(#[from] AnyhowError),

    #[error(transparent)]
    AddrParseError(#[from] std::net::AddrParseError),

    #[error("I/O error")]
    IoError(#[from] std::io::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::AddrParseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::IoError(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()),
            AppError::UnknownError(err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Unexpected error: {}", err),
            ),
        };

        let body = json!( { "error": message } );

        (status, Json(body)).into_response()
    }
}
