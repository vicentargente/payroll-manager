use macros::DeriveCustomModel;
use serde::{Deserialize, Serialize};

use crate::{entities::permission::permission::Role, error::error::{AppError, AppErrorType}};

#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "RetrieveUserDb",
    fields(id, username, email, name, company_id)
))]
#[custom_model(model(
    name = "RetrieveAuthUserDb",
    fields(id, username, email, name, password, company_id)
))]
#[custom_model(model(
    name = "CreateUserDb",
    fields(email, username, name, password, company_id)
))]
#[custom_model(model(
    name = "RetrieveUserDto",
    fields(id, username, email, name, company_id),
    extra_derives(Serialize)
))]
#[custom_model(model(
    name = "RetrieveAuthUserDto",
    fields(id, username, email, name, password, company_id),
    extra_derives(Serialize)
))]
#[custom_model(model(
    name = "CreateUserDto",
    fields(username, email, name, password, company_id, role),
    extra_derives(Deserialize)
))]
#[custom_model(model(
    name = "SignInUserDto",
    fields(username, password),
    extra_derives(Deserialize)
))]

#[allow(dead_code)]
pub struct User {
    id: i64,
    username: String,
    email: Option<String>,
    name: String,
    password: String,
    company_id: i64,
    role: Role
}

impl User {
    pub fn new(id: i64, username: String, email: Option<String>, name: String, password: String, company_id: i64, role: Role) -> User {
        User {
            id,
            username,
            email,
            name,
            password,
            company_id,
            role
        }
    }

    pub fn check_username(username: &str) -> Result<(), AppError> {
        if username.len() == 0 || username.len() > 50 {
            return Err(AppError::new(
                String::from("The username must be between 1 and 50 characters long"),
                AppErrorType::BadRequest,
                None
            ))
        }

        let regex = regex::Regex::new(r"^[a-zA-Z0-9]+(?:(?:\.|_)[a-zA-Z0-9]+)*$").unwrap();
        if !regex.is_match(username) {
            return Err(AppError::new(
                String::from("Invalid username: $1"),
                AppErrorType::BadRequest,
                Some(vec![username.to_string()])
            ))
        }

        Ok(())
    }

    pub fn check_email(email: &Option<String>) -> Result<(), AppError> {
        if let Some(email) = email {
            let regex = regex::Regex::new(r"^[a-z0-9](\.?[a-z0-9_-]){0,}@[a-z0-9-]+\.([a-z]{1,6}\.)?[a-z]{2,6}$").unwrap();
            if !regex.is_match(email) {
                return Err(AppError::new(
                    String::from("Invalid email: $1"),
                    AppErrorType::BadRequest,
                    Some(vec![email.to_string()])
                ))
            }
        }

        Ok(())
    }

    pub fn check_name(name: &str) -> Result<(), AppError> {
        if name.len() == 0 || name.len() > 50 {
            return Err(AppError::new(
                String::from("The name must be between 1 and 50 characters long"),
                AppErrorType::BadRequest,
                None
            ))
        }

        Ok(())
    }

    pub fn check_raw_password(password: &str) -> Result<(), AppError> {
        if password.len() < 8 {
            return Err(AppError::new(
                String::from("Password must be at least 8 characters long"),
                AppErrorType::BadRequest,
                None
            ))
        }

        Ok(())
    }
}

impl RetrieveUserDb {
    pub fn to_retrieve_user_dto(self) -> Result<RetrieveUserDto, AppError> {
        // User::check_name(&self.name)?;

        Ok(RetrieveUserDto {
            id: self.id,
            username: self.username,
            email: self.email,
            name: self.name,
            company_id: self.company_id
        })
    }
}

impl RetrieveAuthUserDb {
    pub fn to_retrieve_auth_user_dto(self) -> Result<RetrieveAuthUserDto, AppError> {
        // User::check_email(&self.email)?;

        Ok(RetrieveAuthUserDto {
            id: self.id,
            username: self.username,
            email: self.email,
            name: self.name,
            password: self.password,
            company_id: self.company_id
        })
    }
}

impl CreateUserDb {
    // pub fn to_create_user_dto(self) -> Result<CreateUserDto, AppError> {
    //     // User::check_email(&self.email)?;
    //     // User::check_name(&self.name)?;

    //     Ok(CreateUserDto {
    //         username: self.username,
    //         email: self.email,
    //         name: self.name,
    //         password: self.password,
    //         company_id: self.company_id
    //     })
    // }

    pub fn from_create_user_dto(user: CreateUserDto) -> Result<CreateUserDb, AppError> {
        User::check_username(&user.username)?;
        User::check_email(&user.email)?;
        User::check_name(&user.name)?;

        Ok(CreateUserDb {
            username: user.username,
            email: user.email,
            name: user.name,
            password: user.password,
            company_id: user.company_id
        })
    }
}
