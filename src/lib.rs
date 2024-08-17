pub mod inventory;
pub mod test_helpers;

use crate::inventory::db::initialize_db_pool;
use sqlx::PgPool;

pub struct AppContext {
    db_pool: PgPool,
}

impl AppContext {
    pub async fn new() -> Self {
        let db_pool = initialize_db_pool().await;
        AppContext { db_pool }
    }

    fn db_pool(&self) -> &PgPool {
        &self.db_pool
    }

    // TODO - wire up all services/repositories
}