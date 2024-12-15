use axum::http::StatusCode;

pub async fn healthz() -> Result<StatusCode, String> {
    Ok(StatusCode::OK)
}

pub async fn readyz() -> Result<StatusCode, String> {
    Ok(StatusCode::OK)
}

pub async fn livenessz() -> Result<StatusCode, String> {
    Ok(StatusCode::OK)
}
