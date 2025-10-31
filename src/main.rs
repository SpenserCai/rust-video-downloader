mod app;
mod auth;
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

    // Handle login if requested
    if cli.needs_login() {
        return handle_login(&cli).await;
    }

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

async fn handle_login(cli: &Cli) -> Result<(), DownloaderError> {
    use auth::login::LoginManager;
    use auth::providers::BilibiliAuthProvider;
    use auth::storage::CredentialStorage;
    use utils::http::HttpClient;
    use std::sync::Arc;

    tracing::info!("Starting login process...");

    // Determine API mode
    let api_mode = cli.get_login_api_mode()
        .ok_or_else(|| DownloaderError::Config("No login mode specified".to_string()))?;

    // Create HTTP client
    let http_client = Arc::new(HttpClient::new()?);

    // Create auth provider
    let provider = Box::new(BilibiliAuthProvider::new(http_client, api_mode));

    // Create login manager
    let manager = LoginManager::new(provider);

    // Perform login
    let credentials = manager.perform_qr_login().await?;

    // Save or display credentials
    if let Some(ref config_path) = cli.config_file {
        CredentialStorage::save_to_config(&credentials, config_path)?;
        tracing::info!("✓ 凭证已保存到配置文件");
        println!("\n✓ 登录成功！凭证已保存到配置文件。");
    } else {
        tracing::info!("ℹ️  凭证仅在本次会话中有效（未指定配置文件）");
        println!("\n✓ 登录成功！");
        println!("ℹ️  提示：使用 --config-file 参数可以保存凭证以供后续使用。");
        
        // Display credentials for manual use
        if let Some(ref cookie) = credentials.cookie {
            let preview = if cookie.len() > 50 {
                format!("{}...", &cookie[..50])
            } else {
                cookie.clone()
            };
            println!("\nCookie: {}", preview);
        }
        if let Some(ref token) = credentials.access_token {
            let preview = if token.len() > 50 {
                format!("{}...", &token[..50])
            } else {
                token.clone()
            };
            println!("Access Token: {}", preview);
        }
    }

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
