mod internal;
mod v1_chat_completions_get;

use std::sync::Arc;

pub use self::v1_chat_completions_get::v1_chat_completions_post;
use crate::config::Config;

#[derive(Clone)]
pub struct HandlerContext {
  pub config: Arc<Config>,
  pub client: reqwest::Client,
}
