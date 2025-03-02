use sqlx::SqliteConnection;

use crate::{error::error::AppError, util::db::to_app_error};

use super::user::{CreateUserDb, RetrieveAuthUserDb, RetrieveUserDb};

pub struct UserRepository {}

impl UserRepository {
    pub fn new() -> UserRepository {
        UserRepository {

        }
    }
    
    // tx: &sqlx::Transaction<'_, sqlx::Sqlite>
    pub async fn create_user(&self, tx: &mut SqliteConnection, user: &CreateUserDb) -> Result<RetrieveUserDb, AppError> {
        sqlx::query_as!(
            RetrieveUserDb,
            r#"
            INSERT INTO AppUser (username, email, name, password, company_id)
            VALUES($1, $2, $3, $4, $5)
            RETURNING id as "id!: i64", username, email, name, company_id
            "#,
            user.username,
            user.email,
            user.name,
            user.password,
            user.company_id
        )
        .fetch_one(tx)
        .await
        .map_err(to_app_error)
    }

    pub async fn user_exists_by_username(&self, tx: &mut SqliteConnection, username: &str) -> Result<bool, AppError> {
        sqlx::query_scalar!(
            r#"
            SELECT 1 as "exists!: i64"
            FROM AppUser
            WHERE username = $1
            LIMIT 1
            "#,
            username
        )
        .fetch_optional(tx)
        .await
        .map(|val| val.is_some())
        .map_err(to_app_error)
    }

    pub async fn get_auth_user_by_username(&self, tx: &mut SqliteConnection, username: &str) -> Result<Option<RetrieveAuthUserDb>, AppError> {
        sqlx::query_as!(
            RetrieveAuthUserDb,
            r#"
            SELECT id as "id!: i64", username, email, name, password, company_id
            FROM AppUser
            WHERE username = $1
            LIMIT 1
            "#,
            username
        )
        .fetch_optional(tx)
        .await
        .map_err(to_app_error)
    }

    pub async fn get_user_by_id(&self, tx: &mut SqliteConnection, id: i64) -> Result<Option<RetrieveUserDb>, AppError> {
        sqlx::query_as!(
            RetrieveUserDb,
            r#"
            SELECT id as "id!: i64", username, email, name, company_id
            FROM AppUser
            WHERE id = $1
            LIMIT 1
            "#,
            id
        )
        .fetch_optional(tx)
        .await
        .map_err(to_app_error)
    }

    pub async fn get_company_id_by_user_id(&self, tx: &mut SqliteConnection, user_id: i64) -> Result<Option<i64>, AppError> {
        sqlx::query_scalar!(
            r#"
            SELECT company_id as "company_id: i64"
            FROM AppUser
            WHERE id = $1
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(tx)
        .await
        .map_err(to_app_error)
    }
}
