use std::sync::Arc;

use crate::platform::config::Config;

#[derive(Clone)]
pub struct HandlerContext {
  pub config: Arc<Config>,
  pub client: reqwest::Client,
}
