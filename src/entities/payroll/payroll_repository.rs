use sqlx::SqliteConnection;

use crate::{error::error::AppError, util::db::to_app_error};

use super::payroll::{CreatePayrollDb, RetrievePayrollDb};

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
}
