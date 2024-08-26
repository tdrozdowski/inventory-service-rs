pub mod inventory;
pub mod jwt;
pub mod test_helpers;

use crate::inventory::db::initialize_db_pool;
use crate::inventory::repositories::person::PersonRepositoryImpl;
use crate::inventory::routes::ApiDoc;
use crate::inventory::services::item::ItemService;
use crate::inventory::services::person::{PersonService, PersonServiceImpl};
use axum::extract::MatchedPath;
use axum::http::Request;
use axum::Router;
use sqlx::PgPool;
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use tracing::{info, info_span};
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[derive(Clone, Debug)]
pub struct AppContext {
    pub person_service: Arc<dyn PersonService + Send + 'static>,
    pub item_service: Arc<dyn ItemService + Send + 'static>,
}

impl AppContext {
    pub async fn new() -> Self {
        let db_pool = initialize_db_pool().await;
        let person_service = Self::init_person_service(&db_pool).await;
        let item_service = Self::init_item_service(&db_pool).await;
        AppContext {
            person_service,
            item_service,
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
}

pub async fn start_server() {
    let app_context = AppContext::new().await;
    let app = Router::new()
        .merge(Redoc::with_url("/redoc", ApiDoc::openapi()))
        .nest("/api/v1/authorize", jwt::route())
        .merge(inventory::routes::api_routes())
        .with_state(app_context)
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
                    matched_path,
                    some_other_field = tracing::field::Empty, // TODO - remove or add new fields as determined
                )
            }),
        );
    info!("Server initialization completed.  Listening on: http://localhost:3000");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
