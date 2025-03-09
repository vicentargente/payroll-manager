use std::sync::Arc;

use actix_web::web;
use macros::executor;
use sqlx::{SqliteConnection, SqlitePool};

use crate::{config, error::error::{AppError, AppErrorType}, util::{file::{check_pdf, remove_file}, minio::MinioService}};

use super::{custom_models::{download_payroll::DownloadPayrollDto, payroll_filter::{PayrollFilterDb, PayrollFilterDto}}, payroll::{CreatePayrollDb, CreatePayrollDto, RetrievePayrollDto}, payroll_repository::PayrollRepository};

pub struct PayrollService {
    db_pool: SqlitePool,
    payroll_repository: PayrollRepository,
    bucket_service: Arc<MinioService>
}

impl PayrollService {
    pub fn new(db_pool: SqlitePool , payroll_repository: PayrollRepository, bucket_service: Arc<MinioService>) -> PayrollService {
        PayrollService {
            db_pool,
            payroll_repository,
            bucket_service
        }
    }

    #[executor]
    pub async fn create_payroll(&self, payroll: CreatePayrollDto, file_path: &str, file_name: &str, original_file_name: &str, file_size: i64) -> Result<RetrievePayrollDto, AppError> {
        let result = self.do_create_payroll(tx, payroll, file_path, file_name, original_file_name, file_size).await;
        remove_file(file_path).await?;
        result
    }

    #[executor]
    pub async fn get_filtered_payrolls(&self, filter: PayrollFilterDto) -> Result<Vec<RetrievePayrollDto>, AppError> {
        let payrolls = self.payroll_repository.get_filtered_payrolls(tx, PayrollFilterDb::from_payroll_filter_dto(filter)?).await?;

        Ok(payrolls.into_iter().map(|payroll| payroll.to_retrieve_payroll_dto()).collect())
    }

    #[executor]
    pub async fn download_payroll(&self, payroll_id: i64) -> Result<DownloadPayrollDto, AppError> {
        let payroll_data = self.payroll_repository.get_payroll_by_id(tx, payroll_id).await?;

        let bucket_name = &config::get().bucket.payroll_base_bucket_name;
        let stream_info = self.bucket_service.get_file_stream(bucket_name, &payroll_data.object_key).await?;

        if stream_info.size != payroll_data.file_size {
            return Err(AppError::new(
                format!("File size mismatch: expected {}, got {}", payroll_data.file_size, stream_info.size),
                AppErrorType::InternalServerError,
                None
            ));
        }

        Ok(DownloadPayrollDto {
            filename: payroll_data.filename,
            content_type: payroll_data.content_type,
            file_size: payroll_data.file_size,
            stream: stream_info.stream
        })
    }

    #[executor]
    pub async fn get_user_by_payroll_id(&self, payroll_id: i64) -> Result<i64, AppError> {
        self.payroll_repository.get_user_by_payroll_id(tx, payroll_id).await
    }

    async fn do_create_payroll(
        &self,
        tx: &mut SqliteConnection,
        payroll: CreatePayrollDto,
        file_path: &str,
        file_name: &str,
        original_file_name: &str,
        file_size: i64
    ) -> Result<RetrievePayrollDto, AppError>
    {
        if &original_file_name[original_file_name.len() - 4..] != ".pdf" {
            return Err(AppError::new(
                format!("Invalid file type: {}", original_file_name),
                AppErrorType::BadRequest,
                None
            ));
        }

        // check if file is pdf
        {
            let file_path = file_path.to_string();
            web::block(move || {
                check_pdf(&file_path)
            })
            .await
            .map_err(|err| AppError::new(
                format!("Failed to check pdf: {}", err),
                AppErrorType::BadRequest,
                None
            ))?
        }?;


        let create_payroll_db = CreatePayrollDb::from_create_payroll_dto(
            payroll,
            file_name.to_string(),
            original_file_name.to_string(),
            String::from("application/pdf"),
            file_size as i64,
            chrono::Utc::now().naive_utc().to_string()
        )?;

        let bucket_name = &config::get().bucket.payroll_base_bucket_name;
        self.bucket_service.upload_file(bucket_name, file_path, file_name).await?;

        let created_payroll = match self.payroll_repository.create_payroll(tx, &create_payroll_db).await {
            Ok(payroll) => payroll,
            Err(e) => {
                self.bucket_service.remove_file(bucket_name, file_name).await?;
                return Err(e);
            },
        };
        

        Ok(created_payroll.to_retrieve_payroll_dto())
    }
}
