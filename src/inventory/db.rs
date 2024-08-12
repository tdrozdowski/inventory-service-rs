use std::env;
use dotenv::dotenv;

pub async fn db_pool() -> sqlx::PgPool {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    sqlx::PgPool::connect(&database_url).await.expect("Failed to create pool")
}