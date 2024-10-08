use crate::inventory::model::DeleteResults;
use sqlx::postgres::PgQueryResult;

pub mod invoice;
pub mod item;
pub mod person;

#[derive(Debug)]
pub enum RepoError {
    NotFound(String),
    InvalidUuid(String),
    Other(String),
    UniqueViolation(String),
}

impl From<sqlx::Error> for RepoError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => RepoError::NotFound(error.to_string()),
            sqlx::Error::Database(err) => {
                if err.is_unique_violation() {
                    RepoError::UniqueViolation(err.to_string())
                } else {
                    RepoError::Other(err.to_string())
                }
            }
            _ => RepoError::Other(error.to_string()),
        }
    }
}

impl From<PgQueryResult> for DeleteResults {
    fn from(result: PgQueryResult) -> Self {
        DeleteResults {
            id: String::new(),
            deleted: result.rows_affected() > 0,
        }
    }
}
