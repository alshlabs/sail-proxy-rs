use crate::config::Config;
use crate::platform::self_upgrade;

#[derive(Debug, clap::Args)]
pub struct UpgradeCommand {}

pub async fn main(_cmd: UpgradeCommand) -> anyhow::Result<()> {
  self_upgrade::do_update().await.map(|_| ())
}
