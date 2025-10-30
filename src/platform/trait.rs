use crate::error::Result;
use crate::types::{Auth, Stream, Subtitle, VideoInfo};
use async_trait::async_trait;

/// Platform trait defines the interface that all video platform implementations must follow.
/// This allows for a modular, extensible architecture where new platforms can be added
/// without modifying the core download logic.
#[async_trait]
pub trait Platform: Send + Sync {
    /// Get a reference to self as Any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Check if this platform can handle the given URL.
    ///
    /// # Arguments
    /// * `url` - The video URL to check
    ///
    /// # Returns
    /// `true` if this platform can handle the URL, `false` otherwise
    fn can_handle(&self, url: &str) -> bool;

    /// Parse video information from the given URL.
    ///
    /// # Arguments
    /// * `url` - The video URL to parse
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    /// A `VideoInfo` struct containing video metadata and page information
    async fn parse_video(&self, url: &str, auth: Option<&Auth>) -> Result<VideoInfo>;

    /// Get available video and audio streams for a specific video.
    ///
    /// # Arguments
    /// * `video_id` - The video identifier
    /// * `cid` - The content identifier (for platforms that use it)
    /// * `auth` - Optional authentication information
    ///
    /// # Returns
    /// A vector of available streams (both video and audio)
    async fn get_streams(
        &self,
        video_id: &str,
        cid: &str,
        auth: Option<&Auth>,
    ) -> Result<Vec<Stream>>;

    /// Get available subtitles for a video.
    ///
    /// # Arguments
    /// * `video_id` - The video identifier
    /// * `cid` - The content identifier
    ///
    /// # Returns
    /// A vector of available subtitles
    async fn get_subtitles(&self, video_id: &str, cid: &str) -> Result<Vec<Subtitle>>;

    /// Get the cover image URL for a video.
    ///
    /// # Arguments
    /// * `video_info` - The video information
    ///
    /// # Returns
    /// The URL of the cover image
    fn get_cover(&self, video_info: &VideoInfo) -> String;

    /// Get the name of this platform.
    ///
    /// # Returns
    /// A string identifying the platform (e.g., "bilibili", "youtube")
    fn name(&self) -> &str;
}
