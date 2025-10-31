/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 16:30:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 16:30:00
 * @Description: QR code display module tests
 */

use rvd::auth::qrcode::QRCodeDisplay;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_qrcode_save_to_file() {
    let url = "https://www.bilibili.com/";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_qrcode.png");

    let result = QRCodeDisplay::save_to_file(url, &file_path);
    assert!(result.is_ok(), "Failed to save QR code: {:?}", result.err());

    // 验证文件存在
    assert!(file_path.exists(), "QR code file was not created");

    // 验证文件大小大于0
    let metadata = fs::metadata(&file_path).unwrap();
    assert!(metadata.len() > 0, "QR code file is empty");

    // 验证文件大小合理（应该大于1KB）
    assert!(
        metadata.len() > 1024,
        "QR code file is too small: {} bytes",
        metadata.len()
    );
}

#[test]
fn test_qrcode_save_with_long_url() {
    // 测试较长的URL
    let url = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key=1234567890abcdef";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_long_qrcode.png");

    let result = QRCodeDisplay::save_to_file(url, &file_path);
    assert!(result.is_ok(), "Failed to save QR code with long URL");

    assert!(file_path.exists());
}

#[test]
fn test_qrcode_invalid_url() {
    // 测试空字符串（应该仍然能生成二维码）
    let url = "";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_empty_qrcode.png");

    let result = QRCodeDisplay::save_to_file(url, &file_path);
    // 空字符串应该也能生成二维码
    assert!(result.is_ok());
}

#[test]
fn test_qrcode_display_in_terminal() {
    // 这个测试只验证函数不会panic
    // 实际的终端显示效果需要手动验证
    let url = "https://www.bilibili.com/";

    // 在测试环境中，终端显示可能会失败，但不应该panic
    let result = QRCodeDisplay::display_in_terminal(url);

    // 我们不强制要求成功，因为测试环境可能没有终端
    // 但至少应该返回一个Result
    match result {
        Ok(_) => {
            // 成功显示
        }
        Err(e) => {
            // 显示失败也是可以接受的
            println!("Terminal display failed (expected in test env): {}", e);
        }
    }
}

#[test]
fn test_qrcode_multiple_saves() {
    // 测试多次保存到同一个文件
    let url = "https://www.bilibili.com/";
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("test_multiple_qrcode.png");

    // 第一次保存
    let result1 = QRCodeDisplay::save_to_file(url, &file_path);
    assert!(result1.is_ok());
    let size1 = fs::metadata(&file_path).unwrap().len();

    // 第二次保存（覆盖）
    let result2 = QRCodeDisplay::save_to_file(url, &file_path);
    assert!(result2.is_ok());
    let size2 = fs::metadata(&file_path).unwrap().len();

    // 文件大小应该相同（因为内容相同）
    assert_eq!(size1, size2);
}

#[test]
fn test_qrcode_different_urls_different_sizes() {
    let dir = tempdir().unwrap();

    // 短URL
    let short_url = "https://b23.tv/";
    let short_path = dir.path().join("short_qrcode.png");
    QRCodeDisplay::save_to_file(short_url, &short_path).unwrap();
    let short_size = fs::metadata(&short_path).unwrap().len();

    // 长URL
    let long_url = "https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key=1234567890abcdef1234567890abcdef";
    let long_path = dir.path().join("long_qrcode.png");
    QRCodeDisplay::save_to_file(long_url, &long_path).unwrap();
    let long_size = fs::metadata(&long_path).unwrap().len();

    // 长URL的二维码应该更大（包含更多数据）
    assert!(
        long_size > short_size,
        "Long URL QR code should be larger than short URL QR code"
    );
}
