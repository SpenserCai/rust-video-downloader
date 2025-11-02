use rvd::core::downloader::{DownloadMethod, Downloader};
use rvd::utils::http::HttpClient;
use std::sync::Arc;

#[tokio::test]
async fn test_aria2c_availability_check() {
    let client = Arc::new(HttpClient::new().unwrap());
    let downloader = Downloader::new(client, 4)
        .with_method(DownloadMethod::Aria2c);

    // Just check if aria2c is available (won't fail if not installed)
    let is_available = downloader.check_aria2c().await.unwrap();
    
    if is_available {
        println!("✓ aria2c is available");
    } else {
        println!("ℹ aria2c is not installed (this is OK for testing)");
    }
}

#[tokio::test]
async fn test_downloader_method_configuration() {
    let client = Arc::new(HttpClient::new().unwrap());
    
    // Test that downloader can be configured with different methods
    // We can't directly test private fields, but we can test that the builder pattern works
    let _downloader_builtin = Downloader::new(client.clone(), 4);
    
    let _downloader_aria2c = Downloader::new(client.clone(), 4)
        .with_method(DownloadMethod::Aria2c);
    
    let _downloader_custom = Downloader::new(client.clone(), 4)
        .with_method(DownloadMethod::Aria2c)
        .with_aria2c_path("/custom/path/aria2c".to_string());
    
    let _downloader_args = Downloader::new(client, 4)
        .with_method(DownloadMethod::Aria2c)
        .with_aria2c_args("-x8 -s8".to_string());
    
    // If we got here without panicking, the builder pattern works correctly
    assert!(true);
}

#[test]
fn test_download_method_enum() {
    // Test enum equality
    assert_eq!(DownloadMethod::Builtin, DownloadMethod::Builtin);
    assert_eq!(DownloadMethod::Aria2c, DownloadMethod::Aria2c);
    assert_ne!(DownloadMethod::Builtin, DownloadMethod::Aria2c);
}
