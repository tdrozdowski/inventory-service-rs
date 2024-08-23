use crate::inventory::services::person::MockPersonService;
use crate::jwt::{AuthRequest, Claims};
use crate::{jwt, AppContext};
use axum::body::Body;
use futures::StreamExt;
use std::string::FromUtf8Error;
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

pub fn mock_claims() -> Claims {
    Claims {
        sub: "test".to_string(),
        exp: 0,
    }
}

pub fn mock_token() -> String {
    let auth_request = AuthRequest {
        client_id: "foo".to_string(),
        client_secret: "bar".to_string(),
    };
    let token = jwt::gen_token(auth_request.clone());
    format!("Bearer {}", token)
}

pub async fn body_to_string(body: Body) -> Result<String, FromUtf8Error> {
    let mut data = body.into_data_stream();
    let mut bytes = Vec::new();

    while let Some(chunk) = data.next().await {
        bytes.extend_from_slice(&chunk.unwrap());
    }

    String::from_utf8(bytes)
}
