use macros::DeriveCustomModel;
use serde::{Deserialize, Serialize};

use crate::{config, error::error::{AppError, AppErrorType}};


#[derive(DeriveCustomModel)]
#[custom_model(model(
    name = "CreatePayrollDb",
    fields(date, user_id, object_key, filename, content_type, file_size, uploaded_at)
))]
#[custom_model(model(
    name = "RetrievePayrollDb",
    fields(id, date, user_id, filename, file_size),
))]
#[custom_model(model(
    name = "CreatePayrollDto",
    fields(date, user_id),
    extra_derives(Deserialize, Debug)
))]
#[custom_model(model(
    name = "RetrievePayrollDto",
    fields(id, date, user_id, filename, file_size),
    extra_derives(Serialize)
))]
#[allow(dead_code)]
pub struct Payroll {
    id: i64,
    date: String,
    user_id: i64,
    object_key: String,
    filename: String,
    content_type: String,
    file_size: i64,
    uploaded_at: String
}

impl Payroll {
    pub fn check_date(date: &str) -> Result<(), AppError> {
        // It must be YYYY-MM
        let rx = regex::Regex::new(r"^\d{4}-\d{2}$").unwrap();
        if !rx.is_match(date) {
            return Err(AppError::new(
                format!("Invalid date format: {}", date),
                AppErrorType::BadRequest,
                None
            ));
        }
        

        Ok(())
    }

    pub fn check_object_key(object_key: &str) -> Result<(), AppError> {
        if !(object_key.len() > 0 && object_key.len() <= 255) {
            return Err(AppError::new(
                String::from("The object key must be between 1 and 255 characters long"),
                AppErrorType::BadRequest,
                None
            ));
        }

        Ok(())
    }

    pub fn check_filename(filename: &str) -> Result<(), AppError> {
        if !(filename.len() > 0 && filename.len() <= 255) {
            return Err(AppError::new(
                String::from("The filename must be between 1 and 255 characters long"),
                AppErrorType::BadRequest,
                None
            ));
        }

        Ok(())
    }

    pub fn check_content_type(content_type: &str) -> Result<(), AppError> {
        if content_type != "application/pdf" {
            return Err(AppError::new(
                format!("Invalid content type: {}", content_type),
                AppErrorType::BadRequest,
                None
            ));
        }
        
        Ok(())
    }

    pub fn check_file_size(file_size: i64) -> Result<(), AppError> {
        if file_size <= 0 || (file_size as u64) > config::get().file.max_size {
            return Err(AppError::new(
                format!("Invalid file size: {}", file_size),
                AppErrorType::BadRequest,
                None
            ));
        }

        Ok(())
    }

    pub fn check_uploaded_at(_uploaded_at: &str) -> Result<(), AppError> {
        // let rx = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z$").unwrap();
        // rx.is_match(uploaded_at)
        
        Ok(())
    }
}

impl CreatePayrollDb {
    pub fn from_create_payroll_dto(
        dto: CreatePayrollDto,
        object_key: String, filename:
        String, content_type: String,
        file_size: i64,
        uploaded_at: String
    ) -> Result<CreatePayrollDb, AppError>
    {
        Payroll::check_date(&dto.date)?;
        Payroll::check_object_key(&object_key)?;
        Payroll::check_filename(&filename)?;
        Payroll::check_content_type(&content_type)?;
        Payroll::check_file_size(file_size)?;
        Payroll::check_uploaded_at(&uploaded_at)?;

        Ok(CreatePayrollDb {
            date: dto.date,
            user_id: dto.user_id,
            object_key,
            filename,
            content_type,
            file_size,
            uploaded_at
        })
    }
}

impl RetrievePayrollDb {
    pub fn to_retrieve_payroll_dto(self) -> RetrievePayrollDto {
        RetrievePayrollDto {
            id: self.id,
            date: self.date,
            user_id: self.user_id,
            filename: self.filename,
            file_size: self.file_size
        }
    }
}
