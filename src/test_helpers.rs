use crate::inventory::services::person::MockPersonService;
use crate::AppContext;
use std::sync::Arc;
use uuid::Uuid;

pub const FIRST_PERSON_UUID: &str = "2b1b425e-dee2-4227-8d94-f470a0ce0cd0";
pub const INVALID_UUID: &str = "00000000-0000-0000-0000-000000000000";
pub const FIRST_PERSON_ID: i32 = 1;

pub fn first_person_uuid() -> Uuid {
    Uuid::parse_str(FIRST_PERSON_UUID).unwrap()
}
pub fn invalid_uuid() -> Uuid {
    Uuid::parse_str(INVALID_UUID).unwrap()
}

pub fn string_to_uuid(s: &str) -> Uuid {
    Uuid::parse_str(s).unwrap()
}

pub fn test_app_context(mock_person_service: MockPersonService) -> AppContext {
    let person_service = Arc::new(mock_person_service);
    AppContext { person_service }
}
