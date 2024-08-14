use std::sync::Arc;
use crate::inventory::db;
use crate::inventory::repositories::person::PersonRepository;

pub mod person;

#[derive(Debug)]
pub enum RepoError {
    NotFound(String),
    InvalidUuid(String),
    Other(String),
}

impl From<sqlx::Error> for RepoError {
    fn from(error: sqlx::Error) -> Self {
        match error {
            sqlx::Error::RowNotFound => RepoError::NotFound(error.to_string()),
            _ => RepoError::Other(error.to_string()),
        }
    }
}

async fn init_person_repository() -> Arc<dyn PersonRepository + Send + Sync + 'static> {
    let db = db::db_pool().await;
    Arc::new(person::PersonRepositoryImpl::new(db).await)
}