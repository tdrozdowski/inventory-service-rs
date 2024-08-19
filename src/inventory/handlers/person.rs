use crate::inventory::model::Person;
use crate::inventory::services::ServiceError;
use crate::AppContext;
use axum::extract::{Path, State};
use axum::Json;
use tracing::instrument;
use uuid::Uuid;

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
    use axum::extract::{Path, State};

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
}
