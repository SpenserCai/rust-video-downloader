// Library entry point for rvd
// This allows tests and other crates to use rvd as a library

pub mod app;
pub mod cli;
pub mod core;
pub mod error;
pub mod platform;
pub mod types;
pub mod utils;

// Re-export commonly used types
pub use error::{DownloaderError, Result};
