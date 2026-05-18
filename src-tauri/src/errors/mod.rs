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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_not_found_format() {
        let err = AppError::NotFound("test.txt".to_string());
        assert_eq!(err.to_string(), "File not found: test.txt");
    }

    #[test]
    fn test_permission_denied_format() {
        let err = AppError::PermissionDenied("/root".to_string());
        assert_eq!(err.to_string(), "Permission denied: /root");
    }

    #[test]
    fn test_internal_format() {
        let err = AppError::Internal("something broke".to_string());
        assert_eq!(err.to_string(), "Internal system error: something broke");
    }

    #[test]
    fn test_invalid_argument_format() {
        let err = AppError::InvalidArgument("bad input".to_string());
        assert_eq!(err.to_string(), "Invalid argument: bad input");
    }

    #[test]
    fn test_serialize_json() {
        let err = AppError::NotFound("missing.json".to_string());
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("NotFound"));
        assert!(json.contains("missing.json"));
    }

    #[test]
    fn test_from_io_not_found() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "no such file");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::NotFound(_)));
    }

    #[test]
    fn test_from_io_permission_denied() {
        let io_err = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "access denied");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::PermissionDenied(_)));
    }

    #[test]
    fn test_from_io_other() {
        let io_err = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "conn reset");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::Internal(_)));
    }

    #[test]
    fn test_error_impl_debug() {
        let err = AppError::Internal("debug me".to_string());
        let debug = format!("{:?}", err);
        assert!(debug.contains("Internal"));
        assert!(debug.contains("debug me"));
    }
}
