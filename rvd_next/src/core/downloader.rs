use crate::error::{DownloaderError, Result};
use crate::types::Auth;
use crate::utils::http::HttpClient;
use futures::StreamExt;
use indicatif::ProgressBar;
use std::path::Path;
use std::sync::Arc;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::process::Command;

/// Download method to use
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DownloadMethod {
    /// Use built-in HTTP downloader
    Builtin,
    /// Use aria2c external downloader
    Aria2c,
}

pub struct Downloader {
    client: Arc<HttpClient>,
    pub(crate) thread_count: usize,
    chunk_size: usize,
    pub(crate) method: DownloadMethod,
    pub(crate) aria2c_path: String,
    pub(crate) aria2c_args: Option<String>,
    auth: Option<Auth>,
}

impl Downloader {
    pub fn new(client: Arc<HttpClient>, thread_count: usize) -> Self {
        Self {
            client,
            thread_count,
            chunk_size: 10 * 1024 * 1024, // 10MB chunks
            method: DownloadMethod::Builtin,
            aria2c_path: "aria2c".to_string(),
            aria2c_args: None,
            auth: None,
        }
    }

    /// Set download method
    pub fn with_method(mut self, method: DownloadMethod) -> Self {
        self.method = method;
        self
    }

    /// Set aria2c binary path
    pub fn with_aria2c_path(mut self, path: String) -> Self {
        self.aria2c_path = path;
        self
    }

    /// Set custom aria2c arguments
    pub fn with_aria2c_args(mut self, args: String) -> Self {
        self.aria2c_args = Some(args);
        self
    }

    /// Set authentication info for aria2c
    pub fn with_auth(mut self, auth: Option<Auth>) -> Self {
        self.auth = auth;
        self
    }

    /// Check if aria2c is available
    pub async fn check_aria2c(&self) -> Result<bool> {
        match Command::new(&self.aria2c_path)
            .arg("--version")
            .output()
            .await
        {
            Ok(output) => Ok(output.status.success()),
            Err(_) => Ok(false),
        }
    }

    pub async fn download(
        &self,
        url: &str,
        output: &Path,
        progress: Option<Arc<ProgressBar>>,
    ) -> Result<()> {
        tracing::info!("Downloading: {} -> {:?}", url, output);

        // Create parent directory if it doesn't exist
        if let Some(parent) = output.parent() {
            tokio::fs::create_dir_all(parent).await?;
        }

        // Use aria2c if specified
        if self.method == DownloadMethod::Aria2c {
            return self.download_with_aria2c(url, output, progress).await;
        }

        // Try to get file size
        let file_size = match self.get_file_size(url).await {
            Ok(size) => size,
            Err(_) => {
                tracing::warn!("Could not get file size, downloading without progress");
                return self.download_simple(url, output).await;
            }
        };

        if let Some(ref pb) = progress {
            pb.set_length(file_size);
        }

        // Check if server supports range requests
        if self.supports_range(url).await && file_size > self.chunk_size as u64 {
            self.download_chunked(url, output, file_size, progress)
                .await
        } else {
            self.download_streaming(url, output, progress).await
        }
    }

    async fn download_simple(&self, url: &str, output: &Path) -> Result<()> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Add required headers for Bilibili video downloads
        if url.contains("bilivideo.com") {
            if let Ok(value) = "https://www.bilibili.com".parse() {
                headers.insert("Referer", value);
            }
            if let Ok(value) =
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".parse()
            {
                headers.insert("User-Agent", value);
            }
        }

        let response = self.client.get(url, Some(headers)).await?;
        let bytes = response.bytes().await?;
        tokio::fs::write(output, bytes).await?;
        Ok(())
    }

    async fn download_streaming(
        &self,
        url: &str,
        output: &Path,
        progress: Option<Arc<ProgressBar>>,
    ) -> Result<()> {
        let mut headers = reqwest::header::HeaderMap::new();

        // Add required headers for Bilibili video downloads
        if url.contains("bilivideo.com") {
            if let Ok(value) = "https://www.bilibili.com".parse() {
                headers.insert("Referer", value);
            }
            if let Ok(value) =
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36".parse()
            {
                headers.insert("User-Agent", value);
            }
        }

        let response = self.client.get(url, Some(headers)).await?;
        let mut file = File::create(output).await?;
        let mut stream = response.bytes_stream();
        let mut downloaded = 0u64;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(DownloaderError::Network)?;
            file.write_all(&chunk).await?;
            downloaded += chunk.len() as u64;

            if let Some(ref pb) = progress {
                pb.set_position(downloaded);
            }
        }

        file.flush().await?;
        Ok(())
    }

    async fn get_file_size(&self, url: &str) -> Result<u64> {
        let mut request = self.client.client.head(url);

        // Add required headers for Bilibili video downloads
        if url.contains("bilivideo.com") {
            request = request.header("Referer", "https://www.bilibili.com");
            request = request.header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            );
        }

        let response = request.send().await?;

        if let Some(content_length) = response.headers().get("content-length") {
            let size = content_length
                .to_str()
                .map_err(|_| DownloaderError::Parse("Invalid content-length".to_string()))?
                .parse::<u64>()
                .map_err(|_| DownloaderError::Parse("Invalid content-length".to_string()))?;
            Ok(size)
        } else {
            Err(DownloaderError::DownloadFailed(
                "No content-length header".to_string(),
            ))
        }
    }

    async fn supports_range(&self, url: &str) -> bool {
        let mut request = self.client.client.head(url);

        // Add required headers for Bilibili video downloads
        if url.contains("bilivideo.com") {
            request = request.header("Referer", "https://www.bilibili.com");
            request = request.header(
                "User-Agent",
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
            );
        }

        if let Ok(response) = request.send().await {
            if let Some(accept_ranges) = response.headers().get("accept-ranges") {
                return accept_ranges.to_str().unwrap_or("") == "bytes";
            }
        }
        false
    }

    async fn download_chunked(
        &self,
        url: &str,
        output: &Path,
        total_size: u64,
        progress: Option<Arc<ProgressBar>>,
    ) -> Result<()> {
        let chunk_count = ((total_size as f64) / (self.chunk_size as f64)).ceil() as usize;
        let mut tasks = Vec::new();

        // Create temp directory for chunks
        let temp_dir = output.parent().unwrap().join(".rvd_temp");
        tokio::fs::create_dir_all(&temp_dir).await?;

        let mut chunk_paths = Vec::new();

        for i in 0..chunk_count {
            let start = i * self.chunk_size;
            let end = std::cmp::min(start + self.chunk_size - 1, total_size as usize - 1);
            let chunk_path = temp_dir.join(format!("chunk_{}", i));
            let url = url.to_string();
            let client = self.client.clone();
            let progress = progress.clone();

            let task = tokio::spawn(async move {
                client
                    .download_file(&url, &chunk_path, Some((start as u64, end as u64)))
                    .await?;

                if let Some(ref pb) = progress {
                    pb.inc((end - start + 1) as u64);
                }

                Ok::<_, DownloaderError>(chunk_path)
            });

            tasks.push(task);

            // Limit concurrent downloads
            if tasks.len() >= self.thread_count {
                let completed = futures::future::join_all(tasks.drain(..)).await;
                for result in completed {
                    let path = result.map_err(|e| {
                        DownloaderError::DownloadFailed(format!("Task failed: {}", e))
                    })??;
                    chunk_paths.push(path);
                }
            }
        }

        // Wait for remaining tasks
        let completed = futures::future::join_all(tasks).await;
        for result in completed {
            let path = result
                .map_err(|e| DownloaderError::DownloadFailed(format!("Task failed: {}", e)))??;
            chunk_paths.push(path);
        }

        // Merge chunks
        self.merge_chunks(&chunk_paths, output).await?;

        // Cleanup
        tokio::fs::remove_dir_all(&temp_dir).await?;

        Ok(())
    }

    async fn merge_chunks(&self, chunks: &[std::path::PathBuf], output: &Path) -> Result<()> {
        let mut output_file = File::create(output).await?;

        for chunk in chunks {
            let mut chunk_file = File::open(chunk).await?;
            tokio::io::copy(&mut chunk_file, &mut output_file).await?;
        }

        output_file.flush().await?;
        Ok(())
    }

    /// Download file using aria2c
    async fn download_with_aria2c(
        &self,
        url: &str,
        output: &Path,
        progress: Option<Arc<ProgressBar>>,
    ) -> Result<()> {
        tracing::info!("Using aria2c for download");

        // Check if aria2c is available
        if !self.check_aria2c().await? {
            tracing::warn!("aria2c not found, falling back to built-in downloader");
            // Fall back to built-in streaming download
            return self.download_streaming(url, output, progress).await;
        }

        let output_dir = output
            .parent()
            .ok_or_else(|| DownloaderError::DownloadFailed("Invalid output path".to_string()))?;
        let output_filename = output
            .file_name()
            .ok_or_else(|| DownloaderError::DownloadFailed("Invalid output filename".to_string()))?
            .to_str()
            .ok_or_else(|| {
                DownloaderError::DownloadFailed("Invalid output filename".to_string())
            })?;

        // Build aria2c command
        let mut args = vec![
            // Basic options
            "--auto-file-renaming=false".to_string(),
            "--download-result=hide".to_string(),
            "--allow-overwrite=true".to_string(),
            "--console-log-level=warn".to_string(),
            // Connection options (matching BBDown defaults)
            "-x16".to_string(), // max connections per server
            "-s16".to_string(), // split into 16 parts
            "-j16".to_string(), // max concurrent downloads
            "-k5M".to_string(), // min split size 5MB
        ];

        // Add headers for Bilibili
        if url.contains("bilivideo.com") {
            // Only add Referer for non-TV/APP API URLs
            if !url.contains("platform=android_tv_yst") && !url.contains("platform=android") {
                args.push("--header=Referer: https://www.bilibili.com".to_string());
            }
            args.push("--header=User-Agent: Mozilla/5.0".to_string());

            // Add cookie if available
            if let Some(ref auth) = self.auth {
                if let Some(ref cookie) = auth.cookie {
                    args.push(format!("--header=Cookie: {}", cookie));
                }
            }
        }

        // Add custom args if provided
        if let Some(ref custom_args) = self.aria2c_args {
            for arg in custom_args.split_whitespace() {
                args.push(arg.to_string());
            }
        }

        // Add URL and output options
        args.push(url.to_string());
        args.push("-d".to_string());
        args.push(
            output_dir
                .to_str()
                .ok_or_else(|| {
                    DownloaderError::DownloadFailed("Invalid output directory".to_string())
                })?
                .to_string(),
        );
        args.push("-o".to_string());
        args.push(output_filename.to_string());

        tracing::debug!("aria2c command: {} {}", self.aria2c_path, args.join(" "));

        // Execute aria2c
        let output_result = Command::new(&self.aria2c_path)
            .args(&args)
            .output()
            .await
            .map_err(|e| {
                DownloaderError::DownloadFailed(format!("Failed to execute aria2c: {}", e))
            })?;

        if !output_result.status.success() {
            let stderr = String::from_utf8_lossy(&output_result.stderr);
            return Err(DownloaderError::DownloadFailed(format!(
                "aria2c failed: {}",
                stderr
            )));
        }

        // Update progress bar to complete if provided
        if let Some(ref pb) = progress {
            if let Ok(metadata) = tokio::fs::metadata(output).await {
                pb.set_length(metadata.len());
                pb.set_position(metadata.len());
            }
            pb.finish();
        }

        tracing::info!("aria2c download completed successfully");
        Ok(())
    }
}
