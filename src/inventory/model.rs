use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct CreatePersonRequest {
    pub name: String,
    pub email: String,
    pub created_by: String,
}

#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
pub struct UpdatePersonRequest {
    pub id: String,
    pub name: String,
    pub email: String,
    pub changed_by: String,
}
