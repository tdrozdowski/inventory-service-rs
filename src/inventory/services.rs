use crate::inventory::repositories::RepoError;
use garde::Report;

pub mod person;

#[derive(Debug)]
pub enum ServiceError {
    NotFound(String),
    InvalidUuid(String),
    UnexpectedError(String),
    UniqueViolation(String),
    InputValidationError(String),
}

impl From<RepoError> for ServiceError {
    fn from(error: RepoError) -> Self {
        match error {
            RepoError::NotFound(err) => ServiceError::NotFound(err),
            RepoError::InvalidUuid(err) => ServiceError::InvalidUuid(err),
            RepoError::Other(err) => ServiceError::UnexpectedError(err),
            RepoError::UniqueViolation(err) => ServiceError::UniqueViolation(err),
        }
    }
}

impl From<Report> for ServiceError {
    fn from(error: Report) -> Self {
        ServiceError::InputValidationError(error.to_string())
    }
}
