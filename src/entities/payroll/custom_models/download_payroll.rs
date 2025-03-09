use std::pin::Pin;

use actix_web::web;
use futures_util::Stream;

pub struct DownloadPayrollDto {
    pub filename: String,
    pub content_type: String,
    pub file_size: i64,
    pub stream: Pin<Box<dyn Stream<Item = Result<web::Bytes, std::io::Error>> + Send>>
}
