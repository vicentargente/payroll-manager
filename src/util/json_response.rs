use actix_web::{http::StatusCode, HttpResponse, Responder};
use serde::Serialize;

use crate::error::{error::AppError, http_error_code::http_error_code};

pub fn json_response<T>(val: &Result<T, AppError>) -> impl Responder
where T: Serialize
{
    match val {
        Ok(value) => HttpResponse::Ok().json(value),
        Err(err) => HttpResponse::build(StatusCode::from_u16(err.code(http_error_code)).unwrap()).json(err),
    }
}
