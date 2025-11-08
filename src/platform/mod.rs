//! Platform abstraction layer
//!
//! This module defines the core `Platform` trait that all video platforms must implement,
//! along with supporting types and utilities.

pub mod metadata;
pub mod selector;
mod r#trait;

#[cfg(test)]
pub mod testing;

pub use metadata::{AuthMethod, PlatformCapabilities, PlatformMetadata};
pub use r#trait::{Platform, PlatformFeature};
pub use selector::StreamSelector;

// Platform implementations
pub mod bilibili;
