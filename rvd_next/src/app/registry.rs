//! Platform registry - manages registered platforms
//!
//! This module provides the `PlatformRegistry` which manages all registered video platforms
//! and provides intelligent platform selection based on URLs.

use crate::error::{DownloaderError, Result};
use crate::platform::Platform;
use std::collections::HashMap;
use std::sync::Arc;

/// Platform registry
///
/// Manages all registered platforms and provides intelligent platform selection.
/// Uses dual indexing (domain index + pattern index) for fast URL matching.
///
/// # Example
///
/// ```no_run
/// use std::sync::Arc;
/// use rvd::app::PlatformRegistry;
/// // use rvd::platform::bilibili::BilibiliPlatform;
///
/// let mut registry = PlatformRegistry::new();
///
/// // Register platforms
/// // let bilibili = Arc::new(BilibiliPlatform::new(Default::default()).unwrap());
/// // registry.register(bilibili);
///
/// // Select platform for a URL
/// // let platform = registry.select_platform("https://www.bilibili.com/video/BV1xx411c7mD")?;
/// // println!("Selected platform: {}", platform.name());
/// ```
pub struct PlatformRegistry {
    /// All registered platforms
    platforms: Vec<Arc<dyn Platform>>,

    /// Domain index - maps domain names to platforms
    ///
    /// Example: "bilibili.com" -> [BilibiliPlatform]
    domain_index: HashMap<String, Vec<Arc<dyn Platform>>>,

    /// Pattern index - maps special patterns to platforms
    ///
    /// Example: "BV" -> [BilibiliPlatform] (for BV号)
    pattern_index: HashMap<String, Vec<Arc<dyn Platform>>>,
}

impl PlatformRegistry {
    /// Create a new empty platform registry
    pub fn new() -> Self {
        Self {
            platforms: Vec::new(),
            domain_index: HashMap::new(),
            pattern_index: HashMap::new(),
        }
    }

    /// Register a platform
    ///
    /// Adds a platform to the registry and builds indexes for fast URL matching.
    ///
    /// # Arguments
    ///
    /// * `platform` - The platform to register
    ///
    /// # Example
    ///
    /// ```no_run
    /// use std::sync::Arc;
    /// use rvd::app::PlatformRegistry;
    /// // use rvd::platform::bilibili::BilibiliPlatform;
    ///
    /// let mut registry = PlatformRegistry::new();
    /// // let bilibili = Arc::new(BilibiliPlatform::new(Default::default()).unwrap());
    /// // registry.register(bilibili);
    /// ```
    pub fn register(&mut self, platform: Arc<dyn Platform>) {
        tracing::info!("Registering platform: {}", platform.name());

        // Build indexes for fast URL matching
        for pattern in &platform.metadata().url_patterns {
            if let Some(stripped) = pattern.strip_prefix('^') {
                // Regex pattern (e.g., ^BV for BV号)
                let key = stripped.to_string();
                tracing::debug!("  Adding pattern index: {} -> {}", key, platform.name());
                self.pattern_index
                    .entry(key)
                    .or_default()
                    .push(platform.clone());
            } else {
                // Domain pattern (e.g., bilibili.com)
                tracing::debug!("  Adding domain index: {} -> {}", pattern, platform.name());
                self.domain_index
                    .entry(pattern.to_string())
                    .or_default()
                    .push(platform.clone());
            }
        }

        self.platforms.push(platform);
    }

    /// Select a platform for the given URL
    ///
    /// Uses a three-tier matching strategy:
    /// 1. Pattern matching (e.g., BV号, av号)
    /// 2. Domain index lookup
    /// 3. Full scan fallback
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to match
    ///
    /// # Returns
    ///
    /// The selected platform
    ///
    /// # Errors
    ///
    /// Returns an error if no platform can handle the URL.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use rvd::app::PlatformRegistry;
    ///
    /// let registry = PlatformRegistry::new();
    /// // let platform = registry.select_platform("https://www.bilibili.com/video/BV1xx411c7mD")?;
    /// ```
    pub fn select_platform(&self, url: &str) -> Result<Arc<dyn Platform>> {
        // Strategy 1: Try pattern matching (e.g., BV号)
        for (pattern, candidates) in &self.pattern_index {
            if url.starts_with(pattern) {
                for platform in candidates {
                    if platform.can_handle(url) {
                        tracing::debug!(
                            "Selected platform: {} for URL: {} (via pattern index: {})",
                            platform.name(),
                            url,
                            pattern
                        );
                        return Ok(platform.clone());
                    }
                }
            }
        }

        // Strategy 2: Try domain index lookup
        if let Some(domain) = extract_domain(url) {
            if let Some(candidates) = self.domain_index.get(&domain) {
                for platform in candidates {
                    if platform.can_handle(url) {
                        tracing::debug!(
                            "Selected platform: {} for URL: {} (via domain index: {})",
                            platform.name(),
                            url,
                            domain
                        );
                        return Ok(platform.clone());
                    }
                }
            }
        }

        // Strategy 3: Full scan fallback
        for platform in &self.platforms {
            if platform.can_handle(url) {
                tracing::debug!(
                    "Selected platform: {} for URL: {} (via full scan)",
                    platform.name(),
                    url
                );
                return Ok(platform.clone());
            }
        }

        Err(DownloaderError::UnsupportedPlatform(format!(
            "No platform can handle URL: {}",
            url
        )))
    }

    /// List all registered platforms
    ///
    /// Returns a vector of references to all registered platforms.
    pub fn list_platforms(&self) -> Vec<&dyn Platform> {
        self.platforms.iter().map(|p| p.as_ref()).collect()
    }

    /// Get a platform by name
    ///
    /// # Arguments
    ///
    /// * `name` - The platform name (e.g., "bilibili", "youtube")
    ///
    /// # Returns
    ///
    /// The platform if found, `None` otherwise
    pub fn get_platform(&self, name: &str) -> Option<Arc<dyn Platform>> {
        self.platforms.iter().find(|p| p.name() == name).cloned()
    }

    /// Get the number of registered platforms
    pub fn count(&self) -> usize {
        self.platforms.len()
    }
}

impl Default for PlatformRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Extract domain from URL
///
/// Extracts the base domain (e.g., "bilibili.com") from a full URL.
///
/// # Arguments
///
/// * `url` - The URL to extract from
///
/// # Returns
///
/// The domain if successfully extracted, `None` otherwise
fn extract_domain(url: &str) -> Option<String> {
    if url.starts_with("http://") || url.starts_with("https://") {
        if let Ok(parsed) = url::Url::parse(url) {
            if let Some(host) = parsed.host_str() {
                let parts: Vec<&str> = host.split('.').collect();
                if parts.len() >= 2 {
                    // Extract base domain (e.g., "bilibili.com" from "www.bilibili.com")
                    return Some(format!(
                        "{}.{}",
                        parts[parts.len() - 2],
                        parts[parts.len() - 1]
                    ));
                }
                return Some(host.to_string());
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://www.bilibili.com/video/BV1xx411c7mD"),
            Some("bilibili.com".to_string())
        );
        assert_eq!(
            extract_domain("https://space.bilibili.com/123456"),
            Some("bilibili.com".to_string())
        );
        assert_eq!(
            extract_domain("https://youtube.com/watch?v=xxx"),
            Some("youtube.com".to_string())
        );
        assert_eq!(extract_domain("BV1xx411c7mD"), None);
        assert_eq!(extract_domain("not a url"), None);
    }
}
