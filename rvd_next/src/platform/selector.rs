//! Stream selector - selects the best streams based on preferences
//!
//! This module provides a generic stream selector that can be used by all platforms.

use crate::error::Result;
use crate::types::{Stream, StreamPreferences, StreamType};

/// Stream selector
///
/// Selects the best video and audio streams based on user preferences.
/// Supports quality and codec priority matching with automatic fallback.
///
/// # Example
///
/// ```no_run
/// use rvd::platform::StreamSelector;
/// use rvd::types::{Stream, StreamPreferences};
///
/// let preferences = StreamPreferences {
///     quality_priority: vec!["1080P".to_string(), "720P".to_string()],
///     codec_priority: vec!["hevc".to_string(), "avc".to_string()],
/// };
///
/// let selector = StreamSelector::new(preferences);
/// // let streams = vec![/* ... */];
/// // let (video, audio) = selector.select_best(&streams)?;
/// ```
pub struct StreamSelector {
    preferences: StreamPreferences,
}

impl StreamSelector {
    /// Create a new stream selector with the given preferences
    ///
    /// # Arguments
    ///
    /// * `preferences` - Stream selection preferences
    pub fn new(preferences: StreamPreferences) -> Self {
        Self { preferences }
    }

    /// Select the best video and audio streams
    ///
    /// Selects streams based on quality and codec preferences, with automatic
    /// fallback to lower quality if the preferred quality is not available.
    ///
    /// # Arguments
    ///
    /// * `streams` - Available streams
    ///
    /// # Returns
    ///
    /// A tuple of (video_stream, audio_stream)
    ///
    /// # Errors
    ///
    /// Returns an error if no suitable streams are found.
    pub fn select_best(&self, streams: &[Stream]) -> Result<(Stream, Stream)> {
        let video = self.select_video_stream(streams)?;
        let audio = self.select_audio_stream(streams)?;
        Ok((video, audio))
    }

    /// Select the best video stream
    ///
    /// Tries to match quality and codec preferences in order, falling back
    /// to the highest quality available if no match is found.
    ///
    /// # Arguments
    ///
    /// * `streams` - Available streams
    ///
    /// # Returns
    ///
    /// The selected video stream
    ///
    /// # Errors
    ///
    /// Returns an error if no video streams are available.
    pub fn select_video_stream(&self, streams: &[Stream]) -> Result<Stream> {
        let video_streams: Vec<&Stream> = streams
            .iter()
            .filter(|s| s.stream_type == StreamType::Video)
            .collect();

        if video_streams.is_empty() {
            return Err(crate::error::DownloaderError::Parse(
                "No video streams available".to_string(),
            ));
        }

        // Try to match quality preferences
        for quality in &self.preferences.quality_priority {
            // Try to match codec preferences for this quality
            for codec in &self.preferences.codec_priority {
                if let Some(stream) = video_streams.iter().find(|s| {
                    s.quality.to_lowercase().contains(&quality.to_lowercase())
                        && s.codec.to_lowercase().contains(&codec.to_lowercase())
                }) {
                    tracing::debug!(
                        "Selected video stream: quality={}, codec={}, bandwidth={}",
                        stream.quality,
                        stream.codec,
                        stream.bandwidth
                    );
                    return Ok((*stream).clone());
                }
            }

            // If no codec match, just match quality
            if let Some(stream) = video_streams
                .iter()
                .find(|s| s.quality.to_lowercase().contains(&quality.to_lowercase()))
            {
                tracing::debug!(
                    "Selected video stream (quality only): quality={}, codec={}, bandwidth={}",
                    stream.quality,
                    stream.codec,
                    stream.bandwidth
                );
                return Ok((*stream).clone());
            }
        }

        // Fallback: select highest quality
        let best = video_streams.iter().max_by_key(|s| s.bandwidth).unwrap();

        tracing::debug!(
            "Selected video stream (fallback to highest quality): quality={}, codec={}, bandwidth={}",
            best.quality, best.codec, best.bandwidth
        );

        Ok((*best).clone())
    }

    /// Select the best audio stream
    ///
    /// Selects the audio stream with the highest bandwidth.
    ///
    /// # Arguments
    ///
    /// * `streams` - Available streams
    ///
    /// # Returns
    ///
    /// The selected audio stream
    ///
    /// # Errors
    ///
    /// Returns an error if no audio streams are available.
    pub fn select_audio_stream(&self, streams: &[Stream]) -> Result<Stream> {
        let audio_streams: Vec<&Stream> = streams
            .iter()
            .filter(|s| s.stream_type == StreamType::Audio)
            .collect();

        if audio_streams.is_empty() {
            return Err(crate::error::DownloaderError::Parse(
                "No audio streams available".to_string(),
            ));
        }

        // Select highest quality audio
        let best = audio_streams.iter().max_by_key(|s| s.bandwidth).unwrap();

        tracing::debug!(
            "Selected audio stream: quality={}, codec={}, bandwidth={}",
            best.quality,
            best.codec,
            best.bandwidth
        );

        Ok((*best).clone())
    }

    /// Interactive stream selection
    ///
    /// Presents available streams to the user and lets them choose.
    ///
    /// # Arguments
    ///
    /// * `streams` - Available streams
    ///
    /// # Returns
    ///
    /// A tuple of (video_stream, audio_stream)
    ///
    /// # Errors
    ///
    /// Returns an error if the user cancels or if no streams are available.
    #[allow(dead_code)]
    pub fn select_interactive(&self, streams: &[Stream]) -> Result<(Stream, Stream)> {
        use dialoguer::Select;

        let video_streams: Vec<&Stream> = streams
            .iter()
            .filter(|s| s.stream_type == StreamType::Video)
            .collect();

        let audio_streams: Vec<&Stream> = streams
            .iter()
            .filter(|s| s.stream_type == StreamType::Audio)
            .collect();

        if video_streams.is_empty() || audio_streams.is_empty() {
            return Err(crate::error::DownloaderError::Parse(
                "No video or audio streams available".to_string(),
            ));
        }

        // Select video stream
        let video_items: Vec<String> = video_streams
            .iter()
            .map(|s| {
                format!(
                    "{} - {} - {} MB/s",
                    s.quality,
                    s.codec,
                    s.bandwidth / 1_000_000
                )
            })
            .collect();

        let video_selection = Select::new()
            .with_prompt("Select video stream")
            .items(&video_items)
            .default(0)
            .interact()
            .map_err(|e| {
                crate::error::DownloaderError::Parse(format!("Selection cancelled: {}", e))
            })?;

        // Select audio stream
        let audio_items: Vec<String> = audio_streams
            .iter()
            .map(|s| format!("{} - {} - {} KB/s", s.quality, s.codec, s.bandwidth / 1_000))
            .collect();

        let audio_selection = Select::new()
            .with_prompt("Select audio stream")
            .items(&audio_items)
            .default(0)
            .interact()
            .map_err(|e| {
                crate::error::DownloaderError::Parse(format!("Selection cancelled: {}", e))
            })?;

        Ok((
            video_streams[video_selection].clone(),
            audio_streams[audio_selection].clone(),
        ))
    }
}

impl Default for StreamSelector {
    fn default() -> Self {
        Self::new(StreamPreferences::default())
    }
}

/// Convenience function to select best streams
///
/// This is a convenience wrapper around `StreamSelector::select_best`.
///
/// # Arguments
///
/// * `streams` - Available streams
/// * `preferences` - Stream selection preferences
///
/// # Returns
///
/// A tuple of (video_stream, audio_stream)
///
/// # Errors
///
/// Returns an error if no suitable streams are found.
pub fn select_best_streams(
    streams: &[Stream],
    preferences: &StreamPreferences,
) -> Result<(Stream, Stream)> {
    let selector = StreamSelector::new(preferences.clone());
    selector.select_best(streams)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Stream, StreamType};

    fn create_test_stream(
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

    #[test]
    fn test_select_best_streams() {
        let streams = vec![
            create_test_stream(StreamType::Video, "1080P", "hevc", 5_000_000),
            create_test_stream(StreamType::Video, "720P", "avc", 3_000_000),
            create_test_stream(StreamType::Video, "480P", "avc", 1_000_000),
            create_test_stream(StreamType::Audio, "High", "aac", 320_000),
            create_test_stream(StreamType::Audio, "Medium", "aac", 128_000),
        ];

        let preferences = StreamPreferences {
            quality_priority: vec!["1080P".to_string(), "720P".to_string()],
            codec_priority: vec!["hevc".to_string(), "avc".to_string()],
        };

        let selector = StreamSelector::new(preferences);
        let (video, audio) = selector.select_best(&streams).unwrap();

        assert_eq!(video.quality, "1080P");
        assert_eq!(video.codec, "hevc");
        assert_eq!(audio.bandwidth, 320_000);
    }

    #[test]
    fn test_fallback_to_highest_quality() {
        let streams = vec![
            create_test_stream(StreamType::Video, "720P", "avc", 3_000_000),
            create_test_stream(StreamType::Video, "480P", "avc", 1_000_000),
            create_test_stream(StreamType::Audio, "High", "aac", 320_000),
        ];

        let preferences = StreamPreferences {
            quality_priority: vec!["4K".to_string(), "1080P".to_string()],
            codec_priority: vec!["av1".to_string()],
        };

        let selector = StreamSelector::new(preferences);
        let (video, audio) = selector.select_best(&streams).unwrap();

        // Should fallback to highest quality available
        assert_eq!(video.quality, "720P");
        assert_eq!(audio.bandwidth, 320_000);
    }
}
