use sqlx::{QueryBuilder, SqliteConnection};

use crate::{error::error::AppError, util::db::to_app_error};

use super::{custom_models::payroll_filter::PayrollFilterDb, payroll::{CreatePayrollDb, RetrievePayrollDb, RetrievePayrollDownloadDataDb}};

pub struct PayrollRepository {}

impl PayrollRepository {
    pub fn new() -> PayrollRepository {
        PayrollRepository {

        }
    }

    pub async fn create_payroll(&self, tx: &mut SqliteConnection, payroll: &CreatePayrollDb) -> Result<RetrievePayrollDb, AppError> {
        sqlx::query_as!(
            RetrievePayrollDb,
            r#"
            INSERT INTO Payroll (date, user_id, object_key, filename, content_type, file_size, uploaded_at)
            VALUES($1, $2, $3, $4, $5, $6, $7)
            RETURNING id as "id!: i64", date, user_id, filename, file_size
            "#,
            payroll.date,
            payroll.user_id,
            payroll.object_key,
            payroll.filename,
            payroll.content_type,
            payroll.file_size,
            payroll.uploaded_at
        )
        .fetch_one(tx)
        .await
        .map_err(to_app_error)
    }

    pub async fn get_filtered_payrolls(&self, tx: &mut SqliteConnection, filter: PayrollFilterDb) -> Result<Vec<RetrievePayrollDb>, AppError> {
        let mut query = QueryBuilder::new(
            r#"
            SELECT id, date, user_id, filename, file_size
            FROM Payroll
            "#
        );

        filter.fill_query(&mut query);

        query.build_query_as()
            .fetch_all(tx)
            .await
            .map_err(to_app_error)
    }

    pub async fn get_payroll_by_id(&self, tx: &mut SqliteConnection, payroll_id: i64) -> Result<RetrievePayrollDownloadDataDb, AppError> {
        sqlx::query_as!(
            RetrievePayrollDownloadDataDb,
            r#"
            SELECT object_key, filename, content_type, file_size
            FROM Payroll
            WHERE id = $1
            LIMIT 1
            "#,
            payroll_id
        )
        .fetch_one(tx)
        .await
        .map_err(to_app_error)
    }

    pub async fn get_user_by_payroll_id(&self, tx: &mut SqliteConnection, payroll_id: i64) -> Result<i64, AppError> {
        sqlx::query!(
            r#"
            SELECT user_id
            FROM Payroll
            WHERE id = $1
            LIMIT 1
            "#,
            payroll_id
        )
        .fetch_one(tx)
        .await
        .map(|row| row.user_id)
        .map_err(to_app_error)
    }
}
