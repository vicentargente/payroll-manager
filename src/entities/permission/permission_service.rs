use macros::executor;
use sqlx::{SqliteConnection, SqlitePool};

use crate::{entities::payroll::payroll::CreatePayrollDto, error::error::{AppError, AppErrorType}, service::{self}, user::user::CreateUserDto};

use super::{permission::{Operation, Permission, Role, Scope}, permission_repository::PermissionRepository};

pub struct PermissionService {
    db_pool: SqlitePool,
    permission_repository: PermissionRepository
}

impl PermissionService {
    pub fn new(db_pool: SqlitePool, permission_repository: PermissionRepository) -> PermissionService {
        PermissionService {
            db_pool,
            permission_repository
        }
    }

    async fn get_permission(&self, tx: &mut SqliteConnection, user_id: i64) -> Result<Permission, AppError> {
        match self.permission_repository.get_permission_by_user_id(tx, user_id).await? {
            Some(permission) => Ok(permission),
            None => return Err(AppError::new(
                String::from("User has not permission"),
                AppErrorType::Forbidden,
                None
            ))
        }
    }

    #[executor]
    pub async fn create_permission(&self, permission: &Permission) -> Result<(), AppError> {
        self.permission_repository.create_permission(tx, permission).await
    }

    #[executor]
    pub async fn create_permission_from_role(&self, user_id: i64, role: Role) -> Result<(), AppError> {
        let permission = Permission::from_role(user_id, role);

        self.permission_repository.create_permission(tx, &permission).await
    }

    #[executor]
    pub async fn create_user(&self, actor_user_id: i64, user: &CreateUserDto) -> Result<bool, AppError> {
        let permission = self.get_permission(tx, actor_user_id).await?;
        let operation = Operation::Create;

        Ok(
            permission.user(Scope::Any(operation)) ||
            (permission.user(Scope::SelfCompany(operation)) && Self::actor_and_created_user_same_company(tx, actor_user_id, user).await)
        )
    }

    #[executor]
    pub async fn retrieve_user(&self, actor_user_id: i64, requested_user_id: i64) -> Result<bool, AppError> {
        let permission = self.get_permission(tx, actor_user_id).await?;
        let operation = Operation::Read;

        Ok(
            (permission.user(Scope::Owned(operation)) && actor_user_id == requested_user_id) ||
            permission.user(Scope::Any(operation)) ||
            (permission.user(Scope::SelfCompany(operation)) && Self::actor_and_requested_user_same_company(tx, actor_user_id, requested_user_id).await)
        )
    }

    #[executor]
    pub async fn create_company(&self, actor_user_id: i64) -> Result<bool, AppError> {
        let permission = self.get_permission(tx, actor_user_id).await?;
        let operation = Operation::Create;

        Ok(
            permission.company(Scope::Any(operation))
        )
    }

    #[executor]
    pub async fn create_payroll(&self, actor_user_id: i64, payroll: &CreatePayrollDto) -> Result<bool, AppError> {
        let permission = self.get_permission(tx, actor_user_id).await?;
        let operation = Operation::Create;

        Ok(
            permission.payroll(Scope::Any(operation)) ||
            (permission.payroll(Scope::SelfCompany(operation)) && Self::actor_and_requested_user_same_company(tx, actor_user_id, payroll.user_id).await)
        )
    }

    #[executor]
    pub async fn get_payrolls(&self, actor_user_id: i64, requested_user_id: Option<i64>) -> Result<bool, AppError> {
        let permission = self.get_permission(tx, actor_user_id).await?;
        let operation = Operation::Read;

        Ok(
            permission.payroll(Scope::Any(operation)) ||
            if let Some(user_id) = requested_user_id {
                (permission.payroll(Scope::Owned(operation)) && actor_user_id == user_id) ||
                (permission.payroll(Scope::SelfCompany(operation)) && Self::actor_and_requested_user_same_company(tx, actor_user_id, user_id).await)
            }
            else { false }
        )
    }

    async fn actor_and_created_user_same_company(tx: &mut SqliteConnection, actor_user_id: i64, user: &CreateUserDto) -> bool {
        let user_service = &service::get().user_service;

        let actor_company_id = user_service.get_company_by_user_id_executor(tx, actor_user_id).await;
        if let Ok(Some(actor_company_id)) = actor_company_id {
            return actor_company_id == user.company_id
        }

        false
    }

    async fn actor_and_requested_user_same_company(tx: &mut SqliteConnection, actor_user_id: i64, requested_user_id: i64) -> bool {
        let user_service = &service::get().user_service;

        let actor_company_id = user_service.get_company_by_user_id_executor(tx, actor_user_id).await;
        let requested_company_id = user_service.get_company_by_user_id_executor(tx, requested_user_id).await;

        if let (Ok(Some(actor_company_id)), Ok(Some(requested_company_id))) = (actor_company_id, requested_company_id) {
            return actor_company_id == requested_company_id
        }
        
        false
    }
}
