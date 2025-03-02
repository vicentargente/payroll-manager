use actix_web::{web, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::json_response::json_response};


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/{requested_user_id}", web::get().to(get_profile))
    );
}

pub async fn get_profile(requested_user_id: web::Path<i64>, claims: Claims) -> impl Responder {
    let requested_user_id = requested_user_id.into_inner();

    check_permission!(service::get().permission().retrieve_user(claims.sub, requested_user_id).await);

    let user_service = service::get().user();

    let user = user_service.get_user_by_id(requested_user_id).await;

    json_response(&user)
}
