pub mod photo_repository;
pub mod album_repository;
pub mod image_repository;

use sqlx::pool::PoolConnection;
use sqlx::PgPool;
use crate::setup::DatabaseConfig;

#[derive(Clone, Debug)]
pub struct PostgresDatabase {
    pool_connection: sqlx::Pool<sqlx::Postgres>,
}

impl PostgresDatabase {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        Ok(Self {
            pool_connection: PgPool::connect(url).await?,
        })
    }

    pub async fn connect_with_db_config(database_config: &DatabaseConfig) -> anyhow::Result<Self> {
        let url = Self::build_connection_string(database_config);
        Self::connect(&url).await
    }

    pub async fn acquire(&self) -> anyhow::Result<PoolConnection<sqlx::Postgres>> {
        Ok(self.pool_connection.acquire().await?)
    }
    
    fn build_connection_string(database_config: &DatabaseConfig) -> String {
        format!("postgres://{}:{}@{}:{}/{}", 
            database_config.username,
            database_config.password,
            database_config.host,
            database_config.port,
            database_config.name
        )
    }
}

const NULL: &'static str = "NULL";
