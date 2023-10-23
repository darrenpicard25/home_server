use std::{str::FromStr, time::Duration};

use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    ConnectOptions, Pool, Postgres,
};

use crate::error::ServiceStartupError;

pub mod repositories;

#[derive(Clone)]
pub struct Database {
    pool: Pool<Postgres>,
}

// Constructor
impl Database {
    pub async fn new(connection_string: &str) -> Result<Self, ServiceStartupError> {
        let options = PgConnectOptions::from_str(connection_string)
            .map_err(|e| {
                log::error!("{e}");

                ServiceStartupError::DatabaseConnection(e.to_string())
            })?
            .log_statements(log::LevelFilter::Debug)
            .log_slow_statements(log::LevelFilter::Warn, Duration::from_millis(500));

        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect_with(options)
            .await
            .map_err(|e| ServiceStartupError::DatabaseConnection(e.to_string()))?;

        sqlx::migrate!().run(&pool).await.map_err(|e| {
            log::error!("{e}");
            ServiceStartupError::DatabaseMigration
        })?;

        Ok(Self { pool })
    }
}

// Methods
impl Database {
    pub fn pool(&self) -> Pool<Postgres> {
        self.pool.clone()
    }
}
