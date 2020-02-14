#[macro_use]
extern crate async_trait;
#[macro_use]
extern crate log;

pub extern crate db;

pub mod plugin;

use db::diesel::{
  r2d2::{ConnectionManager, Pool, PooledConnection},
  PgConnection,
};
pub use db::{diesel, models, schema};
use futures::stream::{FuturesUnordered, StreamExt, TryStreamExt};
use plugin::*;
use serde::de::DeserializeOwned;
use std::iter::FromIterator;
use std::sync::Arc;
pub use toml::{value::Table, Value};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Hydra {
  plugins: Vec<Arc<dyn Plugin>>,
  db: Pool<ConnectionManager<PgConnection>>,
  raw_config: Table,
}

impl Hydra {
  pub async fn get() -> Result<Self> {
    let pool = Pool::new(ConnectionManager::new("host=/run/postgresql user=hydra"))?;
    let mut hydra = Self {
      plugins: vec![],
      db: pool,
      raw_config: toml::from_str(&std::fs::read_to_string(std::env::var("HYDRA_RS_CONFIG")?)?)?,
    };
    hydra.plugins = FuturesUnordered::from_iter(vec![plugin::notify::InfluxDB::init(&hydra)])
      .filter_map(|x| async { x.transpose() })
      .try_collect::<Vec<_>>()
      .await?;
    Ok(hydra)
  }

  pub fn plugins(&self) -> &[Arc<dyn Plugin>] {
    &self.plugins
  }

  pub fn add_plugin<P: Plugin + 'static>(&mut self, p: P) {
    self.plugins.push(Arc::new(p))
  }

  pub fn conn(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>> {
    self.db.get().map_err(Into::into)
  }

  pub fn config(&self) -> &Table {
    &self.raw_config
  }

  pub fn sub_config<T: DeserializeOwned, S: AsRef<str>>(&self, key: S) -> Option<T> {
    self.config().get(key.as_ref()).and_then(|x| {
      x.clone()
        .try_into::<T>()
        .map_err(|e| info!("Unable to parse sub-config: {}", e))
        .ok()
    })
  }
}

#[test]
fn assert_sendable() {
  use assert_impl::assert_impl;

  assert_impl!(Send: Hydra);
  assert_impl!(Sync: Hydra);
}
