use axum::{http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::application::use_cases::error::UseCaseError;

#[derive(Serialize)]
pub enum ClientApiError {
    NotFound,
    BadInput,
    Unknown,
}

pub type ApiResult<T> = axum::response::Result<T, ClientApiError>;

impl From<UseCaseError> for ClientApiError {
    fn from(value: UseCaseError) -> Self {
        match value {
            UseCaseError::NotFound => ClientApiError::NotFound,
            UseCaseError::Unknown => ClientApiError::Unknown,
            UseCaseError::BadInput => ClientApiError::BadInput,
        }
    }
}

impl IntoResponse for ClientApiError {
    fn into_response(self) -> axum::response::Response {
        let response_code = match self {
            ClientApiError::NotFound => StatusCode::NOT_FOUND,
            ClientApiError::Unknown => StatusCode::SERVICE_UNAVAILABLE,
            ClientApiError::BadInput => StatusCode::BAD_REQUEST,
        };

        response_code.into_response()
    }
}
