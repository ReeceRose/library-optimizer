use sqlx::{migrate::Migrator, SqlitePool};
use std::path::Path;
use tracing::{error, info};

/// Runs the migrations located in the `./migrations` folder.
/// This function is called after the connection pool has been created.
pub async fn run_migrations(pool: &SqlitePool) -> bool {
    let migrations_path = Path::new("./migrations");
    let migrator = Migrator::new(migrations_path).await.unwrap();

    match migrator.run(pool).await {
        Ok(_) => info!("Database migrations completed successfully."),
        Err(err) => {
            error!("Failed to run migrations: {}", err);
            return false;
        }
    }

    true
}
