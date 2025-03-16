use actix_web::{web, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::json_response::json_response};

use super::{company::CreateCompanyDto, custom_models::company_filter::CompanyFilterDto};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/companies")
            .route("", web::post().to(create_company))
            .route("", web::get().to(get_companies))
    );
}

pub async fn create_company(company: web::Json<CreateCompanyDto>, claims: Claims) -> impl Responder {
    check_permission!(service::get().permission().create_company(claims.sub).await);

    let company = service::get().company().create_company(company.into_inner()).await;

    json_response(&company)
}

pub async fn get_companies(filters: web::Query<CompanyFilterDto>, claims: Claims) -> impl Responder {
    check_permission!(service::get().permission().retrieve_companies(claims.sub).await);

    let companies = service::get().company().get_companies(filters.into_inner()).await;

    json_response(&companies)
}
