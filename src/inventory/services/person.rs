use crate::inventory::model::{AuditInfo, CreatePersonRequest, Person, UpdatePersonRequest};
use crate::inventory::repositories::person::{PersonRepository, PersonRow};
use crate::inventory::services::ServiceError;
use async_trait::async_trait;
use garde::Validate;
use std::fmt::Debug;
use std::sync::Arc;
use uuid::Uuid;

#[async_trait]
trait PersonService: Sync + Send + Debug + 'static {
    async fn get_person(&self, id: Uuid) -> Result<Person, ServiceError>;
    async fn get_persons(&self) -> Result<Vec<Person>, ServiceError>;
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

    async fn get_persons(&self) -> Result<Vec<Person>, ServiceError> {
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
