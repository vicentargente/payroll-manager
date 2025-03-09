use actix_multipart::Multipart;
use actix_web::{web, HttpResponse, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::{json_response::json_response, multipart::{extract_body, extract_file}}};

use super::custom_models::payroll_filter::PayrollFilterDto;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payrolls")
            .route("", web::post().to(upload_payroll))
            .route("", web::get().to(get_payrolls))
            .route("/{payroll_id}/download", web::get().to(download_payroll))
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
        &file_info.original_file_name,
        file_info.file_size
    ).await;

    json_response(&created_payroll)
}

pub async fn get_payrolls(filters: web::Query<PayrollFilterDto>, claims: Claims) -> impl Responder {
    check_permission!(service::get().permission().get_payrolls(claims.sub, filters.user_id).await);

    let payrolls = service::get().payroll().get_filtered_payrolls(filters.into_inner()).await;

    json_response(&payrolls)
}

pub async fn download_payroll(payroll_id: web::Path<i64>, claims: Claims) -> impl Responder {
    let payroll_id = payroll_id.into_inner();

    match service::get().permission().get_payroll(claims.sub, payroll_id).await {
        Ok(is_allowed) => {
            if !is_allowed {
                return HttpResponse::Forbidden().finish();
            }
        },
        Err(_) => return HttpResponse::InternalServerError().finish(),
    }

    let payroll_data = service::get().payroll().download_payroll(payroll_id).await.unwrap();

    let mut builder = HttpResponse::Ok();

    builder
        .content_type(payroll_data.content_type)
        .append_header(("Content-Disposition", format!("attachment; filename=\"{}\"", payroll_data.filename)))
        .append_header(("Content-Length", payroll_data.file_size.to_string()));

    builder.streaming(payroll_data.stream)
}
