#[cfg(test)]
mod tests {
    use inventory_service::inventory::model::{CreatePersonRequest, UpdatePersonRequest};
    use inventory_service::inventory::repositories::person::{
        PersonRepository, PersonRepositoryImpl, PersonRow,
    };
    use inventory_service::inventory::repositories::RepoError;
    use inventory_service::test_helpers::{
        first_person_uuid, invalid_uuid, FIRST_PERSON_ID, FIRST_PERSON_UUID,
    };
    use sqlx::PgPool;
    use std::sync::Once;
    use tracing::Level;

    static TRACING: Once = Once::new();
    pub fn init() {
        TRACING.call_once(|| {
            tracing_subscriber::fmt()
                .with_max_level(Level::DEBUG)
                .init();
        });
    }
    #[sqlx::test(migrations = "./migrations", fixtures("people"))]
    async fn test_get_all_persons(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.get_all_persons(None, 10).await;
        assert!(result.is_ok());
        let people = result.unwrap();
        assert_eq!(people.len(), 10);
        // get the next page
        let result = repository.get_all_persons(Some(people[9].id), 10).await;
        assert!(result.is_ok());
        let people_page2 = result.unwrap();
        assert_eq!(people_page2.len(), 10);
        // get the final page
        let result = repository
            .get_all_persons(Some(people_page2[9].id), 10)
            .await;
        assert!(result.is_ok());
        let people_page3 = result.unwrap();
        assert_eq!(people_page3.len(), 3);
        // test there are no further pages
        let result = repository
            .get_all_persons(Some(people_page3[2].id), 10)
            .await;
        assert!(result.is_ok());
        let people_page4 = result.unwrap();
        assert_eq!(people_page4.len(), 0);
    }

    // test all functions on PersonRepositoryImpl
    #[sqlx::test(fixtures("people"))]
    async fn test_get_person_by_id(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.get_person_by_id(FIRST_PERSON_ID).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.name, "John Doe");
        assert_eq!(person.email.to_lowercase(), "john.doe@test.com");
        assert_eq!(person.alt_id, first_person_uuid());
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_create_person(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let person_request = CreatePersonRequest {
            name: "Test Person".to_string(),
            email: "test.person@test.com".to_string(),
            created_by: "testuser".to_string(),
        };
        let result = repository.create_person(&person_request).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.name, "Test Person");
        assert_eq!(person.email, "test.person@test.com");
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_update_person(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let person_request = UpdatePersonRequest {
            name: "Updated Person".to_string(),
            email: "updated.person@test.com".to_string(),
            changed_by: "testuser".to_string(),
            id: FIRST_PERSON_UUID.to_string(),
        };
        let result = repository.update_person(&person_request).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.name, "Updated Person");
        assert_eq!(person.email, "updated.person@test.com");
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_delete_person(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.delete_person(first_person_uuid()).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.name, "John Doe");
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_get_person_by_uuid(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.get_person_by_uuid(first_person_uuid()).await;
        assert!(result.is_ok());
        let person = result.unwrap();
        assert_eq!(person.name, "John Doe");
        assert_eq!(person.email.to_lowercase(), "john.doe@test.com");
        assert_eq!(person.alt_id, first_person_uuid());
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_get_person_by_uuid_not_found(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.get_person_by_uuid(invalid_uuid()).await;
        assert!(result.is_err());
        assert_not_found(result);
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_get_person_by_id_not_found(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.get_person_by_id(0).await;
        assert!(result.is_err());
        assert_not_found(result);
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_update_person_not_found(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let person_request = UpdatePersonRequest {
            name: "Updated Person".to_string(),
            email: "update.person@test.com".to_string(),
            changed_by: "testuser".to_string(),
            id: invalid_uuid().to_string(),
        };
        let result = repository.update_person(&person_request).await;
        assert!(result.is_err());
        assert_not_found(result);
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_delete_person_not_found(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let result = repository.delete_person(invalid_uuid()).await;
        assert!(result.is_err());
        assert_not_found(result);
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_update_person_invalid_uuid(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let person_request = UpdatePersonRequest {
            name: "Updated Person".to_string(),
            email: "foo@bar.com".to_string(),
            changed_by: "testuser".to_string(),
            id: "invalid-uuid".to_string(),
        };
        let result = repository.update_person(&person_request).await;
        assert!(result.is_err());
        match result {
            Ok(_) => assert!(false),
            Err(e) => match e {
                RepoError::InvalidUuid(_) => assert!(true),
                re => assert!(false, "Expected InvalidUuid, got {:?}", re),
            },
        }
    }

    #[sqlx::test(fixtures("people"))]
    async fn test_create_person_error_on_create(pool: PgPool) {
        init();
        let repository = PersonRepositoryImpl::new(pool).await;
        let person_request = CreatePersonRequest {
            name: "Test Person".to_string(),
            email: "John.Doe@test.com".to_string(), // duplicate email
            created_by: "testuser".to_string(),
        };
        let result = repository.create_person(&person_request).await;
        assert!(result.is_err());
        match result {
            Ok(_) => assert!(false),
            Err(e) => match e {
                RepoError::UniqueViolation(msg) => {
                    assert!(true, "Got UniqueViolation with message: {:?}", msg)
                }
                re => assert!(false, "Expected UniqueViolation, got {:?}", re),
            },
        }
    }

    // TODO - update to use generics so we can put into helpers
    fn assert_not_found(result: Result<PersonRow, RepoError>) {
        match result {
            Ok(_) => assert!(false),
            Err(e) => match e {
                RepoError::NotFound(_) => assert!(true),
                _ => assert!(false),
            },
        }
    }
}
