use std::sync::Arc;

use uhttp::Router;

use crate::config;
use crate::handler_context::HandlerContext;
use crate::handlers;
use crate::self_upgrade;

pub async fn run() -> anyhow::Result<()> {
  notify_update_available().await;

  let config = Arc::new(config::Config::parse()?);

  let ctx = HandlerContext {
    config: config.clone(),
    client: reqwest::Client::new(),
  };

  let mut app = Router::<HandlerContext>::new(ctx).with(uhttp::middleware::logger_default);

  app.any("/health", handlers::health_any);
  app.post("/v1/chat/completions", handlers::v1_chat_completions_post);

  println!("Serving on {}", config.address);
  uhttp::Server::builder()
    .handler(app.handler())
    .listen(&config.address)
    .await?;

  Ok(())
}

async fn notify_update_available() {
  let options = self_upgrade::UpgradeOptions {
    target_repo: self_upgrade::TARGET_REPO,
    current_version: self_upgrade::CURRENT_VERSION,
  };

  match self_upgrade::check_for_update(&options).await {
    Ok(Some(new_version)) => println!(
      "⚡ Update available: {} -> {}  (run `sail-proxy-rs update`)",
      self_upgrade::CURRENT_VERSION,
      new_version
    ),
    Ok(None) => {}
    Err(err) => eprintln!("Update check failed: {err}"),
  }
}
