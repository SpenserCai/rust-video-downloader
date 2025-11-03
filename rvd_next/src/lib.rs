//! RVD Next - A modular video downloader written in Rust
//!
//! This library provides a flexible, extensible architecture for downloading videos
//! from multiple platforms. The core design is based on a trait-based plugin system
//! that allows easy addition of new video platforms.
//!
//! # Architecture
//!
//! - **Platform Abstraction Layer**: Defines the `Platform` trait that all video platforms must implement
//! - **Application Layer**: Orchestrates the download process and manages platform registry
//! - **Core Layer**: Provides downloading, muxing, and progress tracking functionality
//! - **Utility Layer**: HTTP client, configuration management, and file utilities
//!
//! # Example
//!
//! ```no_run
//! use rvd::platform::Platform;
//! use rvd::app::PlatformRegistry;
//!
//! // Create a platform registry
//! let mut registry = PlatformRegistry::new();
//!
//! // Register platforms
//! // registry.register(bilibili_platform);
//! // registry.register(youtube_platform);
//!
//! // Select platform for a URL
//! // let platform = registry.select_platform("https://www.bilibili.com/video/BV1xx411c7mD")?;
//! ```

pub mod app;
pub mod auth;
pub mod cli;
pub mod core;
pub mod error;
pub mod platform;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use error::{DownloaderError, Result};
pub use types::{Auth, BatchResult, Stream, StreamContext, VideoInfo};
