use clap::Parser;
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::error::Result;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    // Parse command line arguments
    let cli = Cli::parse();

    // Load configuration
    let config_path = std::path::Path::new("rvd.toml");
    let config = rvd::utils::config::Config::load(config_path)?;

    // Create orchestrator
    let orchestrator = Orchestrator::new(config, &cli)?;

    // Run the download process
    orchestrator.run(cli).await?;

    Ok(())
}
