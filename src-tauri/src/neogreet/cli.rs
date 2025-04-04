#[derive(Debug, clap::Parser)]
pub struct Cli {
  /// Default user to login
  #[arg(short, long)]
  pub user: Option<String>,
}
