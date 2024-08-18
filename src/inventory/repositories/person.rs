use crate::inventory::model::{CreatePersonRequest, UpdatePersonRequest};
use crate::inventory::repositories::RepoError;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use mockall::automock;
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::fmt::Debug;

#[derive(sqlx::FromRow, Debug, Clone)]
pub struct PersonRow {
    pub id: i32,
    pub alt_id: Uuid,
    pub name: String,
    pub email: String,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub last_changed_by: String,
    pub last_update: DateTime<Utc>,
}

#[async_trait]
#[automock]
pub trait PersonRepository: Debug {
    async fn get_all_persons(
        &self,
        last_id: Option<i32>,
        page_size: i64,
    ) -> Result<Vec<PersonRow>, RepoError>;
    async fn get_person_by_id(&self, id: i32) -> Result<PersonRow, RepoError>;
    async fn get_person_by_uuid(&self, id: Uuid) -> Result<PersonRow, RepoError>;
    async fn create_person(&self, person: &CreatePersonRequest) -> Result<PersonRow, RepoError>;
    async fn update_person(&self, person: &UpdatePersonRequest) -> Result<PersonRow, RepoError>;
    async fn delete_person(&self, id: Uuid) -> Result<PersonRow, RepoError>;
}

#[derive(Debug)]
pub struct PersonRepositoryImpl {
    pub db: PgPool,
}

impl PersonRepositoryImpl {
    pub async fn new(db: PgPool) -> PersonRepositoryImpl {
        PersonRepositoryImpl { db }
    }
}

#[async_trait]
impl PersonRepository for PersonRepositoryImpl {
    async fn get_all_persons(
        &self,
        last_id: Option<i32>,
        page_size: i64,
    ) -> Result<Vec<PersonRow>, RepoError> {
        let result = if let Some(id) = last_id {
            sqlx::query_as!(
                PersonRow,
                r#"
                    SELECT id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                    FROM persons
                    WHERE id > $1
                    ORDER BY id
                    LIMIT $2
                    "#,
                id,
                page_size
            )
                .fetch_all(&self.db)
                .await
        } else {
            sqlx::query_as!(
                PersonRow,
                r#"
                    SELECT id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                    FROM persons
                    ORDER BY id
                    LIMIT $1
                    "#,
                page_size
            )
                .fetch_all(&self.db)
                .await
        };

        match result {
            Ok(rows) => Ok(rows),
            Err(e) => Err(RepoError::Other(e.to_string())),
        }
    }

    async fn get_person_by_id(&self, id: i32) -> Result<PersonRow, RepoError> {
        let result = sqlx::query_as!(
            PersonRow,
            r#"
                SELECT id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                FROM persons
                WHERE id = $1
                "#,
            id
        )
        .fetch_one(&self.db)
        .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => Err(RepoError::from(e)),
        }
    }

    async fn get_person_by_uuid(&self, id: Uuid) -> Result<PersonRow, RepoError> {
        let result = sqlx::query_as!(
            PersonRow,
            r#"
                SELECT id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                FROM persons
                WHERE alt_id = $1
                "#,
            id
        )
        .fetch_one(&self.db)
        .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => Err(RepoError::from(e)),
        }
    }

    async fn create_person(&self, person: &CreatePersonRequest) -> Result<PersonRow, RepoError> {
        let result = sqlx::query_as!(
            PersonRow,
            r#"
                INSERT INTO persons (name, email, created_by)
                VALUES ($1, $2, $3)
                RETURNING id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                "#,
            person.name,
            person.email,
            person.created_by
        )
            .fetch_one(&self.db)
            .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => Err(RepoError::from(e)),
        }
    }

    async fn update_person(&self, person: &UpdatePersonRequest) -> Result<PersonRow, RepoError> {
        if let Ok(uuid) = Uuid::parse_str(&person.id) {
            let result = sqlx::query_as!(
                PersonRow,
                r#"
                    UPDATE persons
                    SET name = $1, email = $2, last_changed_by = $3, last_update = $4
                    WHERE alt_id = $5
                    RETURNING id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                    "#,
                person.name,
                person.email,
                person.changed_by,
                Utc::now(),
                uuid
            )
                .fetch_one(&self.db)
                .await;

            match result {
                Ok(row) => Ok(row),
                Err(e) => Err(RepoError::from(e)),
            }
        } else {
            Err(RepoError::InvalidUuid(person.id.clone()))
        }
    }

    async fn delete_person(&self, id: Uuid) -> Result<PersonRow, RepoError> {
        let result = sqlx::query_as!(
            PersonRow,
            r#"
                DELETE FROM persons
                WHERE alt_id = $1
                RETURNING id, alt_id, name, email, created_by, created_at, last_changed_by, last_update
                "#,
            id
        )
            .fetch_one(&self.db)
            .await;

        match result {
            Ok(row) => Ok(row),
            Err(e) => Err(RepoError::from(e)),
        }
    }
}
