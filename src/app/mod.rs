//! Application layer
//!
//! This module contains the orchestrator and platform registry that coordinate
//! the download process.

pub mod orchestrator;
pub mod registry;

pub use orchestrator::Orchestrator;
pub use registry::PlatformRegistry;
