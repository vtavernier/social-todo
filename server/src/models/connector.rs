use redis::AsyncCommands;
use serde::{de::DeserializeOwned, ser::Serialize};
use tracing::Level;

#[derive(Clone)]
pub struct Connector {
    pub pg_pool: sqlx::PgPool,
    pub redis_pool: Option<redis::aio::MultiplexedConnection>,
}

#[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
pub enum CacheError {
    #[display("caching disabled")]
    Disabled,
    #[display("redis error: {0}")]
    Redis(#[error(source)] redis::RedisError),
    #[display("serialization error: {0}")]
    Bincode(#[error(source)] Box<bincode::ErrorKind>),
}

impl Connector {
    fn has_cache(&self) -> bool {
        self.redis_pool.is_some()
    }

    async fn get_cache_conn(&self) -> Result<redis::aio::MultiplexedConnection, CacheError> {
        Ok(self
            .redis_pool
            .as_ref()
            .ok_or_else(|| CacheError::Disabled)?
            .clone())
    }

    async fn get_cached<T: DeserializeOwned>(&self, key: &str) -> Result<T, CacheError> {
        // Get a connection to Redis
        let mut conn = self.get_cache_conn().await?;

        // Fetch the value for the given key
        let obj: Vec<u8> = conn.get(key).await?;

        // Deserialize
        Ok(bincode::deserialize(&obj)?)
    }

    async fn set_cache<T: Serialize>(&self, key: &str, obj: &T) -> Result<(), CacheError> {
        // Get a connection to Redis
        let mut conn = self.get_cache_conn().await?;

        let obj: Vec<u8> = bincode::serialize(obj)?;

        conn.set::<_, Vec<u8>, _>(key, obj).await?;

        Ok(())
    }

    pub async fn cached<'a, T: Serialize + DeserializeOwned, E, F>(
        &'a self,
        key: &str,
        else_fn: impl FnOnce(&'a sqlx::PgPool) -> F,
    ) -> Result<T, E>
    where
        F: futures::Future<Output = Result<T, E>>,
    {
        if !self.has_cache() {
            return Ok(else_fn(&self.pg_pool).await?);
        }

        let span = debug_span!("cached", key);

        match self.get_cached(key).await {
            Ok(cached) => {
                event!(parent: &span, Level::DEBUG, "cache hit");
                Ok(cached)
            }
            Err(error) => {
                event!(parent: &span, Level::DEBUG, %error, "cache miss");

                let result = else_fn(&self.pg_pool).await?;

                if let Err(error) = self.set_cache(key, &result).await {
                    event!(parent: &span, Level::ERROR, %error, "cache set error");
                }

                Ok(result)
            }
        }
    }
}
