use macros::DeriveCustomModel;
use serde::{Deserialize, Serialize};

use crate::error::error::{AppError, AppErrorType};

#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "RetrieveCompanyDb",
    fields(id, name)
))]
#[custom_model(model(
    name = "CreateCompanyDb",
    fields(name)
))]
#[custom_model(model(
    name = "RetrieveCompanyDto",
    fields(id, name),
    extra_derives(Serialize)
))]
#[custom_model(model(
    name = "CreateCompanyDto",
    fields(name),
    extra_derives(Deserialize)
))]
#[allow(dead_code)]
pub struct Company {
    id: i64,
    name: String
}

impl Company {
    pub fn new(id: i64, name: String) -> Company {
        Company {
            id,
            name
        }
    }

    pub fn check_name(name: &str) -> Result<(), AppError> {
        if name.len() == 0 || name.len() > 50 {
            return Err(AppError::new(
                String::from("The company name must be between 1 and 50 characters long"),
                AppErrorType::BadRequest,
                None
            ));
        }

        Ok(())
    }
}

impl RetrieveCompanyDb {
    pub fn to_retrieve_company_dto(self) -> Result<RetrieveCompanyDto, AppError> {
        Ok(RetrieveCompanyDto {
            id: self.id,
            name: self.name
        })
    }
}

impl CreateCompanyDb {
    pub fn from_create_company_dto(company: CreateCompanyDto) -> Result<CreateCompanyDb, AppError> {
        Company::check_name(&company.name)?;

        Ok(CreateCompanyDb {
            name: company.name
        })
    }
}