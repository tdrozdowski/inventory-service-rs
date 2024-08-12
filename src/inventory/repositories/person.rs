use async_trait::async_trait;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;
use crate::inventory::repositories::RepoError;

#[derive(sqlx::FromRow)]
struct PersonRow {
    id: i64,
    alt_id: Uuid,
    name: String,
    email: String,
    created_by: String,
    created_at: DateTime<Utc>,
    last_update: DateTime<Utc>,
}

#[async_trait]
pub trait PersonRepository {
    async fn get_all_persons(&self, last_id: Option<i32>, page_size: i64) -> Result<Vec<PersonRow>, RepoError>;
    async fn get_person_by_id(&self, id: Uuid) -> Result<PersonRow, RepoError>;
    async fn get_person_by_uuid(&self, id: Uuid) -> Result<PersonRow, RepoError>;
    async fn create_person(&self, person: &PersonRow) -> Result<PersonRow, RepoError>;
    async fn update_person(&self, person: &PersonRow) -> Result<PersonRow, RepoError>;
    async fn delete_person(&self, id: Uuid) -> Result<PersonRow, RepoError>;
}

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
    async fn get_all_persons(&self, last_id: Option<i32>, page_size: i64) -> Result<Vec<PersonRow>, RepoError> {
        let result = if let Some(id) = last_id {
            sqlx::query_as!(
                PersonRow,
                r#"
                    SELECT id, alt_id, name, email, created_by, created_at, last_update
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
                    SELECT id, alt_id, name, email, created_by, created_at, last_update
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


    async fn get_person_by_id(&self, id: Uuid) -> Result<PersonRow, RepoError> {
        todo!()
    }

    async fn get_person_by_uuid(&self, id: Uuid) -> Result<PersonRow, RepoError> {
        todo!()
    }

    async fn create_person(&self, person: &PersonRow) -> Result<PersonRow, RepoError> {
        todo!()
    }

    async fn update_person(&self, person: &PersonRow) -> Result<PersonRow, RepoError> {
        todo!()
    }

    async fn delete_person(&self, id: Uuid) -> Result<PersonRow, RepoError> {
        todo!()
    }
}