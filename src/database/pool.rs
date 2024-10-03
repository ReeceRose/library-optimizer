use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use tracing::{error, info};

/// Creates and returns a database connection pool.
/// Ensures the database exists before creating the pool.
pub async fn create_pool(db_url: &str) -> SqlitePool {
    if !Sqlite::database_exists(db_url).await.unwrap_or(false) {
        println!("Creating database {}", db_url);
        match Sqlite::create_database(db_url).await {
            Ok(_) => info!("Create db success"),
            Err(error) => error!("error: {}", error),
        }
    } else {
        info!("Database already exists");
    }

    // Create and return the connection pool
    let pool = SqlitePool::connect(db_url).await.unwrap();
    info!("Database connection pool created.");
    pool
}
