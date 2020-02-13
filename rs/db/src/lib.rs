#[macro_use]
pub extern crate diesel;

use diesel::r2d2::{ConnectionManager, Pool, PoolError};
use diesel::PgConnection;

pub mod models;
pub mod schema;

pub fn connect<S: Into<String>>(
  uri: S,
) -> Result<Pool<ConnectionManager<PgConnection>>, PoolError> {
  Pool::new(ConnectionManager::new(uri))
}
