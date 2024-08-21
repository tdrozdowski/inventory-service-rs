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

#[cfg(test)]
mod tests {
    use crate::inventory::model::Person;
    use crate::inventory::routes::person_routes;
    use crate::inventory::services::person::MockPersonService;
    use crate::test_helpers::test_app_context;
    use crate::AppContext;
    use axum::body::Body;
    use axum::handler::Handler;
    use axum::http::Request;
    use axum::{http, Router};
    use tower::ServiceExt;

    async fn app(mock_person_service: MockPersonService) -> Router {
        Router::new()
            .nest("/persons", person_routes())
            .with_state(test_app_context(mock_person_service))
    }
    #[tokio::test]
    async fn test_person_routes_get_all() {
        let mut mock_person_service = MockPersonService::new();
        mock_person_service.expect_get_persons().returning(|_, _| {
            Box::pin(async move {
                let empty_vec_person: Vec<Person> = vec![];
                Ok(empty_vec_person)
            })
        });

        let app = app(mock_person_service).await;
        let request = Request::builder()
            .uri("/persons?last_id=1&page_size=10")
            .method(http::Method::GET)
            .body(Body::empty())
            .unwrap();
        let response = app.oneshot(request).await.unwrap();
        assert_eq!(response.status(), http::StatusCode::OK);
    }
}
