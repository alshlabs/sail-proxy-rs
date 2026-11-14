mod internal;
mod v1_chat_completions_get;

pub use self::v1_chat_completions_get::v1_chat_completions_post;

#[derive(Clone)]
pub struct HandlerContext {
  pub sail_api_base: String,
  pub client: reqwest::Client,
}
