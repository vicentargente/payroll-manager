use actix_multipart::Multipart;
use actix_web::{web, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::{json_response::json_response, multipart::{extract_body, extract_file}}};



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payrolls")
            .route("", web::post().to(upload_payroll))
    );
}

pub async fn upload_payroll(mut payload: Multipart, claims: Claims) -> impl Responder {
    let payroll = match extract_body(&mut payload).await {
        Ok(body) => body,
        Err(err) => return json_response(&Err(err))
    };

    check_permission!(service::get().permission().create_payroll(claims.sub, &payroll).await);

    let file_info = match extract_file(&mut payload).await {
        Ok(file_info) => file_info,
        Err(err) => return json_response(&Err(err))
    };

    let created_payroll = service::get().payroll().create_payroll(
        payroll,
        &file_info.file_path,
        &file_info.unique_file_name,
        &file_info.original_file_name
    ).await;

    json_response(&created_payroll)
}
