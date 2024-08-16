pub mod inventory;

use dotenv::dotenv;
use sqlx::PgPool;
use std::env;
use crate::inventory::db::initialize_db_pool;

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