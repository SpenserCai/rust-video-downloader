mod app;
mod cli;
mod core;
mod error;
mod platform;
mod types;
mod utils;

use app::Orchestrator;
use clap::Parser;
use cli::Cli;
use error::DownloaderError;
use tracing_subscriber::EnvFilter;
use utils::config::Config;

#[tokio::main]
async fn main() {
    let exit_code = match run().await {
        Ok(()) => 0,
        Err(e) => {
            eprintln!("\n❌ Error: {}", e);
            1
        }
    };

    std::process::exit(exit_code);
}

async fn run() -> Result<(), DownloaderError> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Initialize logging
    init_logging(cli.verbose);

    // Load configuration
    let config = if let Some(ref config_path) = cli.config_file {
        Config::load(config_path)?
    } else {
        Config::load_default()?
    };

    // Create orchestrator
    let orchestrator = Orchestrator::new(config, &cli)?;

    // Run the download
    orchestrator.run(cli).await?;

    Ok(())
}

fn init_logging(verbose: bool) {
    let filter = if verbose {
        EnvFilter::new("rvd=debug,info")
    } else {
        EnvFilter::new("rvd=info,warn,error")
    };

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .init();
}
