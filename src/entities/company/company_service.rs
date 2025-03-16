use macros::executor;
use sqlx::{SqliteConnection, SqlitePool};

use crate::error::error::AppError;

use super::{company::{CreateCompanyDb, CreateCompanyDto, RetrieveCompanyDto}, company_repository::CompanyRepository, custom_models::company_filter::{CompanyFilterDb, CompanyFilterDto}};

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

    #[executor]
    pub async fn get_companies(&self, filter: CompanyFilterDto) -> Result<Vec<RetrieveCompanyDto>, AppError> {
        let companies = self.company_repository.get_companies(tx, CompanyFilterDb::from_company_filter_dto(filter)?).await?;

        Ok(
            companies
                .into_iter()
                .map(|company| company.to_retrieve_company_dto())
                .collect::<Result<_, _>>()?
        )
    }
}