//! Platform trait definition
//!
//! This module defines the core `Platform` trait that all video platforms must implement.

use crate::error::Result;
use crate::platform::metadata::PlatformMetadata;
use crate::types::{Auth, BatchResult, Stream, StreamContext, Subtitle, VideoInfo};
use async_trait::async_trait;

/// Platform trait - the core abstraction for all video platforms
///
/// This trait defines the interface that all video platforms must implement.
/// It provides methods for:
/// - URL handling and validation
/// - Video information extraction
/// - Batch/playlist processing
/// - Stream retrieval
/// - Subtitle and cover retrieval
///
/// # Example
///
/// ```no_run
/// use async_trait::async_trait;
/// use rvd::platform::Platform;
/// use rvd::types::{Auth, VideoInfo, Stream, StreamContext, BatchResult};
/// use rvd::error::Result;
///
/// struct MyPlatform;
///
/// #[async_trait]
/// impl Platform for MyPlatform {
///     fn metadata(&self) -> &rvd::platform::PlatformMetadata {
///         // Return platform metadata
///         unimplemented!()
///     }
///     
///     fn can_handle(&self, url: &str) -> bool {
///         url.contains("myplatform.com")
///     }
///     
///     async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo> {
///         // Parse video information
///         unimplemented!()
///     }
///     
///     async fn get_streams(
///         &self,
///         context: &StreamContext,
///         auth: Option<&Auth>,
///     ) -> Result<Vec<Stream>> {
///         // Get available streams
///         unimplemented!()
///     }
///     
///     fn get_cover(&self, video_info: &VideoInfo) -> String {
///         video_info.cover_url.clone()
///     }
///     
///     fn as_any(&self) -> &dyn std::any::Any {
///         self
///     }
/// }
/// ```
#[async_trait]
pub trait Platform: Send + Sync {
    /// Get platform metadata
    ///
    /// Returns metadata describing the platform's capabilities, supported features,
    /// and URL patterns.
    fn metadata(&self) -> &PlatformMetadata;

    /// Check if this platform can handle the given URL
    ///
    /// This method should quickly determine if a URL belongs to this platform.
    /// It's used by the platform registry to select the appropriate platform.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to check
    ///
    /// # Returns
    ///
    /// `true` if this platform can handle the URL, `false` otherwise
    fn can_handle(&self, url: &str) -> bool;

    /// Check if the URL is a batch type (playlist, favorites, etc.)
    ///
    /// This method determines whether a URL points to a single video or a collection
    /// of videos (playlist, favorites, user videos, etc.).
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to check
    ///
    /// # Returns
    ///
    /// `true` if the URL is a batch type, `false` if it's a single video
    ///
    /// # Default Implementation
    ///
    /// Returns `false` by default. Platforms that support batch downloads should override this.
    fn is_batch_url(&self, url: &str) -> bool {
        let _ = url;
        false
    }

    /// Parse a single video's information
    ///
    /// Extracts metadata about a video including title, description, uploader, pages, etc.
    /// If the URL is a batch type, this should return an error or only the first video.
    ///
    /// # Arguments
    ///
    /// * `url` - The video URL
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    ///
    /// A `VideoInfo` struct containing the video's metadata
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The URL is invalid
    /// - The video doesn't exist
    /// - Authentication is required but not provided
    /// - Network errors occur
    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo>;

    /// Parse a batch of videos (playlist, favorites, etc.)
    ///
    /// Extracts information about multiple videos from a batch URL.
    /// This method should return the first page of results if pagination is supported.
    ///
    /// # Arguments
    ///
    /// * `url` - The batch URL
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    ///
    /// A `BatchResult` containing the videos and pagination information
    ///
    /// # Default Implementation
    ///
    /// Calls `parse_video` and wraps the result in a `BatchResult::single`.
    /// Platforms that support batch downloads should override this.
    async fn parse_batch(&self, url: &str, auth: Option<&Auth>) -> Result<BatchResult> {
        let video = self.parse_video(url, auth).await?;
        Ok(BatchResult::single(video))
    }

    /// Get the next page of a batch download
    ///
    /// Retrieves additional pages of videos for paginated batch downloads.
    ///
    /// # Arguments
    ///
    /// * `url` - The original batch URL
    /// * `continuation` - Continuation token from the previous page
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    ///
    /// A `BatchResult` containing the next page of videos
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The platform doesn't support pagination
    /// - The continuation token is invalid
    /// - Network errors occur
    ///
    /// # Default Implementation
    ///
    /// Returns an error if a continuation token is provided, otherwise calls `parse_batch`.
    async fn parse_batch_page(
        &self,
        url: &str,
        continuation: Option<&str>,
        auth: Option<&Auth>,
    ) -> Result<BatchResult> {
        if continuation.is_some() {
            return Err(crate::error::DownloaderError::Parse(format!(
                "Platform {} does not support pagination",
                self.name()
            )));
        }
        self.parse_batch(url, auth).await
    }

    /// Get available streams for a video
    ///
    /// Retrieves the list of available video and audio streams with their quality,
    /// codec, and URL information.
    ///
    /// # Arguments
    ///
    /// * `context` - Stream context containing video ID and platform-specific parameters
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    ///
    /// A vector of `Stream` objects representing available streams
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The video doesn't exist
    /// - Authentication is required but not provided
    /// - Network errors occur
    async fn get_streams(
        &self,
        context: &StreamContext,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>>;

    /// Get available subtitles for a video
    ///
    /// Retrieves the list of available subtitles with their language and URL information.
    ///
    /// # Arguments
    ///
    /// * `context` - Stream context containing video ID and platform-specific parameters
    ///
    /// # Returns
    ///
    /// A vector of `Subtitle` objects representing available subtitles
    ///
    /// # Default Implementation
    ///
    /// Returns an empty vector. Platforms that support subtitles should override this.
    async fn get_subtitles(&self, context: &StreamContext) -> Result<Vec<Subtitle>> {
        let _ = context;
        Ok(Vec::new())
    }

    /// Get the cover image URL for a video
    ///
    /// Returns the URL of the video's cover/thumbnail image.
    ///
    /// # Arguments
    ///
    /// * `video_info` - The video information
    ///
    /// # Returns
    ///
    /// The cover image URL as a string
    fn get_cover(&self, video_info: &VideoInfo) -> String;

    /// Get the platform name
    ///
    /// Returns the platform's name (e.g., "bilibili", "youtube").
    ///
    /// # Default Implementation
    ///
    /// Returns `self.metadata().name`.
    fn name(&self) -> &str {
        self.metadata().name
    }

    /// Check if the platform supports a specific feature
    ///
    /// Checks whether the platform supports a given feature like subtitles,
    /// danmaku, batch downloads, etc.
    ///
    /// # Arguments
    ///
    /// * `feature` - The feature to check
    ///
    /// # Returns
    ///
    /// `true` if the feature is supported, `false` otherwise
    fn supports_feature(&self, feature: PlatformFeature) -> bool {
        let caps = &self.metadata().capabilities;
        match feature {
            PlatformFeature::Subtitles => caps.subtitles,
            PlatformFeature::Danmaku => caps.danmaku,
            PlatformFeature::BatchDownload => caps.batch_download,
            PlatformFeature::Chapters => caps.chapters,
            PlatformFeature::LiveStream => caps.live_stream,
            PlatformFeature::Authentication => caps.requires_auth,
        }
    }

    /// Extract playlist entries (for nested playlists)
    ///
    /// Some platforms (like YouTube) support nested playlists or channels.
    /// This method extracts the URLs of individual playlists from a channel or collection.
    ///
    /// # Arguments
    ///
    /// * `url` - The channel or collection URL
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    ///
    /// A vector of playlist URLs
    ///
    /// # Default Implementation
    ///
    /// Returns an empty vector. Platforms that support nested playlists should override this.
    async fn extract_playlist_entries(
        &self,
        _url: &str,
        _auth: Option<&Auth>,
    ) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    /// Check if the content is a live stream
    ///
    /// Determines whether the URL points to a live stream or a regular video.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL to check
    ///
    /// # Returns
    ///
    /// `true` if the content is a live stream, `false` otherwise
    ///
    /// # Default Implementation
    ///
    /// Returns `false`. Platforms that support live streams should override this.
    async fn is_live(&self, _url: &str) -> Result<bool> {
        Ok(false)
    }

    /// Runtime type conversion support
    ///
    /// Allows downcasting to the concrete platform type for accessing
    /// platform-specific functionality.
    ///
    /// # Returns
    ///
    /// A reference to `Any` for type conversion
    fn as_any(&self) -> &dyn std::any::Any;
}

/// Platform feature enumeration
///
/// Represents the various features that a platform may support.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PlatformFeature {
    /// Subtitle download support
    Subtitles,
    /// Danmaku (bullet comments) support
    Danmaku,
    /// Batch download support (playlists, favorites, etc.)
    BatchDownload,
    /// Chapter information support
    Chapters,
    /// Live stream support
    LiveStream,
    /// Authentication requirement
    Authentication,
}
