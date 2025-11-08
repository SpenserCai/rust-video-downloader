use clap::Parser;
use rvd::app::Orchestrator;
use rvd::cli::Cli;
use rvd::error::DownloaderError;
use rvd::utils::config::Config;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Initialize console with UTF-8 support on Windows
    let _console_guard = rvd::utils::console::ConsoleGuard::new();

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

    // Handle login if requested and get credentials
    let login_auth = if cli.needs_login() {
        Some(handle_login(&cli).await?)
    } else {
        None
    };

    // If login was performed without a URL, just exit successfully
    if cli.needs_login() && cli.url.is_none() {
        return Ok(());
    }

    // Load configuration
    let config = if let Some(ref config_path) = cli.config_file {
        Config::load(config_path)?
    } else {
        Config::load_default()?
    };

    // Create orchestrator with login auth if available
    let mut orchestrator = Orchestrator::new(config, &cli)?;

    // If we have login auth, it takes priority
    if let Some(auth) = login_auth {
        orchestrator.set_auth(Some(auth));
    }

    // Run the download
    orchestrator.run(cli).await?;

    Ok(())
}

async fn handle_login(cli: &Cli) -> Result<rvd::types::Auth, DownloaderError> {
    use rvd::app::PlatformRegistry;
    use rvd::auth::login::LoginManager;
    use rvd::auth::storage::CredentialStorage;

    // Check for deprecated login args
    cli.check_deprecated_login_args();

    tracing::info!("Starting login process...");

    // Create platform registry to detect platform
    let mut registry = PlatformRegistry::new();
    
    // Register platforms (same as in Orchestrator)
    let api_mode = cli.get_api_mode();
    let bilibili = rvd::platform::bilibili::BilibiliPlatform::new(api_mode)?;
    registry.register(std::sync::Arc::new(bilibili));
    
    // Future: Register more platforms here
    // let youtube = Arc::new(rvd::platform::youtube::YouTubePlatform::new()?);
    // registry.register(youtube);

    // Determine platform
    let platform = if let Some(platform_name) = cli.get_login_platform() {
        // Platform-specific login flag used
        // If URL is also provided, validate it matches the platform
        if let Some(ref url) = cli.url {
            let detected_platform = registry.select_platform(url)?;
            if detected_platform.name() != platform_name {
                return Err(DownloaderError::InvalidArgument(format!(
                    "Platform mismatch: login flag is for '{}' but URL is for '{}'\n\
                    URL: {}\n\
                    Hint: Either remove the URL for login-only mode, or use a {} URL.",
                    platform_name, detected_platform.name(), url, platform_name
                )));
            }
            detected_platform
        } else {
            // Login-only mode, get platform by name
            registry.get_platform(platform_name)
                .ok_or_else(|| DownloaderError::InvalidArgument(format!(
                    "Platform '{}' not found or not registered",
                    platform_name
                )))?
        }
    } else if let Some(ref url) = cli.url {
        // Auto-detect platform from URL
        registry.select_platform(url)?
    } else {
        return Err(DownloaderError::InvalidArgument(
            "Login requires either a platform-specific flag (e.g., --login-bilibili-web) or a URL to detect the platform.\n\
            Examples:\n\
            - rvd --login-bilibili-web\n\
            - rvd --login-bilibili-tv\n\
            - rvd --login-qrcode https://www.bilibili.com/video/BV1xx411c7mD".to_string()
        ));
    };

    // Validate CLI args for this platform
    platform.validate_cli_args(cli)?;

    // Create auth provider from platform
    let provider = platform.create_auth_provider(cli)?;

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
    }

    // Convert credentials to Auth and return
    Ok(CredentialStorage::to_auth(&credentials))
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
