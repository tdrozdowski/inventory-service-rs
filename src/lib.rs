pub mod inventory;
pub mod test_helpers;

use crate::inventory::db::initialize_db_pool;
use crate::inventory::repositories::person::PersonRepositoryImpl;
use crate::inventory::services::person::{PersonService, PersonServiceImpl};
use sqlx::PgPool;
use std::sync::Arc;

pub struct AppContext {
    pub person_service: Arc<dyn PersonService + Send + 'static>,
}

impl AppContext {
    pub async fn new() -> Self {
        let db_pool = initialize_db_pool().await;
        let person_service = Self::init_person_service(&db_pool).await;
        AppContext { person_service }
    }

    async fn init_person_service(db_pool: &PgPool) -> Arc<dyn PersonService> {
        let person_repo = PersonRepositoryImpl::new(db_pool.clone()).await;
        Arc::new(PersonServiceImpl::new(Arc::new(person_repo)))
    }
}
