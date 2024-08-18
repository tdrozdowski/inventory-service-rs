use chrono::{DateTime, Utc};
use garde::Validate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Validate)]
pub struct CreatePersonRequest {
    #[garde(length(min = 3, max = 50))]
    pub name: String,
    #[garde(email)]
    pub email: String,
    #[garde(skip)]
    pub created_by: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash, Validate)]
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

#[derive(Debug, Default, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct AuditInfo {
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub changed_by: String,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct Person {
    pub seq: i32,
    pub id: String,
    pub name: String,
    pub email: String,
    pub audit_info: AuditInfo,
}
