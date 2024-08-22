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

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthRequest {
    client_id: String,
    client_secret: String,
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

#[async_trait]
impl<S> FromRequestParts<S> for Claims {
    type Rejection = AuthError;
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, AuthError> {
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

pub fn route() -> Router<AppContext> {
    Router::new().route("/", post(authorize)).layer(
        CorsLayer::new()
            .allow_origin("*".parse::<HeaderValue>().unwrap())
            .allow_methods([Method::POST]),
    )
}
