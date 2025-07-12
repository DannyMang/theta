use sqlx::{Pool, Postgres, PgPool};
use std::time::Duration;

pub struct DatabaseManager {
    pool: PgPool,
}

impl DatabaseManager {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = match PgPool::connect_with(
            sqlx::postgres::PgConnectOptions::new()
                .host("localhost")
                .port(5432)
                .database("theta_browser")
                .username("postgres")
                .password("postgres")
        )
        .await
        {
            Ok(pool) => pool,
            Err(_) => {
                // Fallback to provided URL
                PgPool::connect(database_url).await?
            }
        };

        Ok(Self { pool })
    }

    pub async fn from_url(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPool::connect(database_url).await?;
        Ok(Self { pool })
    }

    pub fn get_pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn test_connection(&self) -> Result<(), sqlx::Error> {
        sqlx::query("SELECT 1")
            .fetch_one(&self.pool)
            .await?;
        Ok(())
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }

    pub async fn migrate(&self) -> Result<(), sqlx::Error> {
        // Run migrations
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;
        Ok(())
    }
}

// Helper function to get database URL from environment
pub fn get_database_url() -> String {
    std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:postgres@localhost:5432/theta_browser".to_string())
}

// Helper function to initialize database connection
pub async fn initialize_database() -> Result<DatabaseManager, sqlx::Error> {
    let database_url = get_database_url();
    let manager = DatabaseManager::new(&database_url).await?;
    manager.test_connection().await?;
    Ok(manager)
} 