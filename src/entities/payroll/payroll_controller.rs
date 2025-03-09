use actix_multipart::Multipart;
use actix_web::{web, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::{json_response::json_response, multipart::{extract_body, extract_file}}};

use super::custom_models::payroll_filter::PayrollFilterDto;



pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payrolls")
            .route("", web::post().to(upload_payroll))
            .route("", web::get().to(get_payrolls))
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

pub async fn get_payrolls(filters: web::Query<PayrollFilterDto>, claims: Claims) -> impl Responder {
    check_permission!(service::get().permission().get_payrolls(claims.sub, filters.user_id).await);

    let payrolls = service::get().payroll().get_filtered_payrolls(filters.into_inner()).await;

    json_response(&payrolls)
}
