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
