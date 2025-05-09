use crate::inventory::handlers::status::{healthz, livenessz, readyz};
use crate::inventory::handlers::{invoice, item, person};
use crate::AppContext;
use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/v1/api/persons", api=person::PersonApi),
        (path = "/v1/api/items", api=item::ItemApi),
        (path = "/v1/api/invoices", api=invoice::InvoiceApi)
    )
)]
pub struct ApiDoc;
pub fn person_routes() -> Router<AppContext> {
    Router::new()
        .route(
            "/",
            axum::routing::get(person::get_persons).post(person::create_person),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
        .route(
            "/:id",
            axum::routing::get(person::get_person_by_id).delete(person::delete_person),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::DELETE]),
        )
}

pub fn item_routes() -> Router<AppContext> {
    Router::new()
        .route(
            "/",
            axum::routing::get(item::get_items).post(item::create_item),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
        .route(
            "/:id",
            axum::routing::get(item::get_item_by_id)
                .delete(item::delete_item)
                .put(item::update_item),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::DELETE]),
        )
}

fn invoice_routes() -> Router<AppContext> {
    Router::new()
        .route(
            "/",
            axum::routing::get(invoice::get_invoices).post(invoice::create_invoice),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::POST]),
        )
        .route(
            "/:id",
            axum::routing::get(invoice::get_invoice_by_id)
                .delete(invoice::delete_invoice)
                .put(invoice::update_invoice),
        )
        .route(
            "/:id/items",
            axum::routing::post(invoice::add_invoice_items),
        )
        .route(
            "/:invoice_id/items/:item_id",
            axum::routing::delete(invoice::remove_invoice_item),
        )
        .route(
            "/users/:id",
            axum::routing::get(invoice::get_invoices_by_user),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("*".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET, Method::DELETE]),
        )
}

fn status_routes() -> Router<AppContext> {
    Router::new()
        .route("/healthz", axum::routing::get(healthz))
        .route("/livez", axum::routing::get(livenessz))
        .route("/readyz", axum::routing::get(readyz))
}

fn all_routes() -> Router<AppContext> {
    Router::new()
        .nest("/persons", person_routes())
        .nest("/items", item_routes())
        .nest("/invoices", invoice_routes())
}

fn v1_routes() -> Router<AppContext> {
    Router::new().nest("/v1", all_routes())
}

pub(crate) fn api_routes_with_status_routes() -> Router<AppContext> {
    Router::new()
        .nest("/api", v1_routes())
        .nest("/status", status_routes())
}

#[cfg(test)]
mod tests {
    use crate::inventory::model::{
        CreateInvoiceRequest, CreateItemRequest, CreatePersonRequest, DeleteResults,
        InvoiceItemRequest, Item, Person, UpdateInvoiceRequest, UpdateItemRequest,
    };
    use crate::inventory::routes::{api_routes_with_status_routes, item_routes, person_routes};
    use crate::inventory::services::invoice::MockInvoiceService;
    use crate::inventory::services::item::MockItemService;
    use crate::inventory::services::person::MockPersonService;
    use crate::test_helpers::{mock_token, test_app_context};
    use axum::body::Body;
    use axum::http::Request;
    use axum::{http, Router};
    use tower::ServiceExt;
    use uuid::Uuid;

    async fn app(
        mock_person_service: MockPersonService,
        mock_item_service: MockItemService,
        mock_invoice_service: MockInvoiceService,
    ) -> Router {
        Router::new()
            .nest("/persons", person_routes())
            .nest("/items", item_routes())
            .with_state(test_app_context(
                mock_person_service,
                mock_item_service,
                mock_invoice_service,
            ))
    }

    async fn app_with_live_mock_person_service(mock_person_service: MockPersonService) -> Router {
        let mock_item_service = MockItemService::new();
        let mock_invoice_service = MockInvoiceService::new();
        app(mock_person_service, mock_item_service, mock_invoice_service).await
    }

    async fn app_with_live_mock_item_service(mock_item_service: MockItemService) -> Router {
        let mock_person_service = MockPersonService::new();
        let mock_invoice_service = MockInvoiceService::new();
        app(mock_person_service, mock_item_service, mock_invoice_service).await
    }

    async fn app_v1(
        mock_person_service: MockPersonService,
        mock_item_service: MockItemService,
        mock_invoice_service: MockInvoiceService,
    ) -> Router {
        api_routes_with_status_routes().with_state(test_app_context(
            mock_person_service,
            mock_item_service,
            mock_invoice_service,
        ))
    }

    async fn app_v1_with_live_mock_person_service(
        mock_person_service: MockPersonService,
    ) -> Router {
        let mock_item_service = MockItemService::new();
        let mock_invoice_service = MockInvoiceService::new();
        app_v1(mock_person_service, mock_item_service, mock_invoice_service).await
    }

    async fn app_v1_with_live_mock_item_service(mock_item_service: MockItemService) -> Router {
        let mock_person_service = MockPersonService::new();
        let mock_invoice_service = MockInvoiceService::new();
        app_v1(mock_person_service, mock_item_service, mock_invoice_service).await
    }

    async fn app_v1_with_live_mock_invoice_service(
        mock_invoice_service: MockInvoiceService,
    ) -> Router {
        let mock_person_service = MockPersonService::new();
        let mock_item_service = MockItemService::new();
        app_v1(mock_person_service, mock_item_service, mock_invoice_service).await
    }

    #[tokio::test]
    async fn test_person_routes_get_all() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_get_persons()
            .returning(|_, _| Box::pin(async move { Ok(vec![Person::default()]) }));
        let app = app_with_live_mock_person_service(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_person_routes_get_by_id() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_get_person()
            .returning(|_| Box::pin(async move { Ok(Person::default()) }));
        let app = app_with_live_mock_person_service(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_person_routes_create() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_create_person()
            .returning(|_| Box::pin(async move { Ok(Person::default()) }));
        let new_person = CreatePersonRequest::default();
        let app = app_with_live_mock_person_service(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons")
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::POST)
            .body(Body::from(serde_json::to_string(&new_person).unwrap()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_person_routes_delete() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_delete_person()
            .returning(|_| Box::pin(async move { Ok(()) }));

        let app = app_with_live_mock_person_service(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .method(http::Method::DELETE)
            .header(http::header::AUTHORIZATION, mock_token())
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_get_person_by_id_route() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_get_person()
            .returning(|_| Box::pin(async move { Ok(Person::default()) }));

        let app = app_v1_with_live_mock_person_service(mock_person_service).await;
        let request = Request::builder()
            .uri("/api/v1/persons/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_get_items_route() {
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_get_all_items()
            .returning(|_| Box::pin(async move { Ok(vec![]) }));
        let app = app_v1_with_live_mock_item_service(mock_item_service).await;
        let request = Request::builder()
            .uri("/api/v1/items")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_get_items_route() {
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_get_all_items()
            .returning(|_| Box::pin(async move { Ok(vec![]) }));
        let app = app_with_live_mock_item_service(mock_item_service).await;
        let request = Request::builder()
            .uri("/items")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_create_item_route() {
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_create_item()
            .returning(|_| Box::pin(async move { Ok(Item::default()) }));
        let app = app_with_live_mock_item_service(mock_item_service).await;
        let create_item_request = CreateItemRequest::default();
        let request = Request::builder()
            .uri("/items")
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::POST)
            .body(Body::from(
                serde_json::to_string(&create_item_request).unwrap(),
            ))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_item_route() {
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_update_item()
            .returning(|_| Box::pin(async move { Ok(Item::default()) }));
        let app = app_with_live_mock_item_service(mock_item_service).await;
        let mut update_item_request = UpdateItemRequest::default();
        let uuid = Uuid::new_v4();
        update_item_request.id = uuid.to_string();
        let request = Request::builder()
            .uri(format!("/items/{}", uuid.clone()))
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::PUT)
            .body(Body::from(
                serde_json::to_string(&update_item_request).unwrap(),
            ))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_delete_item_route() {
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_delete_item()
            .returning(|_| Box::pin(async move { Ok(DeleteResults::default()) }));
        let app = app_with_live_mock_item_service(mock_item_service).await;
        let uuid = Uuid::new_v4();
        let request = Request::builder()
            .uri(format!("/items/{}", uuid.clone()))
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_get_item_by_id_route() {
        let mut mock_item_service = MockItemService::new();
        mock_item_service
            .expect_get_item_by_id()
            .returning(|_| Box::pin(async move { Ok(Item::default()) }));
        let app = app_v1_with_live_mock_item_service(mock_item_service).await;
        let request = Request::builder()
            .uri("/api/v1/items/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_update_item_mismatched_ids() {
        let mock_item_service = MockItemService::new();
        let app = app_with_live_mock_item_service(mock_item_service).await;
        let mut update_item_request = UpdateItemRequest::default();
        let uuid = Uuid::new_v4();
        update_item_request.id = uuid.to_string();
        let request = Request::builder()
            .uri(format!("/items/{}", Uuid::new_v4()))
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::PUT)
            .body(Body::from(
                serde_json::to_string(&update_item_request).unwrap(),
            ))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn test_api_v1_get_invoice_by_id_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoice()
            .returning(|_, _| Box::pin(async move { Ok(Default::default()) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let request = Request::builder()
            .uri("/api/v1/invoices/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_get_invoices_by_user_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_get_invoices_for_user()
            .returning(|_| Box::pin(async move { Ok(vec![]) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let request = Request::builder()
            .uri("/api/v1/invoices/users/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_create_invoice_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_create_invoice()
            .returning(|_| Box::pin(async move { Ok(Default::default()) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let create_request = CreateInvoiceRequest::default();
        let request = Request::builder()
            .uri("/api/v1/invoices")
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::POST)
            .body(Body::from(serde_json::to_string(&create_request).unwrap()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_delete_invoice_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_delete_invoice()
            .returning(|_| Box::pin(async move { Ok(Default::default()) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let request = Request::builder()
            .uri("/api/v1/invoices/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_add_invoice_items_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_add_item_to_invoice()
            .returning(|_, _| Box::pin(async move { Ok(Default::default()) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let request_body = InvoiceItemRequest {
            item_id: Uuid::new_v4(),
            invoice_id: Uuid::new_v4(),
        };
        let request = Request::builder()
            .uri(format!(
                "/api/v1/invoices/{}/items",
                request_body.invoice_id.clone()
            ))
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::POST)
            .body(Body::from(serde_json::to_string(&request_body).unwrap()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_remove_invoice_item_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_remove_item_from_invoice()
            .returning(|_, _| Box::pin(async move { Ok(Default::default()) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let request_body = InvoiceItemRequest {
            item_id: Uuid::new_v4(),
            invoice_id: Uuid::new_v4(),
        };
        let request = Request::builder()
            .uri(format!(
                "/api/v1/invoices/{}/items/{}",
                request_body.invoice_id.clone(),
                request_body.item_id.clone()
            ))
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::DELETE)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_update_invoice_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_update_invoice()
            .returning(|_| Box::pin(async move { Ok(Default::default()) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let update_request = UpdateInvoiceRequest::default();
        let request = Request::builder()
            .uri(format!("/api/v1/invoices/{}", update_request.id.clone()))
            .header(http::header::CONTENT_TYPE, "application/json")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::PUT)
            .body(Body::from(serde_json::to_string(&update_request).unwrap()))
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }

    #[tokio::test]
    async fn test_api_v1_get_invoices_route() {
        let mut mock_invoice_service = MockInvoiceService::new();
        mock_invoice_service
            .expect_list_all_invoices()
            .returning(|_| Box::pin(async move { Ok(vec![]) }));
        let app = app_v1_with_live_mock_invoice_service(mock_invoice_service).await;
        let request = Request::builder()
            .uri("/api/v1/invoices")
            .header(http::header::AUTHORIZATION, mock_token())
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
