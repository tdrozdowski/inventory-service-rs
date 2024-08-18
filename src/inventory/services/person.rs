use crate::inventory::model::{AuditInfo, CreatePersonRequest, Person, UpdatePersonRequest};
use crate::inventory::repositories::person::{PersonRepository, PersonRow};
use crate::inventory::services::ServiceError;
use crate::test_helpers::string_to_uuid;
use async_trait::async_trait;
use garde::Validate;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
trait PersonService: Sync + Send + Debug + 'static {
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
struct PersonServiceImpl {
    person_repo: Arc<dyn PersonRepository + Send + Sync>,
}

impl PersonServiceImpl {
    fn new(person_repo: Arc<dyn PersonRepository + Send + Sync>) -> PersonServiceImpl {
        PersonServiceImpl { person_repo }
    }
}

#[async_trait]
impl PersonService for PersonServiceImpl {
    async fn get_person(&self, id: Uuid) -> Result<Person, ServiceError> {
        let results = self.person_repo.get_person_by_uuid(id).await;
        match results {
            Ok(person) => Ok(person.into()),
            Err(e) => Err(e.into()),
        }
    }

    async fn get_persons(
        &self,
        last_id: Option<i32>,
        page_size: i64,
    ) -> Result<Vec<Person>, ServiceError> {
        let results = self.person_repo.get_all_persons(None, 100).await;
        match results {
            Ok(persons) => Ok(persons.into_iter().map(Person::from).collect()),
            Err(e) => Err(e.into()),
        }
    }

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

    async fn update_person(
        &self,
        update_person_request: UpdatePersonRequest,
    ) -> Result<Person, ServiceError> {
        let results = self.person_repo.update_person(&update_person_request).await;
        match results {
            Ok(person) => Ok(person.into()),
            Err(e) => Err(e.into()),
        }
    }

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
    use crate::inventory::model::{AuditInfo, Person};
    use crate::inventory::repositories::person::{MockPersonRepository, PersonRow};
    use crate::inventory::services::person::{PersonService, PersonServiceImpl};
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
}
