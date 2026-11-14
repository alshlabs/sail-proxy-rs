mod cli;
mod config;
mod handler_context;
mod handlers;
mod sail;
mod self_upgrade;
mod serve;

use clap::Parser;

use crate::cli::Command;
use crate::cli::Commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  match Command::parse().command {
    Commands::Serve => serve::run().await,
    Commands::Update => self_upgrade::do_update().await.map(|_| ()),
  }
}
