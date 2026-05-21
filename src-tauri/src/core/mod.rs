pub mod error;
pub mod ipc;

pub use error::{AppError, AppResult};
pub use ipc::{ApiResponse, ToApiResponse};
