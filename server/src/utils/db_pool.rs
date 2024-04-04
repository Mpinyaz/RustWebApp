use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use std::time::Duration;

pub type DatabasePool = Pool<Postgres>;

pub async fn get_database_pool(db_url: &str) -> Result<DatabasePool, sqlx::Error> {
    // set up connection pool
    tracing::trace!("Connecting to database. {}", db_url);
    match PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(db_url)
        .await
    {
        Ok(pool) => {
            tracing::info!("Database connection established");
            Ok(pool)
        }
        Err(err) => {
            tracing::error!("Failed to connect to database: {}", err);
            Err(err)
        }
    }
}
