//! Testing utilities for platform implementations
//!
//! This module provides mock platforms and testing helpers for unit tests.

#[cfg(test)]
pub mod mock {
    use crate::error::Result;
    use crate::platform::{Platform, PlatformCapabilities, PlatformMetadata};
    use crate::types::{Auth, BatchResult, Stream, StreamContext, StreamType, Subtitle, VideoInfo};
    use async_trait::async_trait;

    /// Mock platform for testing
    ///
    /// A configurable mock platform that can be used in unit tests.
    ///
    /// # Example
    ///
    /// ```
    /// use rvd::platform::testing::mock::MockPlatform;
    ///
    /// let mut mock = MockPlatform::new("test");
    /// mock.add_url_pattern("test.com");
    /// mock.set_mock_video(/* ... */);
    /// ```
    pub struct MockPlatform {
        #[allow(dead_code)]
        name: String,
        url_patterns: Vec<String>,
        mock_video: Option<VideoInfo>,
        mock_streams: Vec<Stream>,
        mock_subtitles: Vec<Subtitle>,
        mock_batch: Option<BatchResult>,
        should_fail: bool,
        metadata: PlatformMetadata,
    }

    impl MockPlatform {
        /// Create a new mock platform
        ///
        /// # Arguments
        ///
        /// * `name` - The platform name
        pub fn new(name: &str) -> Self {
            let metadata = PlatformMetadata {
                name: Box::leak(name.to_string().into_boxed_str()),
                display_name: Box::leak(format!("Mock {}", name).into_boxed_str()),
                version: "1.0.0",
                capabilities: PlatformCapabilities::default(),
                url_patterns: Vec::new(),
            };

            Self {
                name: name.to_string(),
                url_patterns: vec![format!("{}.com", name)],
                mock_video: None,
                mock_streams: Vec::new(),
                mock_subtitles: Vec::new(),
                mock_batch: None,
                should_fail: false,
                metadata,
            }
        }

        /// Add a URL pattern
        pub fn add_url_pattern(&mut self, pattern: &str) {
            self.url_patterns.push(pattern.to_string());
            // Also update metadata for registry indexing
            let leaked_pattern: &'static str = Box::leak(pattern.to_string().into_boxed_str());
            let mut patterns = self.metadata.url_patterns.clone();
            patterns.push(leaked_pattern);
            self.metadata.url_patterns = patterns;
        }

        /// Set the mock video to return
        pub fn set_mock_video(&mut self, video: VideoInfo) {
            self.mock_video = Some(video);
        }

        /// Set the mock streams to return
        pub fn set_mock_streams(&mut self, streams: Vec<Stream>) {
            self.mock_streams = streams;
        }

        /// Set the mock subtitles to return
        pub fn set_mock_subtitles(&mut self, subtitles: Vec<Subtitle>) {
            self.mock_subtitles = subtitles;
        }

        /// Set the mock batch result to return
        pub fn set_mock_batch(&mut self, batch: BatchResult) {
            self.mock_batch = Some(batch);
        }

        /// Make the platform fail on all operations
        pub fn set_should_fail(&mut self, should_fail: bool) {
            self.should_fail = should_fail;
        }

        /// Update platform capabilities
        pub fn set_capabilities(&mut self, capabilities: PlatformCapabilities) {
            self.metadata.capabilities = capabilities;
        }
    }

    #[async_trait]
    impl Platform for MockPlatform {
        fn metadata(&self) -> &PlatformMetadata {
            &self.metadata
        }

        fn can_handle(&self, url: &str) -> bool {
            self.url_patterns.iter().any(|pattern| {
                if let Some(stripped) = pattern.strip_prefix('^') {
                    url.starts_with(stripped)
                } else {
                    url.contains(pattern)
                }
            })
        }

        async fn parse_video(&self, _url: &str, _auth: Option<&Auth>) -> Result<VideoInfo> {
            if self.should_fail {
                return Err(crate::error::DownloaderError::Parse(
                    "Mock failure".to_string(),
                ));
            }

            self.mock_video.clone().ok_or_else(|| {
                crate::error::DownloaderError::Parse("No mock video configured".to_string())
            })
        }

        async fn parse_batch(&self, _url: &str, _auth: Option<&Auth>) -> Result<BatchResult> {
            if self.should_fail {
                return Err(crate::error::DownloaderError::Parse(
                    "Mock failure".to_string(),
                ));
            }

            self.mock_batch.clone().ok_or_else(|| {
                crate::error::DownloaderError::Parse("No mock batch configured".to_string())
            })
        }

        async fn get_streams(
            &self,
            _context: &StreamContext,
            _auth: Option<&Auth>,
        ) -> Result<Vec<Stream>> {
            if self.should_fail {
                return Err(crate::error::DownloaderError::Parse(
                    "Mock failure".to_string(),
                ));
            }

            Ok(self.mock_streams.clone())
        }

        async fn get_subtitles(&self, _context: &StreamContext) -> Result<Vec<Subtitle>> {
            if self.should_fail {
                return Err(crate::error::DownloaderError::Parse(
                    "Mock failure".to_string(),
                ));
            }

            Ok(self.mock_subtitles.clone())
        }

        fn get_cover(&self, video_info: &VideoInfo) -> String {
            video_info.cover_url.clone()
        }

        fn as_any(&self) -> &dyn std::any::Any {
            self
        }
    }

    /// Create a test video info
    pub fn create_test_video(id: &str, title: &str) -> VideoInfo {
        use crate::types::Page;

        VideoInfo {
            id: id.to_string(),
            aid: 12345,
            title: title.to_string(),
            description: "Test description".to_string(),
            duration: 600,
            uploader: "Test Uploader".to_string(),
            uploader_mid: "123456".to_string(),
            upload_date: "2024-01-01".to_string(),
            cover_url: "https://example.com/cover.jpg".to_string(),
            pages: vec![Page {
                number: 1,
                title: "P1".to_string(),
                cid: "67890".to_string(),
                duration: 600,
                ep_id: None,
            }],
            is_bangumi: false,
            ep_id: None,
            extra_data: None,
        }
    }

    /// Create a test stream
    pub fn create_test_stream(
        stream_type: StreamType,
        quality: &str,
        codec: &str,
        bandwidth: u64,
    ) -> Stream {
        Stream {
            stream_type,
            quality: quality.to_string(),
            quality_id: 0,
            codec: codec.to_string(),
            url: "https://example.com/stream".to_string(),
            size: 0,
            bandwidth,
            extra_data: None,
        }
    }

    /// Create a test subtitle
    pub fn create_test_subtitle(language: &str, language_code: &str) -> Subtitle {
        Subtitle {
            language: language.to_string(),
            language_code: language_code.to_string(),
            url: "https://example.com/subtitle.srt".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::mock::*;
    use crate::app::PlatformRegistry;
    use crate::platform::Platform;
    use std::sync::Arc;

    #[tokio::test]
    async fn test_mock_platform() {
        let mut mock = MockPlatform::new("test");
        mock.set_mock_video(create_test_video("test123", "Test Video"));

        assert!(mock.can_handle("https://test.com/video/123"));
        assert!(!mock.can_handle("https://other.com/video/123"));

        let video = mock
            .parse_video("https://test.com/video/123", None)
            .await
            .unwrap();
        assert_eq!(video.id, "test123");
        assert_eq!(video.title, "Test Video");
    }

    #[test]
    fn test_platform_registry_with_mock() {
        let mut registry = PlatformRegistry::new();

        let mock1 = Arc::new(MockPlatform::new("test1"));
        let mock2 = Arc::new(MockPlatform::new("test2"));

        registry.register(mock1);
        registry.register(mock2);

        assert_eq!(registry.count(), 2);

        let platform = registry
            .select_platform("https://test1.com/video/123")
            .unwrap();
        assert_eq!(platform.name(), "test1");

        let platform = registry
            .select_platform("https://test2.com/video/456")
            .unwrap();
        assert_eq!(platform.name(), "test2");
    }

    #[test]
    fn test_platform_registry_pattern_matching() {
        let mut registry = PlatformRegistry::new();

        let mut mock = MockPlatform::new("test");
        mock.add_url_pattern("^BV");
        registry.register(Arc::new(mock));

        let platform = registry.select_platform("BV1xx411c7mD").unwrap();
        assert_eq!(platform.name(), "test");
    }

    #[test]
    fn test_platform_registry_no_match() {
        let registry = PlatformRegistry::new();

        let result = registry.select_platform("https://unknown.com/video/123");
        assert!(result.is_err());
    }
}
