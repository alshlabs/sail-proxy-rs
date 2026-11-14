use clap::Parser;
use clap::Subcommand;

#[derive(Debug, Parser)]
#[command(version)]
pub struct Command {
  #[clap(subcommand)]
  pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
  /// Serve the proxy
  Serve,
  /// Update the binary to the latest release
  Update,
}
