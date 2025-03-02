use super::error::AppErrorType;

pub fn http_error_code(error_type: AppErrorType) -> u16 {
    match error_type {
        AppErrorType::BadRequest => 400,
        AppErrorType::Unauthorized => 401,
        AppErrorType::Forbidden => 403,
        AppErrorType::NotFound => 404,
        AppErrorType::Conflict => 409,
        AppErrorType::UnsupportedMediaType => 415,
        AppErrorType::InternalServerError => 500,
        AppErrorType::NotImplemented => 501
    }
}
