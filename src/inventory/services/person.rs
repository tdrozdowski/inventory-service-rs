use crate::inventory::model::{AuditInfo, CreatePersonRequest, Person, UpdatePersonRequest};
use crate::inventory::repositories::person::{PersonRepository, PersonRow};
use crate::inventory::services::ServiceError;
use crate::test_helpers::string_to_uuid;
use async_trait::async_trait;
use garde::Validate;
use std::fmt::Debug;
use std::sync::Arc;
use tracing::instrument;
use uuid::Uuid;

#[async_trait]
#[mockall::automock]
pub trait PersonService: Sync + Send + Debug + 'static {
    async fn get_person(&self, id: Uuid) -> Result<Person, ServiceError>;
    async fn get_persons(
        &self,
        last_id: Option<i32>,
        page_size: i64,
    ) -> Result<Vec<Person>, ServiceError>;
    async fn create_person(
        &self,
        create_person_request: CreatePersonRequest,
    ) -> Result<Person, ServiceError>;
    async fn update_person(
        &self,
        update_person_request: UpdatePersonRequest,
    ) -> Result<Person, ServiceError>;
    async fn delete_person(&self, id: Uuid) -> Result<(), ServiceError>;
}

#[derive(Debug)]
pub struct PersonServiceImpl {
    person_repo: Arc<dyn PersonRepository + Send + Sync>,
}

impl PersonServiceImpl {
    pub(crate) fn new(person_repo: Arc<dyn PersonRepository + Send + Sync>) -> PersonServiceImpl {
        PersonServiceImpl { person_repo }
    }
}

#[async_trait]
impl PersonService for PersonServiceImpl {
    #[instrument]
    async fn get_person(&self, id: Uuid) -> Result<Person, ServiceError> {
        let results = self.person_repo.get_person_by_uuid(id).await;
        match results {
            Ok(person) => Ok(person.into()),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument]
    async fn get_persons(
        &self,
        last_id: Option<i32>,
        page_size: i64,
    ) -> Result<Vec<Person>, ServiceError> {
        let results = self.person_repo.get_all_persons(last_id, page_size).await;
        match results {
            Ok(persons) => Ok(persons.into_iter().map(Person::from).collect()),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument]
    async fn create_person(
        &self,
        create_person_request: CreatePersonRequest,
    ) -> Result<Person, ServiceError> {
        // validate CreatePersonRequest.email
        if let Err(e) = create_person_request.validate() {
            // convert the error to a ServiceError and return it
            return Err(e.into());
        }
        let results = self.person_repo.create_person(&create_person_request).await;
        match results {
            Ok(person) => Ok(person.into()),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument]
    async fn update_person(
        &self,
        update_person_request: UpdatePersonRequest,
    ) -> Result<Person, ServiceError> {
        if let Err(e) = update_person_request.validate() {
            return Err(e.into());
        }
        let results = self.person_repo.update_person(&update_person_request).await;
        match results {
            Ok(person) => Ok(person.into()),
            Err(e) => Err(e.into()),
        }
    }

    #[instrument]
    async fn delete_person(&self, id: Uuid) -> Result<(), ServiceError> {
        let results = self.person_repo.delete_person(id).await;
        match results {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}

impl From<PersonRow> for Person {
    fn from(person_row: PersonRow) -> Self {
        Person {
            seq: person_row.id,
            id: String::from(person_row.alt_id),
            name: person_row.name,
            email: person_row.email,
            audit_info: AuditInfo {
                created_by: person_row.created_by,
                created_at: person_row.created_at,
                changed_by: person_row.last_changed_by,
                updated_at: person_row.last_update,
            },
        }
    }
}

impl From<Person> for PersonRow {
    fn from(person: Person) -> Self {
        PersonRow {
            id: person.seq,
            alt_id: string_to_uuid(person.id.as_str()),
            name: person.name,
            email: person.email,
            created_by: person.audit_info.created_by,
            created_at: person.audit_info.created_at,
            last_changed_by: person.audit_info.changed_by,
            last_update: person.audit_info.updated_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::inventory::model::{AuditInfo, CreatePersonRequest, Person, UpdatePersonRequest};
    use crate::inventory::repositories::person::{MockPersonRepository, PersonRow};
    use crate::inventory::services::person::{PersonService, PersonServiceImpl};
    use crate::inventory::services::ServiceError;
    use std::sync::{Arc, Once};
    use tracing::Level;
    use uuid::Uuid;

    static TRACING: Once = Once::new();
    pub fn init() {
        TRACING.call_once(|| {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .init();
        });
    }

    fn create_person(uuid: Uuid, seq: i32) -> Person {
        let name = format!("Test Person {}", uuid);
        let email = format!("{}@testing.com", uuid);
        Person {
            seq: seq,
            id: uuid.to_string(),
            name: name,
            email: email.clone(),
            audit_info: AuditInfo {
                created_by: "testuser".to_string(),
                created_at: chrono::Utc::now(),
                changed_by: "testuser".to_string(),
                updated_at: chrono::Utc::now(),
            },
        }
    }
    #[tokio::test]
    async fn test_get_person() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_person(uuid, seq);
        let mock_expected_results = PersonRow::from(expected_results.clone());
        mock_repo.expect_get_person_by_uuid().returning(move |_| {
            let cloned_results = mock_expected_results.clone();
            Box::pin(async move { Ok(cloned_results) })
        });
        let service = super::PersonServiceImpl::new(Arc::new(mock_repo));
        let result = service.get_person(uuid).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.id, expected_results.id);
    }

    #[tokio::test]
    async fn test_get_persons() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_person(uuid, seq);
        let mock_results = vec![PersonRow::from(expected_results.clone())];
        mock_repo.expect_get_all_persons().returning(move |_, _| {
            let cloned_results = mock_results.clone();
            Box::pin(async move { Ok(cloned_results) })
        });
        let service = PersonServiceImpl::new(Arc::new(mock_repo));
        let result = service.get_persons(None, 100).await;
        assert!(result.is_ok());
        let persons = result.unwrap();
        assert_eq!(persons.len(), 1);
        assert_eq!(persons[0].id, expected_results.id);
    }

    #[tokio::test]
    async fn test_create_person() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_person(uuid, seq);
        let mock_expected_results = PersonRow::from(expected_results.clone());
        mock_repo.expect_create_person().returning(move |_| {
            let cloned_results = mock_expected_results.clone();
            Box::pin(async move { Ok(cloned_results) })
        });
        let service = PersonServiceImpl::new(Arc::new(mock_repo));
        let request = CreatePersonRequest {
            name: expected_results.name.clone(),
            email: expected_results.email.clone(),
            created_by: "test".to_string(),
        };
        let result = service.create_person(request).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.id, expected_results.id);
    }

    #[tokio::test]
    async fn test_create_person_invalid_email() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        mock_repo.expect_create_person().never();
        let service = PersonServiceImpl::new(Arc::new(mock_repo));
        let request = CreatePersonRequest {
            name: "Test Person".to_string(),
            email: "test".to_string(),
            created_by: "test".to_string(),
        };
        let result = service.create_person(request).await;
        assert!(result.is_err());
        match result {
            Err(e) => match e {
                ServiceError::InputValidationError(_) => assert!(true),
                _ => assert!(false, "Expected InputValidationError, got {:?}", e),
            },
            _ => panic!("Expected an error"),
        }
    }

    #[tokio::test]
    async fn test_update_person() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_person(uuid, seq);
        let mock_expected_results = PersonRow::from(expected_results.clone());
        mock_repo.expect_update_person().returning(move |_| {
            let cloned_results = mock_expected_results.clone();
            Box::pin(async move { Ok(cloned_results) })
        });
        let service = PersonServiceImpl::new(Arc::new(mock_repo));
        let request = UpdatePersonRequest {
            id: expected_results.id.clone(),
            name: expected_results.name.clone(),
            email: expected_results.email.clone(),
            changed_by: "test".to_string(),
        };
        let result = service.update_person(request).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.id, expected_results.id);
    }

    #[tokio::test]
    async fn test_update_person_invalid_email() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        mock_repo.expect_update_person().never();
        let service = PersonServiceImpl::new(Arc::new(mock_repo));
        let request = UpdatePersonRequest {
            id: Uuid::new_v4().to_string(),
            name: "Test Person".to_string(),
            email: "test".to_string(),
            changed_by: "test".to_string(),
        };
        let result = service.update_person(request).await;
        assert!(result.is_err());
        match result {
            Err(e) => match e {
                ServiceError::InputValidationError(_) => assert!(true),
                _ => assert!(false, "Expected InputValidationError, got {:?}", e),
            },
            _ => panic!("Expected an error"),
        }
    }

    #[tokio::test]
    async fn test_delete_person() {
        init();
        let mut mock_repo = MockPersonRepository::new();
        let uuid = Uuid::new_v4();
        let seq = 1;
        let expected_results = create_person(uuid, seq);
        let mock_expected_results = PersonRow::from(expected_results.clone());
        mock_repo.expect_delete_person().returning(move |_| {
            let cloned_results = mock_expected_results.clone();
            Box::pin(async move { Ok(cloned_results) })
        });
        let service = PersonServiceImpl::new(Arc::new(mock_repo));
        let result = service.delete_person(uuid).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_person_from_row() {
        let row = PersonRow {
            id: 1,
            alt_id: Uuid::new_v4(),
            name: "Test Person".to_string(),
            email: "test@testing.com".to_string(),
            created_by: "testuser".to_string(),
            created_at: chrono::Utc::now(),
            last_changed_by: "testuser".to_string(),
            last_update: chrono::Utc::now(),
        };

        let person = Person::from(row.clone());
        assert_eq!(person.seq, row.id);
        assert_eq!(person.id, row.alt_id.to_string());
        assert_eq!(person.name, row.name);
        assert_eq!(person.email, row.email);
        assert_eq!(person.audit_info.created_by, row.created_by);
        assert_eq!(person.audit_info.created_at, row.created_at);
        assert_eq!(person.audit_info.changed_by, row.last_changed_by);
        assert_eq!(person.audit_info.updated_at, row.last_update);
    }

    #[tokio::test]
    async fn test_row_from_person() {
        let person = Person {
            seq: 1,
            id: Uuid::new_v4().to_string(),
            name: "Test Person".to_string(),
            email: "testing@test.com".to_string(),
            audit_info: AuditInfo {
                created_by: "testuser".to_string(),
                created_at: chrono::Utc::now(),
                changed_by: "testuser".to_string(),
                updated_at: chrono::Utc::now(),
            },
        };

        let row = PersonRow::from(person.clone());
        assert_eq!(row.id, person.seq);
        assert_eq!(row.alt_id, Uuid::parse_str(person.id.as_str()).unwrap());
        assert_eq!(row.name, person.name);
        assert_eq!(row.email, person.email);
        assert_eq!(row.created_by, person.audit_info.created_by);
        assert_eq!(row.created_at, person.audit_info.created_at);
        assert_eq!(row.last_changed_by, person.audit_info.changed_by);
        assert_eq!(row.last_update, person.audit_info.updated_at);
    }
}
