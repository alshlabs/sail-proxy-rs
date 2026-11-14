use uhttp::Request;
use uhttp::Response;
use uhttp::Router;

use crate::cmd::serve::handlers::v1_chat_completions_post;
use crate::platform::config::Config;
use crate::platform::handler_context::HandlerContext;
use crate::platform::self_upgrade;

#[derive(Debug, clap::Args)]
pub struct ServeCommand {}

pub async fn main(_cmd: ServeCommand) -> anyhow::Result<()> {
  notify_update_available().await;

  let config = Config::parse()?;

  let ctx = HandlerContext {
    config: std::sync::Arc::new(config.clone()),
    client: reqwest::Client::new(),
  };

  let mut app = Router::<HandlerContext>::new(ctx).with(uhttp::middleware::logger_default);

  app.any(
    "/health",
    async |_req: Request, res: Response, _ctx: HandlerContext| res.status(200).empty(),
  );
  app.post("/v1/chat/completions", v1_chat_completions_post);

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
