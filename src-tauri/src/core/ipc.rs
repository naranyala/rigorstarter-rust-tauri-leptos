use serde::Serialize;
use crate::core::error::AppResult;

/// Standardized response structure for all Tauri commands.
/// This ensures the frontend always receives a consistent shape.
#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    /// Wraps a successful result.
    pub fn ok(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
        }
    }

    /// Wraps an error into a standard response.
    pub fn err(err: impl Into<String>) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(err.into()),
        }
    }
}

/// A helper trait to easily convert AppResult into ApiResponse for Tauri.
pub trait ToApiResponse<T> {
    fn to_api_response(self) -> ApiResponse<T>;
}

impl<T, E: std::fmt::Display> ToApiResponse<T> for Result<T, E> {
    fn to_api_response(self) -> ApiResponse<T> {
        match self {
            Ok(val) => ApiResponse::ok(val),
            Err(e) => ApiResponse::err(e.to_string()),
        }
    }
}
