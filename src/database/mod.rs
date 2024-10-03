mod migrations;
mod pool;

use sqlx::SqlitePool;
use tracing::info;

pub struct Database {
    pool: SqlitePool,
}

impl Database {
    /// Creates a new `Database` struct by connecting to the database and running migrations.
    pub async fn new(db_url: &str) -> Self {
        // Create the connection pool
        let pool = pool::create_pool(db_url).await;

        // Run migrations
        migrations::run_migrations(&pool).await;

        info!("Database initialized successfully.");
        Self { pool }
    }

    /// Get a reference to the connection pool
    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
}
