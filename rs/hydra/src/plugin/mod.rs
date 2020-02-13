dirmod::all!(default pub);

use super::models::{Build, BuildStep};
use super::{Hydra, Result};
use std::path::Path;
use std::sync::Arc;

#[allow(unused_variables)] // for rustdoc
#[async_trait]
pub trait Plugin: Send + Sync {
  async fn build_started(&self, build: &Build) -> Result<()> {
    Ok(())
  }
  async fn build_finished(&self, build: &Build, dependents: &[Build]) -> Result<()> {
    Ok(())
  }
  async fn step_finished(&self, step: &BuildStep, log_path: &Path) -> Result<()> {
    Ok(())
  }

  fn name(&self) -> &'static str;
}

#[async_trait]
pub trait PluginInit: Sized + Plugin {
  async fn init(hydra: &Hydra) -> Result<Option<Arc<dyn Plugin>>>;
}
