use serde::Deserialize;

use crate::error::error::{AppError, AppErrorType};

pub struct CompanyFilterDb {
    pub limit: i64,
    pub offset: i64
}

impl CompanyFilterDb {
    pub fn check_limit(limit: i64) -> Result<(), AppError> {
        if limit < 1 || limit > 25 {
            return Err(AppError::new(
                String::from(r#"Limit must be between 1 and 25"#),
                AppErrorType::BadRequest,
                None
            ));
        }

        Ok(())
    }

    pub fn check_offset(offset: i64) -> Result<(), AppError> {
        if offset < 0 {
            return Err(AppError::new(
                String::from(r#"Offset must be greater than or equal to 0"#),
                AppErrorType::BadRequest,
                None
            ));
        }

        Ok(())
    }

    pub fn from_company_filter_dto(filter: CompanyFilterDto) -> Result<CompanyFilterDb, AppError> {
        Self::check_limit(filter.limit)?;
        Self::check_offset(filter.offset)?;
        
        Ok(CompanyFilterDb {
            limit: filter.limit,
            offset: filter.offset
        })
    }
}

#[derive(Deserialize)]
pub struct CompanyFilterDto {
    pub limit: i64,
    pub offset: i64
}