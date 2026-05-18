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
    use std::io;

    #[test]
    fn test_not_found_format() {
        let err = AppError::NotFound("file.txt".to_string());
        assert_eq!(err.to_string(), "File not found: file.txt");
    }

    #[test]
    fn test_permission_denied_format() {
        let err = AppError::PermissionDenied("access denied".to_string());
        assert_eq!(err.to_string(), "Permission denied: access denied");
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
        let err = AppError::NotFound("missing".to_string());
        let json = serde_json::to_value(&err).unwrap();
        assert_eq!(json["NotFound"], "missing");
        // The Display impl via thiserror
        assert_eq!(err.to_string(), "File not found: missing");
    }

    #[test]
    fn test_from_io_not_found() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "file not found");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::NotFound(_)));
    }

    #[test]
    fn test_from_io_permission_denied() {
        let io_err = io::Error::new(io::ErrorKind::PermissionDenied, "no permission");
        let app_err: AppError = io_err.into();
        assert!(matches!(app_err, AppError::PermissionDenied(_)));
    }

    #[test]
    fn test_from_io_other() {
        let io_err = io::Error::new(io::ErrorKind::ConnectionRefused, "refused");
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

    // --- EDGE CASES ---

    #[test]
    fn test_not_found_empty_string() {
        let err = AppError::NotFound(String::new());
        assert_eq!(err.to_string(), "File not found: ");
    }

    #[test]
    fn test_internal_unicode_message() {
        let err = AppError::Internal("🔥 システムエラー 🔥".to_string());
        assert!(err.to_string().contains("🔥"));
        let json = serde_json::to_value(&err).unwrap();
        assert!(json["Internal"].as_str().unwrap().contains("🔥"));
    }

    #[test]
    fn test_internal_very_long_message() {
        let msg = "x".repeat(10_000);
        let err = AppError::Internal(msg);
        let s = err.to_string();
        assert_eq!(s.len(), 10_000 + 23);
        assert!(s.starts_with("Internal system error: "));
        assert!(s.ends_with("x"));
    }

    #[test]
    fn test_error_serialize_all_variants() {
        let variants: Vec<(AppError, &str)> = vec![
            (AppError::NotFound("n".to_string()), "NotFound"),
            (
                AppError::PermissionDenied("p".to_string()),
                "PermissionDenied",
            ),
            (AppError::Internal("i".to_string()), "Internal"),
            (
                AppError::InvalidArgument("a".to_string()),
                "InvalidArgument",
            ),
        ];
        for (v, expected_tag) in variants {
            let json = serde_json::to_value(&v).unwrap();
            assert!(
                json.get(expected_tag).is_some(),
                "Missing serde tag: {}",
                expected_tag
            );
            assert!(json[expected_tag].as_str().unwrap_or("").len() > 0);
        }
    }

    #[test]
    fn test_error_display_consistency() {
        let err = AppError::Internal("same".to_string());
        let s1 = err.to_string();
        let err2 = AppError::Internal("same".to_string());
        let s2 = err2.to_string();
        assert_eq!(s1, s2);
    }

    #[test]
    fn test_io_error_preserves_message() {
        let io_err = io::Error::new(io::ErrorKind::NotFound, "custom: file missing");
        let app_err: AppError = io_err.into();
        if let AppError::NotFound(msg) = app_err {
            assert!(
                msg.contains("custom:"),
                "Original message should be preserved"
            );
        } else {
            panic!("Expected NotFound variant");
        }
    }
}
