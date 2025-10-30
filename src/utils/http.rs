use crate::error::{DownloaderError, Result};
use crate::types::Auth;
use reqwest::{header::HeaderMap, Client, Response};
use std::path::Path;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub struct HttpClient {
    pub client: Client,
    retry_count: usize,
    #[allow(dead_code)]
    timeout: Duration,
}

impl HttpClient {
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .build()?;

        Ok(Self {
            client,
            retry_count: 3,
            timeout: Duration::from_secs(60),
        })
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
            request = request.header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36");
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
