use actix_web::{web, Responder};

use crate::{auth::jwt::Claims, check_permission, service, util::json_response::json_response};

use super::user::{CreateUserDto, SignInUserDto};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/signup", web::post().to(sign_up))
            .route("/signin", web::post().to(sign_in))
    );
}

pub async fn sign_up(user: web::Json<CreateUserDto>, claims: Claims) -> impl Responder {
    check_permission!(service::get().permission().create_user(claims.sub, &user).await);

    let auth_service = service::get().auth();
    let created_user = auth_service.sign_up(user.into_inner()).await;
    
    json_response(&created_user)
}

pub async fn sign_in(credentials: web::Json<SignInUserDto>) -> impl Responder {
    let auth_service = service::get().auth();
    let logged_user = auth_service.sign_in(credentials.into_inner()).await;

    json_response(&logged_user)
}
