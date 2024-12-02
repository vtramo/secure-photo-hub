pub mod photo_repository;
pub mod album_repository;

use sqlx::pool::PoolConnection;
use sqlx::PgPool;

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

    pub async fn acquire(&self) -> anyhow::Result<PoolConnection<sqlx::Postgres>> {
        Ok(self.pool_connection.acquire().await?)
    }
}
