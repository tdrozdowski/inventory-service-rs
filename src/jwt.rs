use std::fmt::Display;

use crate::AppContext;
use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use axum::{Json, RequestPartsExt, Router};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use axum_macros::debug_handler;
use jsonwebtoken::{decode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tower_http::cors::CorsLayer;

struct Keys {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Keys {
        let encoding_key = EncodingKey::from_secret(secret);
        let decoding_key = DecodingKey::from_secret(secret);
        Keys {
            encoding_key,
            decoding_key,
        }
    }
}

static KEYS: Lazy<Keys> = Lazy::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    pub(crate) client_id: String,
    pub(crate) client_secret: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthResponse {
    token: String,
}

#[derive(Debug)]
pub enum AuthError {
    WrongCredentials,
    MissingCredentials,
    TokenCreation,
    InvalidToken,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AuthError::WrongCredentials => (StatusCode::UNAUTHORIZED, "Wrong credentials"),
            AuthError::MissingCredentials => (StatusCode::BAD_REQUEST, "Missing credentials"),
            AuthError::TokenCreation => (StatusCode::INTERNAL_SERVER_ERROR, "Token creation error"),
            AuthError::InvalidToken => (StatusCode::BAD_REQUEST, "Invalid token"),
        };
        let body = Json(json!({
            "error": error_message,
        }));
        (status, body).into_response()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Claims {
    pub(crate) sub: String,
    pub(crate) exp: usize,
}

impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Subject: {}\nExpiration: {}", self.sub, self.exp)
    }
}

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;
        // Decode the user data
        let token_data =
            decode::<Claims>(bearer.token(), &KEYS.decoding_key, &Validation::default())
                .map_err(|_| AuthError::InvalidToken)?;

        Ok(token_data.claims)
    }
}

#[debug_handler]
pub async fn authorize(Json(payload): Json<AuthRequest>) -> Result<Json<AuthResponse>, AuthError> {
    // TODO - replace with call to lookup user/pass from db
    if payload.client_id == "foo" && payload.client_secret == "bar" {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap();
        let exp = now + std::time::Duration::from_secs(60 * 60 * 24);
        let claims = Claims {
            sub: payload.client_id,
            exp: exp.as_secs() as usize,
        };
        let token = jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding_key)
            .map_err(|_e| AuthError::InvalidToken)?;
        Ok(Json(AuthResponse { token }))
    } else {
        Err(AuthError::WrongCredentials)
    }
}

pub fn gen_token(auth_request: AuthRequest) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap();
    let exp = now + std::time::Duration::from_secs(60 * 60 * 24);
    let claims = Claims {
        sub: auth_request.client_id,
        exp: exp.as_secs() as usize,
    };
    jsonwebtoken::encode(&Header::default(), &claims, &KEYS.encoding_key)
        .expect("Failed to generate token")
}

pub fn route() -> Router<AppContext> {
    Router::new().route("/", post(authorize)).layer(
        CorsLayer::new()
            .allow_origin("*".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::POST]),
    )
}

#[cfg(test)]
mod tests {
    use crate::jwt::{Claims, KEYS};
    use crate::test_helpers::body_to_string;
    use axum::extract::FromRequestParts;
    use axum::response::IntoResponse;
    use jsonwebtoken::{decode, Validation};
    use tower::ServiceExt;

    #[test]
    fn test_gen_token() {
        let auth_request = crate::jwt::AuthRequest {
            client_id: "foo".to_string(),
            client_secret: "bar".to_string(),
        };
        let token = crate::jwt::gen_token(auth_request);
        assert!(!token.is_empty());
        let token_data =
            decode::<Claims>(token.as_str(), &KEYS.decoding_key, &Validation::default())
                .expect("Failed to decode token");
        assert_eq!(token_data.claims.sub, "foo");
        assert!(token_data.claims.exp > 0);
    }

    #[tokio::test]
    async fn test_into_response() {
        let error = crate::jwt::AuthError::WrongCredentials;
        let response = error.into_response();
        assert_eq!(response.status(), 401);
        let error = crate::jwt::AuthError::MissingCredentials;
        let response = error.into_response();
        assert_eq!(response.status(), 400);
        let error = crate::jwt::AuthError::TokenCreation;
        let response = error.into_response();
        assert_eq!(response.status(), 500);
        let error = crate::jwt::AuthError::InvalidToken;
        let response = error.into_response();
        assert_eq!(response.status(), 400);
        let response_str = body_to_string(response.into_body()).await.unwrap();
        assert_eq!(
            response_str.as_bytes(),
            r#"{"error":"Invalid token"}"#.as_bytes()
        );
    }

    #[test]
    fn test_display() {
        let claims = Claims {
            sub: "test".to_string(),
            exp: 0,
        };
        let display = format!("{}", claims);
        assert_eq!(display, "Subject: test\nExpiration: 0");
    }

    #[tokio::test]
    async fn test_authorize() {
        let auth_request = crate::jwt::AuthRequest {
            client_id: "foo".to_string(),
            client_secret: "bar".to_string(),
        };
        let response = crate::jwt::authorize(axum::Json(auth_request)).await;
        assert!(response.is_ok());
        let response = response.unwrap();
        assert!(!response.0.token.is_empty());
    }

    #[tokio::test]
    async fn test_authorize_wrong_credentials() {
        let auth_request = crate::jwt::AuthRequest {
            client_id: "foo".to_string(),
            client_secret: "baz".to_string(),
        };
        let response = crate::jwt::authorize(axum::Json(auth_request)).await;
        assert!(response.is_err());
        let error = response.unwrap_err();
        match error {
            crate::jwt::AuthError::WrongCredentials => assert!(true),
            _ => assert!(false),
        }
    }

    #[tokio::test]
    async fn test_from_request_parts() {
        let token = crate::jwt::gen_token(crate::jwt::AuthRequest {
            client_id: "foo".to_string(),
            client_secret: "bar".to_string(),
        });
        let request = axum::http::Request::builder()
            .header("Authorization", format!("Bearer {}", token))
            .body(())
            .unwrap();
        let mut parts = request.into_parts().0;
        let claims = crate::jwt::Claims::from_request_parts(&mut parts, &())
            .await
            .unwrap();
        assert_eq!(claims.sub, "foo");
    }

    #[tokio::test]
    async fn test_from_request_parts_invalid_token() {
        let request = axum::http::Request::builder()
            .header("Authorization", "Bearer invalid_token")
            .body(())
            .unwrap();
        let mut parts = request.into_parts().0;
        let result = crate::jwt::Claims::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::jwt::AuthError::InvalidToken
        ));
    }

    #[tokio::test]
    async fn test_from_request_parts_missing_token() {
        let request = axum::http::Request::builder().body(()).unwrap();
        let mut parts = request.into_parts().0;
        let result = crate::jwt::Claims::from_request_parts(&mut parts, &()).await;
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            crate::jwt::AuthError::InvalidToken
        ));
    }

    #[tokio::test]
    async fn test_authorize_route() {
        let app = crate::jwt::route().with_state(crate::test_helpers::test_app_context(
            crate::inventory::services::person::MockPersonService::new(),
            crate::inventory::services::item::MockItemService::new(),
            crate::inventory::services::invoice::MockInvoiceService::new(),
        ));
        let request = axum::http::Request::builder()
            .uri("/")
            .method(axum::http::Method::POST)
            .header("Content-Type", "application/json")
            .body(r#"{"client_id":"foo","client_secret":"bar"}"#.to_string())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), 200);
        let body = body_to_string(response.into_body()).await.unwrap();
        assert!(!body.is_empty());
    }
}
