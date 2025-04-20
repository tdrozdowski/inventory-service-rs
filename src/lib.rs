pub mod inventory;
pub mod jwt;
pub mod test_helpers;

use crate::inventory::db::initialize_db_pool;
use crate::inventory::repositories::person::PersonRepositoryImpl;
use crate::inventory::routes::ApiDoc;
use crate::inventory::services::invoice::InvoiceService;
use crate::inventory::services::item::ItemService;
use crate::inventory::services::person::{PersonService, PersonServiceImpl};
use axum::extract::MatchedPath;
use axum::extract::Request;
use axum::middleware::Next;
use axum::response::IntoResponse;
use axum::routing::get;
use axum::{middleware, Router};
use axum_prometheus::metrics;
use axum_prometheus::metrics_exporter_prometheus::{Matcher, PrometheusBuilder, PrometheusHandle};
use sqlx::PgPool;
use std::future::ready;
use std::sync::Arc;
use std::time::Instant;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub person_service: Arc<dyn PersonService + Send + 'static>,
    pub item_service: Arc<dyn ItemService + Send + 'static>,
    pub invoice_service: Arc<dyn InvoiceService + Send + 'static>,
}

impl AppContext {
    pub async fn new() -> Self {
        let db_pool = initialize_db_pool().await;
        let person_service = Self::init_person_service(&db_pool).await;
        let item_service = Self::init_item_service(&db_pool).await;
        let invoice_service = Self::init_invoice_service(&db_pool).await;
        AppContext {
            person_service,
            item_service,
            invoice_service,
        }
    }

    async fn init_person_service(db_pool: &PgPool) -> Arc<dyn PersonService> {
        let person_repo = PersonRepositoryImpl::new(db_pool.clone()).await;
        Arc::new(PersonServiceImpl::new(Arc::new(person_repo)))
    }

    async fn init_item_service(db_pool: &PgPool) -> Arc<dyn ItemService> {
        let item_repo =
            inventory::repositories::item::ItemRepositoryImpl::new(db_pool.clone()).await;
        Arc::new(inventory::services::item::ItemServiceImpl::new(Arc::new(
            item_repo,
        )))
    }

    async fn init_invoice_service(db_pool: &PgPool) -> Arc<dyn InvoiceService> {
        let invoice_repo =
            inventory::repositories::invoice::InvoiceRepositoryImpl::new(db_pool.clone()).await;
        Arc::new(inventory::services::invoice::InvoiceServiceImpl::new(
            Arc::new(invoice_repo),
        ))
    }
}

fn setup_metrics_recorder() -> PrometheusHandle {
    const EXPONENTIAL_SECONDS: &[f64] = &[
        0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
    ];

    PrometheusBuilder::new()
        .set_buckets_for_metric(
            Matcher::Full("http_requests_duration_seconds".to_string()),
            EXPONENTIAL_SECONDS,
        )
        .unwrap()
        .install_recorder()
        .unwrap()
}

fn metrics_app() -> Router {
    let recorder_handle = setup_metrics_recorder();
    Router::new().route("/metrics", get(move || ready(recorder_handle.render())))
}

pub async fn start_server() {
    let app_context = AppContext::new().await;
    let app = Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .nest("/api/v1/authorize", jwt::route())
        .merge(inventory::routes::api_routes_with_status_routes())
        .with_state(app_context)
        .route_layer(middleware::from_fn(track_metrics))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
                // Log the matched route's path (with placeholders not filled in).
                // Use request.uri() or OriginalUri if you want the real path.
                let matched_path = request
                    .extensions()
                    .get::<MatchedPath>()
                    .map(MatchedPath::as_str);

                info_span!(
                    "http_request",
                    method = ?request.method(),
                    matched_path
                )
            }),
        );
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!(
        "Server initialization completed. Listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

pub async fn start_metrics_server() {
    let app = metrics_app();

    // NOTE: expose metrics endpoint on a different port
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    info!(
        "Metrics server initialization completed. Listening on {}",
        listener.local_addr().unwrap()
    );
    axum::serve(listener, app).await.unwrap();
}

async fn track_metrics(req: Request, next: Next) -> impl IntoResponse {
    let start = Instant::now();
    let path = if let Some(matched_path) = req.extensions().get::<MatchedPath>() {
        matched_path.as_str().to_owned()
    } else {
        req.uri().path().to_owned()
    };
    let method = req.method().clone();

    let response = next.run(req).await;

    let latency = start.elapsed().as_secs_f64();
    let status = response.status().as_u16().to_string();

    let labels = [
        ("method", method.to_string()),
        ("path", path),
        ("status", status),
    ];

    metrics::counter!("http_requests_total", &labels).increment(1);
    metrics::histogram!("http_requests_duration_seconds", &labels).record(latency);

    response
}

// add function to add profiling around each request; utilize pyroscope agent
