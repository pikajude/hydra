use super::*;

pub struct GithubStatus;

#[async_trait]
impl Plugin for GithubStatus {
  fn name(&self) -> &'static str {
    std::any::type_name::<Self>()
  }
}

#[async_trait]
impl PluginInit for GithubStatus {
  async fn init(_: &Hydra) -> Result<Option<Arc<dyn Plugin>>> {
    Ok(Some(Arc::new(Self)))
  }
}
