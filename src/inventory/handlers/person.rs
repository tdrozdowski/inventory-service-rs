use crate::inventory::model::{CreatePersonRequest, Pagination, Person};
use crate::inventory::services::ServiceError;
use crate::jwt::Claims;
use crate::{inventory, AppContext};
use axum::extract::{Path, Query, State};
use axum::Json;
use tracing::{debug, instrument};
use utoipa::OpenApi;
use uuid::Uuid;

#[derive(OpenApi)]
#[openapi(
    paths(get_persons, get_person_by_id, create_person, delete_person),
    components(schemas(
        inventory::model::CreatePersonRequest,
        inventory::model::UpdatePersonRequest,
        inventory::model::Person,
        inventory::model::ApiError,
        inventory::model::AuditInfo
    ))
)]
pub struct PersonApi;

#[axum_macros::debug_handler]
#[utoipa::path(
    get,
    path = "",
    summary = "Get a list of persons",
    description = "Returns a list of persons",
    params(
        Pagination,
        ("Authorization", Header, description="Bearer token"),
    ),
    responses(
        (status = 200, description = "Returns a list of persons", body=[Person]),
        (status = 400, description = "Bad request", body=ApiError),
        (status = 401, description = "Unauthorized", body=ApiError),
        (status = 403, description = "Forbidden", body=ApiError),
        (status = 500, description = "Internal server error", body=ApiError),
    ),
)]
#[instrument]
pub async fn get_persons(
    claims: Claims,
    pagination: Option<Query<Pagination>>,
    State(app_context): State<AppContext>,
) -> Result<Json<Vec<Person>>, ServiceError> {
    let Query(pagination) = pagination.unwrap_or_default();
    debug!("Claims: {:?}", claims);
    app_context
        .person_service
        .get_persons(pagination.last_id, pagination.page_size) // TODO - refactor service interface to accept Option<Pagination>
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
        post,
        path = "",
        request_body = CreatePersonRequest,
        responses(
            (status = 201, description = "Todo item created successfully", body = Person),
            (status = 400, description = "Bad request", body = ApiError),
            (status = 401, description = "Unauthorized", body = ApiError),
            (status = 403, description = "Forbidden", body = ApiError),
            (status = 404, description = "Not found", body = ApiError),
            (status = 500, description = "Internal server error", body = ApiError),
        ),
        params(
            ("Authorization", Header, description="Bearer token"),
        ),
)]
pub async fn create_person(
    claims: Claims,
    State(app_context): State<AppContext>,
    Json(person): Json<CreatePersonRequest>,
) -> Result<Json<Person>, ServiceError> {
    debug!("Claims: {:?}", claims);
    app_context
        .person_service
        .create_person(person)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    delete,
    path = "/{id}",
    summary = "Remove a specific person",
    description = "Removes a specific person",
    params(
        ("Authorization", Header, description="Bearer token"),
        ("id" = Uuid, Path, description = "Person Id - UUID"),
    ),
    responses(
        (status = 200, description = "Indicates success", body=String),
        (status = 400, description = "Bad request", body=ApiError),
        (status = 401, description = "Unauthorized", body=ApiError),
        (status = 403, description = "Forbidden", body=ApiError),
        (status = 404, description = "Not found", body=ApiError),
        (status = 500, description = "Internal server error", body=ApiError),
    ),
)]
pub async fn delete_person(
    claims: Claims,
    Path(id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<()>, ServiceError> {
    debug!("Claims: {:?}", claims);
    app_context.person_service.delete_person(id).await.map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
#[utoipa::path(
    get,
    path = "/{id}",
    summary = "Get a specific person",
    description = "Returns a specific person identified by the id",
    params(
        ("id" = Uuid, Path, description = "Person Id - UUID"),
        ("Authorization", Header, description="Bearer token"),
    ),
    responses(
        (status = 200, description = "Returns a list of persons", body=Person),
        (status = 400, description = "Bad request", body=ApiError),
        (status = 401, description = "Unauthorized", body=ApiError),
        (status = 403, description = "Forbidden", body=ApiError),
        (status = 500, description = "Internal server error", body=ApiError),
    ),
)]
pub async fn get_person_by_id(
    claims: Claims,
    Path(id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<Person>, ServiceError> {
    debug!("Claims: {:?}", claims);
    app_context.person_service.get_person(id).await.map(Json)
}

#[cfg(test)]
mod tests {
    use crate::inventory::model::Pagination;
    use crate::inventory::services::item::MockItemService;
    use crate::inventory::services::person::MockPersonService;
    use crate::test_helpers::mock_claims;
    use axum::extract::{Path, Query, State};
    use axum::Json;

    #[tokio::test]
    async fn test_get_person_by_id() {
        use crate::inventory::model::Person;
        use crate::test_helpers::test_app_context;
        use uuid::Uuid;

        let expected_person = Person {
            seq: 1,
            id: Uuid::new_v4().to_string(),
            name: "John".to_string(),
            email: "test@test.com".to_string(),
            audit_info: Default::default(),
        };
        let cloned_expected_person = expected_person.clone();
        let mut mock_person_service = MockPersonService::new();
        mock_person_service.expect_get_person().returning(move |_| {
            let cloned_person = expected_person.clone();
            Box::pin(async move { Ok(cloned_person) })
        });
        let app_context = test_app_context(mock_person_service, MockItemService::new());
        let result =
            super::get_person_by_id(mock_claims(), Path(Uuid::new_v4()), State(app_context)).await;
        assert!(result.is_ok());
        let person = result.unwrap().0;
        assert_eq!(person, cloned_expected_person);
    }

    #[tokio::test]
    async fn test_get_persons() {
        use crate::inventory::model::Person;
        use crate::test_helpers::test_app_context;
        use uuid::Uuid;

        let expected_persons = vec![
            Person {
                seq: 1,
                id: Uuid::new_v4().to_string(),
                name: "John".to_string(),
                email: "john@test.com".to_string(),
                audit_info: Default::default(),
            },
            Person {
                seq: 2,
                id: Uuid::new_v4().to_string(),
                name: "Jane".to_string(),
                email: "jane@test.com".to_string(),
                audit_info: Default::default(),
            },
        ];
        let cloned_expected_persons = expected_persons.clone();
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_get_persons()
            .returning(move |_, _| {
                let cloned_persons = expected_persons.clone();
                Box::pin(async move { Ok(cloned_persons) })
            });
        let app_context = test_app_context(mock_person_service, MockItemService::new());
        let maybe_pagination = Some(Query(Pagination::default()));
        let result = super::get_persons(mock_claims(), maybe_pagination, State(app_context)).await;
        assert!(result.is_ok());
        let persons = result.unwrap().0;
        assert_eq!(persons, cloned_expected_persons);
    }

    #[tokio::test]
    async fn test_create_person() {
        use crate::inventory::model::{CreatePersonRequest, Person};
        use crate::test_helpers::test_app_context;
        use uuid::Uuid;

        let person = CreatePersonRequest {
            name: "John".to_string(),
            email: "john@test.com".to_string(),
            created_by: "test".to_string(),
        };
        let expected_person = Person {
            seq: 1,
            id: Uuid::new_v4().to_string(),
            name: person.name.clone(),
            email: person.email.clone(),
            audit_info: Default::default(),
        };
        let cloned_expected_person = expected_person.clone();
        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_create_person()
            .returning(move |_| {
                let cloned_person = expected_person.clone();
                Box::pin(async move { Ok(cloned_person) })
            });
        let app_context = test_app_context(mock_person_service, MockItemService::new());
        let result = super::create_person(mock_claims(), State(app_context), Json(person)).await;
        assert!(result.is_ok());
        let person = result.unwrap().0;
        assert_eq!(person, cloned_expected_person);
    }

    #[tokio::test]
    async fn test_delete_person() {
        use crate::test_helpers::test_app_context;
        use uuid::Uuid;

        let mut mock_person_service = MockPersonService::new();
        mock_person_service
            .expect_delete_person()
            .returning(move |_| Box::pin(async move { Ok(()) }));
        let app_context = test_app_context(mock_person_service, MockItemService::new());
        let result =
            super::delete_person(mock_claims(), Path(Uuid::new_v4()), State(app_context)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_person_by_id_not_found() {
        use crate::inventory::services::ServiceError;
        use crate::test_helpers::test_app_context;
        use uuid::Uuid;

        let mut mock_person_service = MockPersonService::new();
        mock_person_service.expect_get_person().returning(move |_| {
            Box::pin(async move { Err(ServiceError::NotFound("Mock NotFound".to_string())) })
        });
        let app_context = test_app_context(mock_person_service, MockItemService::new());
        let result =
            super::get_person_by_id(mock_claims(), Path(Uuid::new_v4()), State(app_context)).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            ServiceError::NotFound(msg) => assert!(true, "Expected NotFound: msg: {}", msg),
            _ => assert!(false, "Expected NotFound"),
        }
    }
}
