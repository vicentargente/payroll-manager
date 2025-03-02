use crate::{error::error::AppError, util::db::to_app_error};

use super::company::{CreateCompanyDb, RetrieveCompanyDb};

pub struct CompanyRepository {

}

impl CompanyRepository {
    pub fn new() -> CompanyRepository {
        CompanyRepository {
            
        }
    }

    pub async fn create_company(&self, tx: &mut sqlx::SqliteConnection, company: &CreateCompanyDb) -> Result<RetrieveCompanyDb, AppError> {
        sqlx::query_as!(
            RetrieveCompanyDb,
            r#"
            INSERT INTO Company (name)
            VALUES ($1)
            RETURNING id, name 
            "#,
            company.name
        )
        .fetch_one(tx)
        .await
        .map_err(to_app_error)
    }

    pub async fn company_exists_by_id(&self, tx: &mut sqlx::SqliteConnection, company_id: i64) -> Result<bool, AppError> {
        sqlx::query_scalar!(
            r#"
            SELECT 1
            FROM Company
            WHERE id = $1
            LIMIT 1
            "#,
            company_id
        )
        .fetch_optional(tx)
        .await
        .map(|r| r.is_some())
        .map_err(to_app_error)
    }
}
