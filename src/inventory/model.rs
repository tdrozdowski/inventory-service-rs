use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, Deserialize, IntoParams, Clone, Copy, PartialEq, Serialize, ToSchema)]
pub struct Pagination {
    pub last_id: Option<i32>,
    pub page_size: i64,
}

impl Default for Pagination {
    fn default() -> Self {
        Pagination {
            last_id: None,
            page_size: 10,
        }
    }
}
#[derive(
    Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Validate, ToSchema,
)]
pub struct CreatePersonRequest {
    #[garde(length(min = 3, max = 50))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub created_by: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Validate, ToSchema)]
pub struct UpdatePersonRequest {
    #[garde(skip)] // TODO - improve this
    pub id: String,
    #[garde(length(min = 3, max = 50))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub changed_by: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, ToSchema)]
pub struct AuditInfo {
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub changed_by: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, ToSchema)]
pub struct Person {
    pub seq: i32,
    pub id: String,
    pub name: String,
    pub email: String,
    pub audit_info: AuditInfo,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, ToSchema)]
pub struct ApiError {
    pub status_code: i32,
    pub message: String,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, ToSchema)]
pub struct DeleteResults {
    pub id: String,
    pub deleted: bool,
}

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, ToSchema)]
pub struct PersonList {
    pub persons: Vec<Person>,
    pub total: i32,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateItemRequest {
    #[garde(length(min = 3, max = 255))]
    pub name: String,
    #[garde(skip)]
    pub description: String,
    #[garde(range(min = 0.0, max = 1000000.0))]
    pub unit_price: f64,
    #[garde(skip)]
    pub created_by: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateItemRequest {
    #[garde(skip)]
    pub id: String,
    #[garde(length(min = 3, max = 255))]
    pub name: String,
    #[garde(skip)]
    pub description: String,
    #[garde(range(min = 0.0, max = 1000000.0))]
    pub unit_price: f64,
    #[garde(skip)]
    pub changed_by: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct Item {
    #[garde(range(min = 1))]
    pub seq: i32,
    #[garde(skip)]
    pub id: String,
    #[garde(length(min = 3, max = 255))]
    pub name: String,
    #[garde(skip)]
    pub description: String,
    #[garde(range(min = 0.0, max = 1000000.0))]
    pub unit_price: f64,
    #[garde(skip)]
    pub audit_info: AuditInfo,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateInvoiceRequest {
    #[garde(skip)]
    pub user_id: Uuid,
    #[garde(range(min = 0.0, max = 1000000.0))]
    pub total: f64,
    #[garde(skip)]
    pub created_by: String,
    #[garde(skip)]
    pub items: Vec<Uuid>,
    #[garde(skip)]
    pub paid: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct UpdateInvoiceRequest {
    #[garde(skip)]
    pub id: Uuid,
    #[garde(range(min = 0.0, max = 1000000.0))]
    pub total: f64,
    #[garde(skip)]
    pub paid: bool,
    #[garde(skip)]
    pub changed_by: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema, Validate)]
pub struct Invoice {
    #[garde(range(min = 1))]
    pub seq: i32,
    #[garde(skip)]
    pub id: String,
    #[garde(skip)]
    pub user_id: String,
    #[garde(range(min = 0.0, max = 1000000.0))]
    pub total: f64,
    #[garde(skip)]
    pub paid: bool,
    #[garde(skip)]
    pub audit_info: AuditInfo,
    #[garde(skip)]
    pub items: Vec<Item>,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ServiceResults {
    pub message: String,
    pub success: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct WithItemsQuery {
    pub with_items: bool,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct InvoiceItemRequest {
    pub invoice_id: Uuid,
    pub item_id: Uuid,
}
