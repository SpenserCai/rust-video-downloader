use crate::error::{DownloaderError, Result};
use crate::types::Auth;
use rand::Rng;
use reqwest::{header::HeaderMap, Client, Response};
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

/// User-Agent generator for randomizing browser fingerprints
///
/// Generates realistic User-Agent strings to avoid detection as a bot.
/// Supports multiple browsers (Chrome, Firefox, Safari, Edge) and platforms
/// (Windows, macOS, Linux).
pub struct UserAgentGenerator {
    rng: rand::rngs::ThreadRng,
}

impl UserAgentGenerator {
    /// Create a new User-Agent generator
    pub fn new() -> Self {
        Self {
            rng: rand::thread_rng(),
        }
    }

    /// Generate a random User-Agent string
    ///
    /// Returns a realistic User-Agent string combining a random platform
    /// and browser.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let mut generator = UserAgentGenerator::new();
    /// let ua = generator.generate();
    /// // Returns something like:
    /// // "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.5790.110 Safari/537.36"
    /// ```
    pub fn generate(&mut self) -> String {
        let platform = self.random_platform();
        let browser = self.random_browser();
        format!("Mozilla/5.0 ({}) {}", platform, browser)
    }

    /// Get a random platform string
    fn random_platform(&mut self) -> &'static str {
        let platforms = [
            "Windows NT 10.0; Win64; x64",
            "Windows NT 11.0; Win64; x64",
            "Macintosh; Intel Mac OS X 10_15_7",
            "Macintosh; Intel Mac OS X 11_6_0",
            "X11; Linux x86_64",
            "X11; Ubuntu; Linux x86_64",
        ];
        platforms[self.rng.gen_range(0..platforms.len())]
    }

    /// Get a random browser string
    fn random_browser(&mut self) -> String {
        let browser_type = self.rng.gen_range(0..4);

        match browser_type {
            0 => self.chrome_ua(),
            1 => self.firefox_ua(),
            2 => self.safari_ua(),
            _ => self.edge_ua(),
        }
    }

    /// Generate Chrome User-Agent
    fn chrome_ua(&mut self) -> String {
        let version = self.rng.gen_range(100..120);
        let build = self.rng.gen_range(5000..6000);
        format!(
            "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.{}.0 Safari/537.36",
            version, build
        )
    }

    /// Generate Firefox User-Agent
    fn firefox_ua(&mut self) -> String {
        let version = self.rng.gen_range(100..120);
        format!("Gecko/20100101 Firefox/{}.0", version)
    }

    /// Generate Safari User-Agent
    fn safari_ua(&mut self) -> String {
        let version = self.rng.gen_range(15..17);
        format!(
            "AppleWebKit/605.1.15 (KHTML, like Gecko) Version/{}.0 Safari/605.1.15",
            version
        )
    }

    /// Generate Edge User-Agent
    fn edge_ua(&mut self) -> String {
        let version = self.rng.gen_range(100..120);
        let build = self.rng.gen_range(5000..6000);
        format!(
            "AppleWebKit/537.36 (KHTML, like Gecko) Chrome/{}.0.{}.0 Safari/537.36 Edg/{}.0.0.0",
            version, build, version
        )
    }
}

impl Default for UserAgentGenerator {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HttpClient {
    pub client: Client,
    retry_count: usize,
    #[allow(dead_code)]
    timeout: Duration,
    user_agent: String,
    custom_user_agent: Option<String>,
}

impl HttpClient {
    /// Create a new HTTP client with a random User-Agent
    pub fn new() -> Result<Self> {
        let mut ua_gen = UserAgentGenerator::new();
        let user_agent = ua_gen.generate();

        tracing::debug!("Generated User-Agent: {}", user_agent);

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent(&user_agent)
            .build()?;

        Ok(Self {
            client,
            retry_count: 3,
            timeout: Duration::from_secs(60),
            user_agent,
            custom_user_agent: None,
        })
    }

    /// Create a new HTTP client with a custom User-Agent
    ///
    /// This is useful for debugging or when you need a specific User-Agent.
    ///
    /// # Arguments
    ///
    /// * `user_agent` - The custom User-Agent string to use
    pub fn with_custom_user_agent(user_agent: String) -> Result<Self> {
        tracing::info!("Using custom User-Agent: {}", user_agent);

        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent(&user_agent)
            .build()?;

        Ok(Self {
            client,
            retry_count: 3,
            timeout: Duration::from_secs(60),
            user_agent: user_agent.clone(),
            custom_user_agent: Some(user_agent),
        })
    }

    /// Get the current User-Agent string
    pub fn user_agent(&self) -> &str {
        self.custom_user_agent
            .as_deref()
            .unwrap_or(&self.user_agent)
    }

    pub async fn get(&self, url: &str, headers: Option<HeaderMap>) -> Result<Response> {
        self.request_with_retry(url, headers, None).await
    }

    #[allow(dead_code)]
    pub async fn post(
        &self,
        url: &str,
        body: &str,
        headers: Option<HeaderMap>,
    ) -> Result<Response> {
        self.request_with_retry(url, headers, Some(body)).await
    }

    async fn request_with_retry(
        &self,
        url: &str,
        headers: Option<HeaderMap>,
        body: Option<&str>,
    ) -> Result<Response> {
        let mut last_error = None;

        for attempt in 0..self.retry_count {
            if attempt > 0 {
                let delay = Duration::from_secs(2u64.pow(attempt as u32));
                tokio::time::sleep(delay).await;
                tracing::debug!("Retrying request (attempt {})", attempt + 1);
            }

            let mut request = if body.is_some() {
                self.client.post(url)
            } else {
                self.client.get(url)
            };

            if let Some(ref h) = headers {
                request = request.headers(h.clone());
            }

            if let Some(b) = body {
                request = request.body(b.to_string());
            }

            match request.send().await {
                Ok(response) => {
                    if response.status().is_success() {
                        return Ok(response);
                    } else {
                        last_error = Some(DownloaderError::Network(
                            response.error_for_status().unwrap_err(),
                        ));
                    }
                }
                Err(e) => {
                    last_error = Some(DownloaderError::Network(e));
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| DownloaderError::DownloadFailed("Max retries exceeded".to_string())))
    }

    pub async fn download_file(
        &self,
        url: &str,
        output: &Path,
        range: Option<(u64, u64)>,
    ) -> Result<()> {
        let mut request = self.client.get(url);

        // Add required headers for Bilibili video downloads
        if url.contains("bilivideo.com") {
            request = request.header("Referer", "https://www.bilibili.com");
            request = request.header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            );
        }

        if let Some((start, end)) = range {
            request = request.header("Range", format!("bytes={}-{}", start, end));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            return Err(DownloaderError::DownloadFailed(format!(
                "HTTP {}",
                response.status()
            )));
        }

        let mut file = File::create(output).await?;
        let bytes = response.bytes().await?;
        file.write_all(&bytes).await?;
        file.flush().await?;

        Ok(())
    }

    pub fn add_auth(&self, headers: &mut HeaderMap, auth: &Auth) {
        if let Some(ref cookie) = auth.cookie {
            if let Ok(value) = cookie.parse() {
                headers.insert("Cookie", value);
            }
        }

        if let Some(ref token) = auth.access_token {
            if let Ok(value) = format!("identify_v1 {}", token).parse() {
                headers.insert("Authorization", value);
            }
        }
    }

    pub async fn get_with_auth(&self, url: &str, auth: Option<&Auth>) -> Result<Response> {
        let mut headers = HeaderMap::new();

        if let Some(auth) = auth {
            self.add_auth(&mut headers, auth);
        }

        // Add referer for bilibili API
        if url.contains("api.bilibili.com") {
            if let Ok(value) = "https://www.bilibili.com/".parse() {
                headers.insert("Referer", value);
            }
        }

        self.get(url, Some(headers)).await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_agent_generation() {
        let mut ua_gen = UserAgentGenerator::new();

        // Generate multiple UAs
        let ua1 = ua_gen.generate();
        let ua2 = ua_gen.generate();
        let ua3 = ua_gen.generate();

        // All should start with Mozilla/5.0
        assert!(ua1.starts_with("Mozilla/5.0"));
        assert!(ua2.starts_with("Mozilla/5.0"));
        assert!(ua3.starts_with("Mozilla/5.0"));

        // At least one should be different (very high probability)
        assert!(ua1 != ua2 || ua2 != ua3 || ua1 != ua3);
    }

    #[test]
    fn test_user_agent_format() {
        let mut ua_gen = UserAgentGenerator::new();
        let ua = ua_gen.generate();

        // Should contain platform
        assert!(
            ua.contains("Windows") || ua.contains("Macintosh") || ua.contains("Linux"),
            "UA should contain a platform: {}",
            ua
        );

        // Should contain browser
        assert!(
            ua.contains("Chrome") || ua.contains("Firefox") || ua.contains("Safari") || ua.contains("Edg"),
            "UA should contain a browser: {}",
            ua
        );
    }

    #[test]
    fn test_chrome_ua_format() {
        let mut ua_gen = UserAgentGenerator::new();
        let ua = ua_gen.chrome_ua();

        assert!(ua.contains("Chrome/"));
        assert!(ua.contains("Safari/537.36"));
        assert!(ua.contains("AppleWebKit/537.36"));
    }

    #[test]
    fn test_firefox_ua_format() {
        let mut ua_gen = UserAgentGenerator::new();
        let ua = ua_gen.firefox_ua();

        assert!(ua.contains("Firefox/"));
        assert!(ua.contains("Gecko/20100101"));
    }

    #[test]
    fn test_safari_ua_format() {
        let mut ua_gen = UserAgentGenerator::new();
        let ua = ua_gen.safari_ua();

        assert!(ua.contains("Safari/"));
        assert!(ua.contains("Version/"));
        assert!(ua.contains("AppleWebKit/605.1.15"));
    }

    #[test]
    fn test_edge_ua_format() {
        let mut ua_gen = UserAgentGenerator::new();
        let ua = ua_gen.edge_ua();

        assert!(ua.contains("Edg/"));
        assert!(ua.contains("Chrome/"));
        assert!(ua.contains("Safari/537.36"));
    }

    #[test]
    fn test_http_client_random_ua() {
        let client1 = HttpClient::new().unwrap();
        let client2 = HttpClient::new().unwrap();

        let ua1 = client1.user_agent();
        let ua2 = client2.user_agent();

        // Both should be valid UAs
        assert!(ua1.starts_with("Mozilla/5.0"));
        assert!(ua2.starts_with("Mozilla/5.0"));

        // They might be different (not guaranteed but very likely)
        tracing::info!("UA1: {}", ua1);
        tracing::info!("UA2: {}", ua2);
    }

    #[test]
    fn test_http_client_custom_ua() {
        let custom_ua = "CustomBot/1.0".to_string();
        let client = HttpClient::with_custom_user_agent(custom_ua.clone()).unwrap();

        assert_eq!(client.user_agent(), custom_ua);
    }
}
