use sqlx::SqlitePool;

use crate::error::error::{AppError, AppErrorType};

pub async fn get_db_pool(db_url: &str) -> sqlx::Pool<sqlx::Sqlite> {
    SqlitePool::connect(db_url)
        .await
        .expect("Failed to connect to database")
}

pub fn to_app_error(error: sqlx::Error) -> AppError {
    AppError::new(
        error.to_string(),
        AppErrorType::InternalServerError,
        None
    )
}

pub async fn run_migrations(pool: &sqlx::Pool<sqlx::Sqlite>) -> Result<(), AppError> {
    sqlx::migrate!()
        .run(pool)
        .await
        .map_err(
            |error| AppError::new(
                error.to_string(),
                AppErrorType::InternalServerError,
                None
            )
        )?;

    Ok(())
}
