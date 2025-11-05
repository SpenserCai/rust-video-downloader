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
}

impl BilibiliCdnOptimizer {
    /// Create a new CDN optimizer with default settings
    ///
    /// Default backup hosts:
    /// - upos-sz-mirrorcoso1.bilivideo.com
    /// - upos-sz-mirrorcos.bilivideo.com
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
        }
    }

    /// Optimize a download URL
    ///
    /// Performs the following optimizations:
    /// 1. Detects and replaces PCDN URLs (URLs with port numbers)
    /// 2. Detects and replaces foreign source URLs (akamaized.net)
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
    /// // Normal URL (no optimization needed)
    /// let normal_url = "https://upos-sz-mirrorcos.bilivideo.com/video.m4s";
    /// let optimized = optimizer.optimize_url(normal_url);
    /// // Returns: original URL unchanged
    /// ```
    pub fn optimize_url(&self, url: &str) -> String {
        let mut optimized = url.to_string();

        // Check for PCDN (URLs with port numbers like :8080)
        if self.pcdn_regex.is_match(&optimized) {
            tracing::warn!(
                "Bilibili: Detected PCDN URL (with port number), replacing with backup CDN"
            );
            optimized = self
                .pcdn_regex
                .replace(&optimized, format!("://{}/", self.backup_hosts[0]))
                .to_string();
        }

        // Check for foreign sources (akamaized.net)
        if self.foreign_source_regex.is_match(&optimized) {
            tracing::warn!(
                "Bilibili: Detected foreign source (akamaized.net), replacing with backup CDN"
            );
            optimized = self
                .foreign_source_regex
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
    fn test_normal_url_unchanged() {
        let optimizer = BilibiliCdnOptimizer::new();

        // Test normal URL (should not be modified)
        let normal_url = "https://upos-sz-mirrorcos.bilivideo.com/upgcxcode/video.m4s";
        let optimized = optimizer.optimize_url(normal_url);

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
