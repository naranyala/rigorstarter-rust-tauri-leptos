use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("File not found: {0}")]
    NotFound(String),
    #[error("Permission denied: {0}")]
    PermissionDenied(String),
    #[error("Internal system error: {0}")]
    Internal(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        use std::io::ErrorKind::*;
        match err.kind() {
            NotFound => AppError::NotFound(err.to_string()),
            PermissionDenied => AppError::PermissionDenied(err.to_string()),
            _ => AppError::Internal(err.to_string()),
        }
    }
}
