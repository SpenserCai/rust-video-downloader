//! Bilibili CDN optimization module
//!
//! This module provides CDN optimization functionality specific to Bilibili platform,
//! including PCDN detection and replacement, foreign source detection, and CMCC CDN handling.

use regex::Regex;

/// Bilibili CDN optimizer
///
/// Handles CDN-related optimizations for Bilibili platform:
/// - PCDN (P2P CDN) detection and replacement
/// - Foreign source (akamaized.net) detection and replacement
/// - Force host replacement to prevent redirect to unstable CDN nodes
/// - CMCC CDN detection for special handling
///
/// # Example
///
/// ```ignore
/// let optimizer = BilibiliCdnOptimizer::new();
/// let optimized_url = optimizer.optimize_url("https://upos-sz-mirrorali.bilivideo.com:8080/video.m4s");
/// // Returns URL with backup CDN instead of PCDN
/// ```
pub struct BilibiliCdnOptimizer {
    /// List of backup CDN hosts to use when replacing PCDN or foreign sources
    backup_hosts: Vec<String>,
    /// Regex for detecting PCDN URLs (URLs with port numbers)
    pcdn_regex: Regex,
    /// Regex for detecting foreign source URLs (akamaized.net)
    foreign_source_regex: Regex,
    /// Regex for detecting CMCC CDN URLs
    cmcc_regex: Regex,
    /// Regex for replacing any host in URL (for force replacement)
    host_regex: Regex,
    /// Whether to force replace all bilivideo.com hosts with backup CDN (default: true)
    /// This prevents HTTP redirects to unstable PCDN nodes
    force_replace_host: bool,
}

impl BilibiliCdnOptimizer {
    /// Create a new CDN optimizer with default settings
    ///
    /// Default backup hosts:
    /// - upos-sz-mirrorcoso1.bilivideo.com
    /// - upos-sz-mirrorcos.bilivideo.com
    ///
    /// Force host replacement is enabled by default to prevent redirects to unstable PCDN nodes.
    pub fn new() -> Self {
        Self {
            backup_hosts: vec![
                "upos-sz-mirrorcoso1.bilivideo.com".to_string(),
                "upos-sz-mirrorcos.bilivideo.com".to_string(),
            ],
            pcdn_regex: Regex::new(r"://[^/]+:\d+/").expect("Invalid PCDN regex"),
            foreign_source_regex: Regex::new(r"://[^/]*akamaized\.net/")
                .expect("Invalid foreign source regex"),
            cmcc_regex: Regex::new(r"-cmcc").expect("Invalid CMCC regex"),
            host_regex: Regex::new(r"://[^/]+/").expect("Invalid host regex"),
            force_replace_host: true,
        }
    }

    /// Create a new CDN optimizer with custom backup hosts
    ///
    /// # Arguments
    ///
    /// * `backup_hosts` - List of backup CDN hosts to use
    pub fn with_backup_hosts(backup_hosts: Vec<String>) -> Self {
        Self {
            backup_hosts,
            pcdn_regex: Regex::new(r"://[^/]+:\d+/").expect("Invalid PCDN regex"),
            foreign_source_regex: Regex::new(r"://[^/]*akamaized\.net/")
                .expect("Invalid foreign source regex"),
            cmcc_regex: Regex::new(r"-cmcc").expect("Invalid CMCC regex"),
            host_regex: Regex::new(r"://[^/]+/").expect("Invalid host regex"),
            force_replace_host: true,
        }
    }

    /// Create a new CDN optimizer with custom settings
    ///
    /// # Arguments
    ///
    /// * `backup_hosts` - List of backup CDN hosts to use
    /// * `force_replace_host` - Whether to force replace all hosts with backup CDN
    pub fn with_config(backup_hosts: Vec<String>, force_replace_host: bool) -> Self {
        Self {
            backup_hosts,
            pcdn_regex: Regex::new(r"://[^/]+:\d+/").expect("Invalid PCDN regex"),
            foreign_source_regex: Regex::new(r"://[^/]*akamaized\.net/")
                .expect("Invalid foreign source regex"),
            cmcc_regex: Regex::new(r"-cmcc").expect("Invalid CMCC regex"),
            host_regex: Regex::new(r"://[^/]+/").expect("Invalid host regex"),
            force_replace_host,
        }
    }

    /// Optimize a download URL
    ///
    /// Performs the following optimizations in order:
    /// 1. Detects and replaces PCDN URLs (URLs with port numbers)
    /// 2. Detects and replaces foreign source URLs (akamaized.net)
    /// 3. Force replaces all bilivideo.com hosts with backup CDN (if enabled)
    ///
    /// The force replacement (step 3) is enabled by default to prevent HTTP redirects
    /// to unstable PCDN nodes, which is a common anti-restriction mechanism used by
    /// downloaders like BBDown.
    ///
    /// # Arguments
    ///
    /// * `url` - The original download URL
    ///
    /// # Returns
    ///
    /// The optimized URL. Returns the original URL if no optimization is needed.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let optimizer = BilibiliCdnOptimizer::new();
    ///
    /// // PCDN URL with port number
    /// let pcdn_url = "https://upos-sz-mirrorali.bilivideo.com:8080/video.m4s";
    /// let optimized = optimizer.optimize_url(pcdn_url);
    /// // Returns: "https://upos-sz-mirrorcoso1.bilivideo.com/video.m4s"
    ///
    /// // Foreign source URL
    /// let foreign_url = "https://cn-hk-eq-bcache-01.akamaized.net/video.m4s";
    /// let optimized = optimizer.optimize_url(foreign_url);
    /// // Returns: "https://upos-sz-mirrorcoso1.bilivideo.com/video.m4s"
    ///
    /// // Normal bilivideo.com URL (will be force replaced if enabled)
    /// let normal_url = "https://upos-sz-302ppio.bilivideo.com/video.m4s";
    /// let optimized = optimizer.optimize_url(normal_url);
    /// // Returns: "https://upos-sz-mirrorcoso1.bilivideo.com/video.m4s"
    /// ```
    pub fn optimize_url(&self, url: &str) -> String {
        let mut optimized = url.to_string();

        // Step 1: Check for PCDN (URLs with port numbers like :8080)
        if self.pcdn_regex.is_match(&optimized) {
            tracing::warn!(
                "Bilibili: Detected PCDN URL (with port number), replacing with backup CDN"
            );
            optimized = self
                .pcdn_regex
                .replace(&optimized, format!("://{}/", self.backup_hosts[0]))
                .to_string();
        }

        // Step 2: Check for foreign sources (akamaized.net)
        if self.foreign_source_regex.is_match(&optimized) {
            tracing::warn!(
                "Bilibili: Detected foreign source (akamaized.net), replacing with backup CDN"
            );
            optimized = self
                .foreign_source_regex
                .replace(&optimized, format!("://{}/", self.backup_hosts[0]))
                .to_string();
        }

        // Step 3: Force replace all bilivideo.com hosts with backup CDN
        // This prevents HTTP redirects to unstable PCDN nodes
        if self.force_replace_host && optimized.contains("bilivideo.com") {
            tracing::debug!(
                "Bilibili: Force replacing host with backup CDN to prevent PCDN redirect"
            );
            optimized = self
                .host_regex
                .replace(&optimized, format!("://{}/", self.backup_hosts[0]))
                .to_string();
        }

        optimized
    }

    /// Check if a URL is from CMCC CDN
    ///
    /// CMCC CDN requires special handling (single-threaded download) due to
    /// connection limitations.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to check
    ///
    /// # Returns
    ///
    /// `true` if the URL is from CMCC CDN, `false` otherwise
    ///
    /// # Example
    ///
    /// ```ignore
    /// let optimizer = BilibiliCdnOptimizer::new();
    /// let cmcc_url = "https://upos-sz-mirrorali-cmcc.bilivideo.com/video.m4s";
    /// assert!(optimizer.is_cmcc_cdn(cmcc_url));
    /// ```
    pub fn is_cmcc_cdn(&self, url: &str) -> bool {
        self.cmcc_regex.is_match(url)
    }
}

impl Default for BilibiliCdnOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pcdn_detection_and_replacement() {
        let optimizer = BilibiliCdnOptimizer::new();

        // Test PCDN URL with port number
        let pcdn_url = "https://upos-sz-mirrorali.bilivideo.com:8080/upgcxcode/video.m4s";
        let optimized = optimizer.optimize_url(pcdn_url);

        assert!(!optimized.contains(":8080"));
        assert!(optimized.contains("upos-sz-mirrorcoso1.bilivideo.com"));
        assert!(optimized.contains("/upgcxcode/video.m4s"));
    }

    #[test]
    fn test_foreign_source_detection_and_replacement() {
        let optimizer = BilibiliCdnOptimizer::new();

        // Test foreign source URL (akamaized.net)
        let foreign_url = "https://cn-hk-eq-bcache-01.akamaized.net/upgcxcode/video.m4s";
        let optimized = optimizer.optimize_url(foreign_url);

        assert!(!optimized.contains("akamaized.net"));
        assert!(optimized.contains("upos-sz-mirrorcoso1.bilivideo.com"));
        assert!(optimized.contains("/upgcxcode/video.m4s"));
    }

    #[test]
    fn test_cmcc_cdn_detection() {
        let optimizer = BilibiliCdnOptimizer::new();

        // Test CMCC CDN URL
        let cmcc_url = "https://upos-sz-mirrorali-cmcc.bilivideo.com/upgcxcode/video.m4s";
        assert!(optimizer.is_cmcc_cdn(cmcc_url));

        // Test non-CMCC URL
        let normal_url = "https://upos-sz-mirrorcos.bilivideo.com/upgcxcode/video.m4s";
        assert!(!optimizer.is_cmcc_cdn(normal_url));
    }

    #[test]
    fn test_force_replace_host() {
        let optimizer = BilibiliCdnOptimizer::new();

        // Test normal bilivideo.com URL (should be force replaced by default)
        let normal_url = "https://upos-sz-302ppio.bilivideo.com/upgcxcode/video.m4s";
        let optimized = optimizer.optimize_url(normal_url);

        // Should be replaced with backup host
        assert!(!optimized.contains("upos-sz-302ppio.bilivideo.com"));
        assert!(optimized.contains("upos-sz-mirrorcoso1.bilivideo.com"));
        assert!(optimized.contains("/upgcxcode/video.m4s"));
    }

    #[test]
    fn test_force_replace_host_disabled() {
        let optimizer = BilibiliCdnOptimizer::with_config(
            vec!["upos-sz-mirrorcoso1.bilivideo.com".to_string()],
            false,
        );

        // Test normal URL with force replacement disabled
        let normal_url = "https://upos-sz-mirrorcos.bilivideo.com/upgcxcode/video.m4s";
        let optimized = optimizer.optimize_url(normal_url);

        // Should remain unchanged
        assert_eq!(optimized, normal_url);
    }

    #[test]
    fn test_custom_backup_hosts() {
        let custom_hosts = vec!["custom-cdn.example.com".to_string()];
        let optimizer = BilibiliCdnOptimizer::with_backup_hosts(custom_hosts);

        let pcdn_url = "https://upos-sz-mirrorali.bilivideo.com:8080/video.m4s";
        let optimized = optimizer.optimize_url(pcdn_url);

        assert!(optimized.contains("custom-cdn.example.com"));
    }

    #[test]
    fn test_multiple_optimizations() {
        let optimizer = BilibiliCdnOptimizer::new();

        // Test URL that could match multiple patterns (though unlikely in practice)
        // This ensures the optimizer handles edge cases correctly
        let url = "https://test.akamaized.net:8080/video.m4s";
        let optimized = optimizer.optimize_url(url);

        // Should replace both PCDN and foreign source
        assert!(!optimized.contains(":8080"));
        assert!(!optimized.contains("akamaized.net"));
        assert!(optimized.contains("upos-sz-mirrorcoso1.bilivideo.com"));
    }
}
