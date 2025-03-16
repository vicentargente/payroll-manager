
use actix_web::web;
use bcrypt::DEFAULT_COST;

use crate::{auth::jwt::generate_token, error::error::{AppError, AppErrorType}, service};

use super::{custom_dto::auth_dto::AuthDto, user::{CreateUserDto, RetrieveUserDto, SignInUserDto, User}};

pub struct AuthService {

}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService {

        }
    }

    pub async fn sign_up(&self, user: CreateUserDto) -> Result<RetrieveUserDto, AppError> {
        if let Err(app_error) = User::check_raw_password(&user.password) {
            return Err(app_error);
        }

        let hashed_pass = web::block(move || {
            bcrypt::hash(&user.password, DEFAULT_COST)
        })
            .await
            .map_err(AppError::internal_from_generic)?
            .map_err(AppError::internal_from_generic)?;

        let hashed_user = CreateUserDto {
            username: user.username,
            email: user.email,
            name: user.name,
            password: hashed_pass,
            company_id: user.company_id,
            role: user.role
        };

        let created_user = service::get().user().create_user(hashed_user).await?;

        Ok(created_user)
    }

    pub async fn sign_in(&self, user: SignInUserDto) -> Result<AuthDto, AppError> {
        let user_service = service::get().user();
        let existing_user = match user_service.get_auth_user_by_username(&user.username).await? {
            Some(user) => user,
            None => {
                return Err(AppError::new(
                    String::from(r#"User with email "$1" does not exist"#),
                    AppErrorType::NotFound,
                    Some(vec![user.username])
                ));
            }
        };

        let password_is_correct = web::block(move || {
            bcrypt::verify(&user.password, &existing_user.password)
        })
            .await
            .map_err(AppError::internal_from_generic)?
            .map_err(AppError::internal_from_generic)?;

        if !password_is_correct {
            return Err(AppError::new(
                String::from("Incorrect password"),
                AppErrorType::Unauthorized,
                None
            ));
        }

        let user = RetrieveUserDto {
            id: existing_user.id,
            username: existing_user.username,
            email: existing_user.email,
            name: existing_user.name,
            company_id: existing_user.company_id
        };

        Ok(AuthDto {
            token: generate_token(user.id),
            user
        })
    }
}