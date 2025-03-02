use macros::executor;
use sqlx::{SqliteConnection, SqlitePool};

use crate::error::error::AppError;

use super::{company::{CreateCompanyDb, CreateCompanyDto, RetrieveCompanyDto}, company_repository::CompanyRepository};

pub struct CompanyService {
    db_pool: SqlitePool,
    company_repository: CompanyRepository
}

impl CompanyService {
    pub fn new(db_pool: SqlitePool, company_repository: CompanyRepository) -> CompanyService {
        CompanyService {
            db_pool,
            company_repository
        }
    }

    #[executor]
    pub async fn create_company(&self, company: CreateCompanyDto) -> Result<RetrieveCompanyDto, AppError> {
        let company_db = CreateCompanyDb::from_create_company_dto(company)?;

        Ok(self.company_repository.create_company(tx, &company_db).await?.to_retrieve_company_dto()?)
    }

    #[executor]
    pub async fn company_exists_by_id(&self, company_id: i64) -> Result<bool, AppError> {
        Ok(self.company_repository.company_exists_by_id(tx, company_id).await?)
    }
}