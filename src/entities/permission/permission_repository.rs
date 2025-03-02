use sqlx::SqliteConnection;

use crate::{error::error::AppError, util::db::to_app_error};

use super::permission::Permission;

pub struct PermissionRepository {

}

impl PermissionRepository {
    pub fn new() -> PermissionRepository {
        PermissionRepository {

        }
    }

    pub async fn create_permission(&self, tx: &mut SqliteConnection, permission: &Permission) -> Result<(), AppError> {
        sqlx::query!(
            r#"
            INSERT INTO Permission (user_id, user, payroll, company)
            VALUES($1, $2, $3, $4)
            "#,
            permission.user_id,
            permission.user,
            permission.payroll,
            permission.company
        )
        .execute(tx)
        .await
        .map_err(to_app_error)
        .map(|_| ())
    }

    pub async fn get_permission_by_user_id(&self, tx: &mut SqliteConnection, user_id: i64) -> Result<Option<Permission>, AppError> {
        sqlx::query_as!(
            Permission,
            r#"
            SELECT
                user_id as "user_id: i64",
                user as "user: i16",
                payroll as "payroll: i16",
                company as "company: i16"
            FROM Permission 
            WHERE user_id = $1
            "#,
            user_id
        )
        .fetch_optional(tx)
        .await
        .map_err(to_app_error)
    }
}
