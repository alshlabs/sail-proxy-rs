mod cmd;
mod config;
mod platform;

use clap::Parser;
use clap::Subcommand;

use crate::cmd::ServeCommand;
use crate::cmd::UpgradeCommand;
use crate::cmd::serve;
use crate::cmd::upgrade;
use crate::config::Config;

#[derive(Debug, Parser)]
#[command(version)]
struct Command {
  #[clap(flatten)]
  config: Config,
  #[clap(subcommand)]
  command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
  /// Serve the proxy
  Serve(ServeCommand),
  /// Update the binary to the latest release
  #[clap(name = "update", visible_alias = "upgrade")]
  Update(UpgradeCommand),
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let Command { config, command } = Command::parse();
  match command {
    Commands::Serve(cmd) => serve::main(cmd, config).await,
    Commands::Update(cmd) => upgrade::main(cmd, config).await,
  }
}
