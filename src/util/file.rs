use crate::error::error::{AppError, AppErrorType};

pub async fn remove_file(file_path: &str) -> Result<(), AppError> {
    tokio::fs::remove_file(file_path)
        .await
        .map_err(|err| AppError::new(
            format!("Failed to remove file: {}", err),
            AppErrorType::InternalServerError,
            None
        ))
}

pub fn check_pdf(file_path: &str) -> Result<(), AppError> {
    lopdf::Document::load(file_path)
        .map(|_| ())
        .map_err(|err| AppError::new(
            format!("Failed to load pdf file: {}", err),
            AppErrorType::BadRequest,
            None
        ))
}
