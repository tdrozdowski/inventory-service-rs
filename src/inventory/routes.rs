use crate::inventory::handlers::person;
use crate::AppContext;
use axum::http::{HeaderValue, Method};
use axum::Router;
use tower_http::cors::CorsLayer;

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

fn all_routes() -> Router<AppContext> {
    Router::new().nest("/persons", person_routes())
}

fn v1_routes() -> Router<AppContext> {
    Router::new().nest("/v1", all_routes())
}

fn api_routes() -> Router<AppContext> {
    Router::new().nest("/api", v1_routes())
}

#[cfg(test)]
mod tests {
    use crate::inventory::model::{CreatePersonRequest, Person};
    use crate::inventory::routes::{api_routes, person_routes};
    use crate::inventory::services::person::MockPersonService;
    use crate::test_helpers::test_app_context;
    use axum::body::Body;
    use axum::http::Request;
    use axum::{http, Router};
    use tower::ServiceExt;

    async fn app(mock_person_service: MockPersonService) -> Router {
        Router::new()
            .nest("/persons", person_routes())
            .with_state(test_app_context(mock_person_service))
    }

    async fn app_v1(mock_person_service: MockPersonService) -> Router {
        api_routes().with_state(test_app_context(mock_person_service))
    }

    #[tokio::test]
    async fn test_person_routes_get_all() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_get_persons()
            .returning(|_, _| Box::pin(async move { Ok(vec![Person::default()]) }));

        let app = app(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons")
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

        let app = app(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
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
        let app = app(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons")
            .header(http::header::CONTENT_TYPE, "application/json")
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

        let app = app(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .method(http::Method::DELETE)
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

        let app = app_v1(mock_person_service).await;
        let request = Request::builder()
            .uri("/api/v1/persons/2b1b425e-dee2-4227-8d94-f470a0ce0cd0")
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
