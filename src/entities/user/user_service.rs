use macros::executor;
use sqlx::{SqliteConnection, SqlitePool};

use crate::{error::error::{AppError, AppErrorType}, service};

use super::{user::{CreateUserDb, CreateUserDto, RetrieveAuthUserDto, RetrieveUserDto}, user_repository::UserRepository};

pub struct UserService {
    db_pool: SqlitePool,
    user_repository: UserRepository
}

impl UserService {
    pub fn new(db_pool: SqlitePool, user_repository: UserRepository) -> UserService {
        UserService {
            db_pool,
            user_repository
        }
    }

    #[executor] 
    pub async fn create_user(&self, create_user_dto: CreateUserDto) -> Result<RetrieveUserDto, AppError> {
        if self.user_repository.user_exists_by_username(tx, &create_user_dto.username).await? {
            return Err(AppError::new(
                String::from(r#"User with username "$1" already exists"#),
                AppErrorType::Conflict,
                Some(vec![create_user_dto.username])
            ));
        }

        if !service::get().company().company_exists_by_id(create_user_dto.company_id).await? {
            return Err(AppError::new(
                String::from(r#"Company with id "$1" does not exist"#),
                AppErrorType::BadRequest,
                Some(vec![create_user_dto.company_id.to_string()])
            ))
        }

        let role = create_user_dto.role;
        let create_user_db = CreateUserDb::from_create_user_dto(create_user_dto)?;

        let created_user = self.user_repository.create_user(tx, &create_user_db).await?;

        service::get().permission().create_permission_from_role_executor(tx, created_user.id, role).await?;

        Ok(created_user.to_retrieve_user_dto()?)
    }

    #[executor]
    pub async fn get_auth_user_by_username(&self, username: &str) -> Result<Option<RetrieveAuthUserDto>, AppError> {
        match self.user_repository.get_auth_user_by_username(tx, username).await? {
            Some(user) => Ok(Some(user.to_retrieve_auth_user_dto()?)),
            None => Err(AppError::new(
                String::from(r#"User with username "$1" does not exist"#),
                AppErrorType::NotFound,
                Some(vec![username.to_string()])
            ))
        }
    }

    #[executor]
    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<RetrieveUserDto>, AppError> {
        match self.user_repository.get_user_by_id(tx, id).await? {
            Some(user) => Ok(Some(user.to_retrieve_user_dto()?)),
            None => Err(AppError::new(
                String::from(r#"User with id "$1" does not exist"#),
                AppErrorType::NotFound,
                Some(vec![id.to_string()])
            )),
        }
    }

    #[executor]
    pub async fn get_company_by_user_id(&self, user_id: i64) -> Result<Option<i64>, AppError> {
        self.user_repository.get_company_id_by_user_id(tx, user_id).await
    }
}
