use serde::Deserialize;
use sqlx::{QueryBuilder, Sqlite};

use crate::{entities::payroll::payroll::Payroll, error::error::AppError};

pub struct PayrollFilterDb {
    pub user_id: Option<i64>,
    pub date: Option<String>
}

impl PayrollFilterDb {
    pub fn from_payroll_filter_dto(filter: PayrollFilterDto) -> Result<PayrollFilterDb, AppError> {
        if let Some(date) = &filter.date {
            Payroll::check_date(date)?;
        }

        Ok(PayrollFilterDb {
            user_id: filter.user_id,
            date: filter.date
        })
    }

    pub fn fill_query(self, query: &mut QueryBuilder<Sqlite>) {
        query.push(" WHERE 1 = 1");

        if let Some(user_id) = self.user_id {
            query.push(" AND user_id = ");
            query.push_bind(user_id);
        }

        if let Some(date) = self.date {
            query.push(" AND date = ");
            query.push_bind(date);
        }
    }
}

#[derive(Deserialize)]
pub struct PayrollFilterDto {
    pub user_id: Option<i64>,
    pub date: Option<String>
}
