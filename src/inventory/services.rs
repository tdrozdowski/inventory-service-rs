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

#[cfg(test)]
mod tests {
    use crate::inventory::repositories::RepoError;
    use crate::inventory::services::ServiceError;

    #[test]
    fn test_from_repo_error() {
        let repo_error = RepoError::NotFound("Not found".to_string());
        let service_error = ServiceError::from(repo_error);
        match service_error {
            ServiceError::NotFound(err) => assert_eq!(err, "Not found"),
            _ => panic!("Expected NotFound"),
        }
        let repo_error = RepoError::InvalidUuid("Invalid uuid".to_string());
        let service_error = ServiceError::from(repo_error);
        match service_error {
            ServiceError::InvalidUuid(err) => assert_eq!(err, "Invalid uuid"),
            _ => panic!("Expected InvalidUuid"),
        }
        let repo_error = RepoError::Other("Other error".to_string());
        let service_error = ServiceError::from(repo_error);
        match service_error {
            ServiceError::UnexpectedError(err) => assert_eq!(err, "Other error"),
            _ => panic!("Expected UnexpectedError"),
        }
        let repo_error = RepoError::UniqueViolation("Unique violation".to_string());
        let service_error = ServiceError::from(repo_error);
        match service_error {
            ServiceError::UniqueViolation(err) => assert_eq!(err, "Unique violation"),
            _ => panic!("Expected UniqueViolation"),
        }
    }
}
