use crate::inventory::model::{CreatePersonRequest, Person};
use crate::inventory::services::ServiceError;
use crate::AppContext;
use axum::extract::{Path, Query, State};
use axum::Json;
use tracing::instrument;
use uuid::Uuid;

#[axum_macros::debug_handler]
#[instrument]
pub async fn get_persons(
    Query(last_id): Query<Option<i32>>,
    Query(page_size): Query<i64>,
    State(app_context): State<AppContext>,
) -> Result<Json<Vec<Person>>, ServiceError> {
    app_context
        .person_service
        .get_persons(last_id, page_size)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
pub async fn create_person(
    State(app_context): State<AppContext>,
    Json(person): Json<CreatePersonRequest>,
) -> Result<Json<Person>, ServiceError> {
    app_context
        .person_service
        .create_person(person)
        .await
        .map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
pub async fn delete_person(
    Path(id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<()>, ServiceError> {
    app_context.person_service.delete_person(id).await.map(Json)
}

#[axum_macros::debug_handler]
#[instrument]
pub async fn get_person_by_id(
    Path(id): Path<Uuid>,
    State(app_context): State<AppContext>,
) -> Result<Json<Person>, ServiceError> {
    app_context.person_service.get_person(id).await.map(Json)
}

#[cfg(test)]
mod tests {
    use crate::inventory::services::person::MockPersonService;
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
        let cloned_expeted_person = expected_person.clone();
        let mut mock_person_service = MockPersonService::new();
        mock_person_service.expect_get_person().returning(move |_| {
            let cloned_person = expected_person.clone();
            Box::pin(async move { Ok(cloned_person) })
        });
        let app_context = test_app_context(mock_person_service);
        let result = super::get_person_by_id(Path(Uuid::new_v4()), State(app_context)).await;
        assert!(result.is_ok());
        let person = result.unwrap().0;
        assert_eq!(person, cloned_expeted_person);
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
        let app_context = test_app_context(mock_person_service);
        let result = super::get_persons(Query(None), Query(100), State(app_context)).await;
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
        let app_context = test_app_context(mock_person_service);
        let result = super::create_person(State(app_context), Json(person)).await;
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
        let app_context = test_app_context(mock_person_service);
        let result = super::delete_person(Path(Uuid::new_v4()), State(app_context)).await;
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
        let app_context = test_app_context(mock_person_service);
        let result = super::get_person_by_id(Path(Uuid::new_v4()), State(app_context)).await;
        assert!(result.is_err());
        let error = result.unwrap_err();
        match error {
            ServiceError::NotFound(msg) => assert!(true, "Expected NotFound: msg: {}", msg),
            _ => assert!(false, "Expected NotFound"),
        }
    }
}
