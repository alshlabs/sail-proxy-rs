#[derive(Debug, Clone, clap::Args)]
pub struct Config {
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
