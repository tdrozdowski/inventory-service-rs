use crate::inventory::db;
use crate::inventory::repositories::person::PersonRepository;
use std::sync::Arc;

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