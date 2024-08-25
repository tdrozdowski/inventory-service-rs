use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Deserialize, IntoParams)]
pub struct Pagination {
    pub(crate) last_id: Option<i32>,
    pub(crate) page_size: i64,
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
