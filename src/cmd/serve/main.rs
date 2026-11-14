use uhttp::Request;
use uhttp::Response;
use uhttp::Router;

use super::handlers::HandlerContext;
use super::handlers::v1_chat_completions_post;
use crate::platform::self_upgrade;

#[derive(Debug, Clone, clap::Args)]
pub struct ServeCommand {
  /// Base URL of the Sail API
  #[arg(
    long,
    env = "SAIL_API_BASE",
    default_value = "https://api.sailresearch.com/v1"
  )]
  pub sail_api_base: String,

  /// Address to listen on
  #[arg(long, env = "SAIL_PROXY_ADDRESS", default_value = "0.0.0.0:9090")]
  pub address: String,
}

pub async fn main(cmd: ServeCommand) -> anyhow::Result<()> {
  notify_update_available().await;

  let ctx = HandlerContext {
    client: reqwest::Client::new(),
    sail_api_base: cmd.sail_api_base.clone(),
  };

  let mut app = Router::<HandlerContext>::new(ctx).with(uhttp::middleware::logger_default);

  app.any(
    "/health",
    async |_req: Request, res: Response, _ctx: HandlerContext| res.status(200).empty(),
  );
  app.post("/v1/chat/completions", v1_chat_completions_post);

  println!("Serving on {}", cmd.address);
  uhttp::Server::builder()
    .handler(app.handler())
    .listen(&cmd.address)
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
