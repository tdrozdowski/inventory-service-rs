use crate::inventory::model::{CreateItemRequest, Pagination, UpdateItemRequest};
use crate::inventory::repositories::RepoError;
use async_trait::async_trait;
use bigdecimal::FromPrimitive;
use chrono::{DateTime, Utc};
use mockall::automock;
use sqlx::types::BigDecimal;
use sqlx::{FromRow, PgPool};
use std::fmt::Debug;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct ItemRow {
    pub id: i32,
    pub alt_id: Uuid,
    pub name: String,
    pub description: String,
    pub unit_price: BigDecimal,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub last_changed_by: String,
    pub last_update: DateTime<Utc>,
}

#[async_trait]
#[automock]
pub trait ItemRepository: Debug {
    async fn get_all_items(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<ItemRow>, RepoError>;
    async fn get_item_by_id(&self, id: i32) -> Result<ItemRow, RepoError>;
    async fn get_item_by_uuid(&self, id: Uuid) -> Result<ItemRow, RepoError>;
    async fn create_item(&self, item: &CreateItemRequest) -> Result<ItemRow, RepoError>;
    async fn update_item(&self, item: &UpdateItemRequest) -> Result<ItemRow, RepoError>;
    async fn delete_item(&self, id: Uuid) -> Result<ItemRow, RepoError>;
}

#[derive(Debug)]
pub struct ItemRepositoryImpl {
    pub db: PgPool,
}

impl ItemRepositoryImpl {
    pub async fn new(db: PgPool) -> ItemRepositoryImpl {
        ItemRepositoryImpl { db }
    }
}

#[async_trait]
impl ItemRepository for ItemRepositoryImpl {
    async fn get_all_items(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<ItemRow>, RepoError> {
        let result = if let Some(pagination) = maybe_pagination {
            if let Some(last_id) = pagination.last_id {
                sqlx::query_as!(
                    ItemRow,
                    r#"
                        SELECT id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
                        FROM items
                        WHERE id > $1
                        ORDER BY id
                        LIMIT $2
                    "#,
                    last_id,
                    pagination.page_size,
                )
                    .fetch_all(&self.db)
                    .await
            } else {
                sqlx::query_as!(
                    ItemRow,
                    r#"
                        SELECT id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
                        FROM items
                        ORDER BY id
                        LIMIT $1
                    "#,
                    pagination.page_size,
                )
                    .fetch_all(&self.db)
                    .await
            }
        } else {
            sqlx::query_as!(
                ItemRow,
                r#"
                    SELECT id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
                    FROM items
                    ORDER BY id
                "#
            )
                .fetch_all(&self.db)
                .await
        };
        result.map_err(RepoError::from)
    }

    async fn get_item_by_id(&self, id: i32) -> Result<ItemRow, RepoError> {
        let result = sqlx::query_as!(
            ItemRow,
            r#"
                SELECT id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
                FROM items
                WHERE id = $1
            "#,
            id,
        )
            .fetch_one(&self.db)
            .await;
        result.map_err(RepoError::from)
    }

    async fn get_item_by_uuid(&self, id: Uuid) -> Result<ItemRow, RepoError> {
        let result = sqlx::query_as!(
            ItemRow,
            r#"
                SELECT id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
                FROM items
                WHERE alt_id = $1
            "#,
            id,
        )
            .fetch_one(&self.db)
            .await;
        result.map_err(RepoError::from)
    }

    async fn create_item(&self, item: &CreateItemRequest) -> Result<ItemRow, RepoError> {
        let result = sqlx::query_as!(
            ItemRow,
            r#"
                INSERT INTO items (alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
            "#,
            Uuid::new_v4(),
            item.name,
            item.description,
            BigDecimal::from_f64(item.unit_price),
            item.created_by,
            Utc::now(),
            item.created_by,
            Utc::now(),
        )
            .fetch_one(&self.db)
            .await;
        result.map_err(RepoError::from)
    }

    async fn update_item(&self, item: &UpdateItemRequest) -> Result<ItemRow, RepoError> {
        if let Ok(uuid) = Uuid::parse_str(&item.id) {
            let result = sqlx::query_as!(
                ItemRow,
                r#"
                    UPDATE items
                    SET name = $1, description = $2, unit_price = $3, last_changed_by = $4, last_update = $5
                    WHERE alt_id = $6
                    RETURNING id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
                "#,
                item.name,
                item.description,
                BigDecimal::from_f64(item.unit_price),
                item.changed_by,
                Utc::now(),
                uuid,
            )
                .fetch_one(&self.db)
                .await;
            result.map_err(RepoError::from)
        } else {
            return Err(RepoError::InvalidUuid(item.clone().id));
        }
    }

    async fn delete_item(&self, id: Uuid) -> Result<ItemRow, RepoError> {
        let result = sqlx::query_as!(
            ItemRow,
            r#"
                DELETE FROM items
                WHERE alt_id = $1
                RETURNING id, alt_id, name, description, unit_price, created_by, created_at, last_changed_by, last_update
            "#,
            id,
        )
            .fetch_one(&self.db)
            .await;
        result.map_err(RepoError::from)
    }
}
