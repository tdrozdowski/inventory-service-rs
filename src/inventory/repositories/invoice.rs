use crate::inventory::model::{
    CreateInvoiceRequest, DeleteResults, Pagination, UpdateInvoiceRequest,
};
use crate::inventory::repositories::RepoError;
use async_trait::async_trait;
use bigdecimal::{BigDecimal, FromPrimitive};
use chrono::{DateTime, Utc};
use mockall::automock;
use sqlx::FromRow;
use std::fmt::Debug;
use tracing::instrument;
use uuid::Uuid;

#[derive(Clone, Debug, FromRow)]
pub struct InvoiceRow {
    pub id: i32,
    pub alt_id: Uuid,
    pub user_id: Uuid,
    pub total: BigDecimal,
    pub paid: bool,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub last_changed_by: String,
    pub last_update: DateTime<Utc>,
}

#[derive(Clone, Debug, FromRow)]
pub struct InvoiceItemRow {
    pub invoice_id: Uuid,
    pub item_id: Uuid,
}

#[async_trait]
#[automock]
pub trait InvoiceRepository: Debug {
    async fn create(&self, invoice: CreateInvoiceRequest) -> Result<InvoiceRow, RepoError>;
    async fn get_all_invoices(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<InvoiceRow>, RepoError>;
    async fn get_by_id(&self, id: i32) -> Result<InvoiceRow, RepoError>;
    async fn get_by_uuid(&self, alt_id: Uuid) -> Result<InvoiceRow, RepoError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<InvoiceRow>, RepoError>;
    async fn update(&self, invoice: UpdateInvoiceRequest) -> Result<InvoiceRow, RepoError>;
    async fn delete(&self, id: Uuid) -> Result<DeleteResults, RepoError>;
    async fn add_item(&self, invoice_item: InvoiceItemRow) -> Result<InvoiceItemRow, RepoError>;
    async fn remove_item(&self, invoice_item: InvoiceItemRow) -> Result<DeleteResults, RepoError>;
    async fn get_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceItemRow>, RepoError>;
}

#[derive(Debug)]
pub struct InvoiceRepositoryImpl {
    pool: sqlx::PgPool,
}

impl InvoiceRepositoryImpl {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl InvoiceRepository for InvoiceRepositoryImpl {
    #[instrument]
    async fn create(&self, invoice: CreateInvoiceRequest) -> Result<InvoiceRow, RepoError> {
        let total = BigDecimal::from_f64(invoice.total).unwrap();
        let now = Utc::now();
        // TODO - add items to invoice
        let result = sqlx::query_as!(
            InvoiceRow,
            r#"
            INSERT INTO invoices (user_id, total, paid, created_by, created_at, last_changed_by, last_update)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            RETURNING id, alt_id, total, paid, created_by, created_at, last_changed_by, last_update, user_id
            "#,
            invoice.user_id,
            total,
            invoice.paid,
            invoice.created_by,
            now,
            invoice.created_by,
            now
        )
            .fetch_one(&self.pool)
            .await;

        result.map_err(RepoError::from)
    }

    #[instrument]
    async fn get_all_invoices(
        &self,
        maybe_pagination: Option<Pagination>,
    ) -> Result<Vec<InvoiceRow>, RepoError> {
        let result = if let Some(pagination) = maybe_pagination {
            if let Some(last_id) = pagination.last_id {
                sqlx::query_as!(
                    InvoiceRow,
                    r#"
                    SELECT id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
                    FROM invoices
                    WHERE id > $1
                    ORDER BY id ASC
                    LIMIT $2
                    "#,
                    last_id,
                    pagination.page_size
                )
                    .fetch_all(&self.pool)
                    .await
            } else {
                sqlx::query_as!(
                    InvoiceRow,
                    r#"
                    SELECT id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
                    FROM invoices
                    ORDER BY id ASC
                    LIMIT $1
                    "#,
                    pagination.page_size
                )
                    .fetch_all(&self.pool)
                    .await
            }
        } else {
            sqlx::query_as!(
                InvoiceRow,
                r#"
                SELECT id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
                FROM invoices
                ORDER BY id ASC
                LIMIT 10
                "#,
            )
                .fetch_all(&self.pool)
                .await
        };
        result.map_err(RepoError::from)
    }

    #[instrument]
    async fn get_by_id(&self, id: i32) -> Result<InvoiceRow, RepoError> {
        let result = sqlx::query_as!(
            InvoiceRow,
            r#"
            SELECT id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
            FROM invoices
            WHERE id = $1
            "#,
            id
        )
            .fetch_one(&self.pool)
            .await;

        result.map_err(RepoError::from)
    }

    #[instrument]
    async fn get_by_uuid(&self, alt_id: Uuid) -> Result<InvoiceRow, RepoError> {
        let result = sqlx::query_as!(
            InvoiceRow,
            r#"
            SELECT id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
            FROM invoices
            WHERE alt_id = $1
            "#,
            alt_id
        )
            .fetch_one(&self.pool)
            .await;
        result.map_err(RepoError::from)
    }

    #[instrument]
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<InvoiceRow>, RepoError> {
        let result = sqlx::query_as!(
            InvoiceRow,
            r#"
            SELECT id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
            FROM invoices
            WHERE user_id = $1
            "#,
            user_id
        )
            .fetch_all(&self.pool)
            .await;
        result.map_err(RepoError::from)
    }

    #[instrument]
    async fn update(&self, invoice: UpdateInvoiceRequest) -> Result<InvoiceRow, RepoError> {
        let total = BigDecimal::from_f64(invoice.total).unwrap();
        let row = sqlx::query_as!(
            InvoiceRow,
            r#"
            UPDATE invoices
            SET total = $1, last_changed_by = $2, last_update = now()
            WHERE alt_id = $3
            RETURNING id, alt_id, user_id, total, paid, created_by, created_at, last_changed_by, last_update
            "#,
            total,
            invoice.changed_by,
            invoice.id
        )
            .fetch_one(&self.pool)
            .await;
        row.map_err(RepoError::from)
    }

    #[instrument]
    async fn delete(&self, id: Uuid) -> Result<DeleteResults, RepoError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM invoices
            WHERE alt_id = $1
            "#,
            id
        )
        .execute(&self.pool)
        .await;
        result
            .map(DeleteResults::from)
            .map(|mut r| {
                r.id = id.to_string();
                r
            })
            .map_err(RepoError::from)
    }

    #[instrument]
    async fn add_item(&self, invoice_item: InvoiceItemRow) -> Result<InvoiceItemRow, RepoError> {
        let row = sqlx::query_as!(
            InvoiceItemRow,
            r#"
            INSERT INTO invoices_items (invoice_id, item_id)
            VALUES ($1, $2)
            RETURNING invoice_id, item_id
            "#,
            invoice_item.invoice_id,
            invoice_item.item_id
        )
        .fetch_one(&self.pool)
        .await;
        row.map_err(RepoError::from)
    }

    #[instrument]
    async fn remove_item(&self, invoice_item: InvoiceItemRow) -> Result<DeleteResults, RepoError> {
        let result = sqlx::query!(
            r#"
            DELETE FROM invoices_items
            WHERE invoice_id = $1 AND item_id = $2
            "#,
            invoice_item.invoice_id,
            invoice_item.item_id
        )
        .execute(&self.pool)
        .await;
        result
            .map(DeleteResults::from)
            .map(|mut r| {
                r.id = invoice_item.item_id.to_string();
                r
            })
            .map_err(RepoError::from)
    }

    #[instrument]
    async fn get_items(&self, invoice_id: Uuid) -> Result<Vec<InvoiceItemRow>, RepoError> {
        let result = sqlx::query_as!(
            InvoiceItemRow,
            r#"
            SELECT invoice_id, item_id
            FROM invoices_items
            WHERE invoice_id = $1
            "#,
            invoice_id
        )
        .fetch_all(&self.pool)
        .await;
        result.map_err(RepoError::from)
    }
}