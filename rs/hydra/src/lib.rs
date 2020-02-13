#[macro_use]
extern crate async_trait;

pub extern crate db;

pub mod plugin;

use db::diesel::{
  r2d2::{ConnectionManager, Pool, PooledConnection},
  PgConnection,
};
pub use db::{diesel, models, schema};
use futures::stream::{FuturesUnordered, StreamExt, TryStreamExt};
use plugin::*;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::Arc;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub struct Hydra {
  plugins: Vec<Arc<dyn Plugin>>,
  db: Pool<ConnectionManager<PgConnection>>,
  config: HashMap<String, String>,
}

impl Hydra {
  pub async fn get() -> Result<Self> {
    let pool = Pool::new(ConnectionManager::new("host=/run/postgresql user=hydra"))?;
    let mut hydra = Self {
      plugins: vec![],
      db: pool,
      config: Default::default(),
    };
    hydra.plugins =
      FuturesUnordered::from_iter(vec![plugin::github_status::GithubStatus::init(&hydra)])
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

  pub fn config(&self) -> &HashMap<String, String> {
    &self.config
  }
}

#[test]
fn assert_sendable() {
  use assert_impl::assert_impl;

  assert_impl!(Send: Hydra);
  assert_impl!(Sync: Hydra);
}
