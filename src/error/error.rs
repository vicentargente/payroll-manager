use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug)]
pub struct AppError {
    message: String,
    r#type: AppErrorType,
    parameters: Option<Vec<String>>
}

impl AppError {
    pub fn new(message: String, r#type: AppErrorType, parameters: Option<Vec<String>>) -> AppError {
        println!("{message}");

        AppError {
            message,
            r#type,
            parameters
        }
    }
    
    pub fn code(&self, adapter: fn(AppErrorType) -> u16) -> u16 {
        adapter(self.r#type)
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    /// Converts any error that implements the `Display` trait into an `AppError`
    /// with the `InternalServerError` type.
    ///
    /// # Arguments
    ///
    /// * `err` - The error to be converted, which must implement the `Display` trait.
    ///
    /// # Returns
    ///
    /// * `AppError` - An application error with the `InternalServerError` type.
    ///
    /// # Example
    ///
    /// ```
    /// let error = std::io::Error::new(std::io::ErrorKind::Other, "some error");
    /// let app_error = AppError::internal_from_generic(error);
    /// assert_eq!(app_error.r#type, AppErrorType::InternalServerError);
    /// ```
    pub fn internal_from_generic<E>(err: E) -> AppError
    where E: std::fmt::Display
    {
        AppError::new(
            err.to_string(),
            AppErrorType::InternalServerError,
            None
        )
    }

    fn scope(&self) -> AppErrorScope {
        match self.r#type {
            AppErrorType::BadRequest |
            AppErrorType::Unauthorized |
            AppErrorType::Forbidden |
            AppErrorType::NotFound |
            AppErrorType::Conflict |
            AppErrorType::UnsupportedMediaType => AppErrorScope::Public,

            AppErrorType::InternalServerError |
            AppErrorType::NotImplemented => AppErrorScope::Internal
        }
    }
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer
    {
        match self.scope() {
            AppErrorScope::Internal => {
                let s = serializer.serialize_struct("Error", 0)?;
                serde::ser::SerializeStruct::end(s)
            },
            AppErrorScope::Public => {
                let mut s = serializer.serialize_struct("Error", 2)?;
                s.serialize_field("message", &self.message)?;
                s.serialize_field("parameters", self.parameters.as_ref().unwrap_or(&vec![]))?;
                serde::ser::SerializeStruct::end(s)
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AppErrorType {
    BadRequest,
    Unauthorized,
    Forbidden,
    NotFound,
    Conflict,
    UnsupportedMediaType,
    InternalServerError,
    NotImplemented
}

enum AppErrorScope {
    Internal,
    Public
}
