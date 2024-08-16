use dotenv::dotenv;
use sqlx::PgPool;
use std::env;

/// Helper function to initialize the database connection pool.
/// Reads the DATABASE_URL environment variable to determine the connection string.
/// Returns a new connection pool.
/// # Returns
/// A new connection pool.
/// # Panics
/// Panics if the DATABASE_URL environment variable is not set.
/// Panics if the connection pool cannot be created.
pub async fn initialize_db_pool() -> PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPool::connect(&database_url).await.expect("Failed to create pool")
}
