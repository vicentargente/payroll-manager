use actix_web::{web, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::json_response::json_response};

use super::company::CreateCompanyDto;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/companies")
            .route("", web::post().to(create_company))
    );
}

pub async fn create_company(company: web::Json<CreateCompanyDto>, claims: Claims) -> impl Responder {
    check_permission!(service::get().permission().create_company(claims.sub).await);

    let company = service::get().company().create_company(company.into_inner()).await;

    json_response(&company)
}
