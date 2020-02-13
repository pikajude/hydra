use super::models::Build;
use super::Result;

#[async_trait]
pub trait Plugin: Send + Sync {
  async fn build_finished(&self, build: &Build, dependents: &[Build]) -> Result<()>;
}
