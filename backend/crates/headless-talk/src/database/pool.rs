use std::path::Path;

use futures::{Future, FutureExt};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use thiserror::Error;

use super::MigrationExt;

#[derive(Debug, Clone)]
pub struct DatabasePool(Pool<SqliteConnectionManager>);

impl DatabasePool {
    pub fn file(path: impl AsRef<Path>) -> Result<Self, PoolError> {
        Ok(Self(Pool::new(SqliteConnectionManager::file(path))?))
    }

    pub(crate) fn get(&self) -> Result<PooledConnection, PoolError> {
        self.0.get().map_err(PoolError)
    }

    pub(crate) fn spawn<R: Send + 'static, F: FnOnce(PooledConnection) -> PoolTaskResult<R>>(
        &self,
        closure: F,
    ) -> impl Future<Output = PoolTaskResult<R>>
    where
        F: Send + 'static,
    {
        let this = self.clone();

        tokio::task::spawn_blocking(move || {
            let connection = this.get()?;

            closure(connection)
        })
        .map(|res| res.unwrap())
    }

    pub(crate) async fn migrate_to_latest(&self) -> PoolTaskResult<()> {
        self.spawn(|mut connection| Ok(connection.migrate_to_latest()?))
            .await
    }
}

pub type PooledConnection = r2d2::PooledConnection<SqliteConnectionManager>;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct PoolError(#[from] r2d2::Error);

#[derive(Debug, Error)]
pub enum PoolTaskError {
    #[error(transparent)]
    Rusqlite(#[from] rusqlite::Error),

    #[error(transparent)]
    Pool(#[from] PoolError),

    #[error(transparent)]
    Initialization(#[from] rusqlite_migration::Error),
}

pub type PoolTaskResult<T> = Result<T, PoolTaskError>;
