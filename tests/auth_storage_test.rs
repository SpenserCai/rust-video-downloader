/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 16:45:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 16:45:00
 * @Description: Credential storage module tests
 */

use rvd::auth::storage::CredentialStorage;
use rvd::auth::types::Credentials;
use std::fs;
use tempfile::tempdir;

#[test]
fn test_save_and_load_credentials() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 创建测试凭证
    let credentials = Credentials {
        cookie: Some("SESSDATA=test123; bili_jct=test456".to_string()),
        access_token: Some("test_access_token".to_string()),
        refresh_token: Some("test_refresh_token".to_string()),
        expires_at: Some(1234567890),
        mid: Some(123456),
    };

    // 保存凭证
    let result = CredentialStorage::save_to_config(&credentials, &config_path);
    assert!(result.is_ok(), "Failed to save credentials: {:?}", result.err());

    // 验证auth.toml文件存在
    let auth_path = dir.path().join("auth.toml");
    assert!(auth_path.exists(), "auth.toml file was not created");

    // 验证文件内容
    let content = fs::read_to_string(&auth_path).unwrap();
    assert!(content.contains("认证凭证文件"), "Missing warning comment");
    assert!(content.contains("cookie"), "Missing cookie field");
    assert!(content.contains("access_token"), "Missing access_token field");

    // 加载凭证
    let loaded = CredentialStorage::load_from_config(&config_path).unwrap();
    assert!(loaded.is_some(), "Failed to load credentials");

    let loaded_creds = loaded.unwrap();
    assert_eq!(loaded_creds.cookie, credentials.cookie);
    assert_eq!(loaded_creds.access_token, credentials.access_token);
    assert_eq!(loaded_creds.refresh_token, credentials.refresh_token);
    assert_eq!(loaded_creds.expires_at, credentials.expires_at);
    assert_eq!(loaded_creds.mid, credentials.mid);
}

#[test]
fn test_save_credentials_with_optional_fields() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 创建只有部分字段的凭证
    let credentials = Credentials {
        cookie: Some("test_cookie".to_string()),
        access_token: None,
        refresh_token: None,
        expires_at: None,
        mid: None,
    };

    let result = CredentialStorage::save_to_config(&credentials, &config_path);
    assert!(result.is_ok());

    // 加载并验证
    let loaded = CredentialStorage::load_from_config(&config_path).unwrap();
    assert!(loaded.is_some());

    let loaded_creds = loaded.unwrap();
    assert_eq!(loaded_creds.cookie, credentials.cookie);
    assert_eq!(loaded_creds.access_token, None);
    assert_eq!(loaded_creds.refresh_token, None);
}

#[test]
fn test_to_auth() {
    let credentials = Credentials {
        cookie: Some("test_cookie".to_string()),
        access_token: Some("test_token".to_string()),
        refresh_token: Some("test_refresh".to_string()),
        expires_at: Some(1234567890),
        mid: Some(123456),
    };

    let auth = CredentialStorage::to_auth(&credentials);
    assert_eq!(auth.cookie, credentials.cookie);
    assert_eq!(auth.access_token, credentials.access_token);
}

#[test]
fn test_load_nonexistent_config() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("nonexistent.toml");

    let result = CredentialStorage::load_from_config(&config_path);
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_save_credentials_creates_directory() {
    let dir = tempdir().unwrap();
    let subdir = dir.path().join("subdir");
    let config_path = subdir.join("config.toml");

    // 创建子目录
    fs::create_dir_all(&subdir).unwrap();

    let credentials = Credentials {
        cookie: Some("test_cookie".to_string()),
        access_token: Some("test_token".to_string()),
        refresh_token: None,
        expires_at: None,
        mid: None,
    };

    let result = CredentialStorage::save_to_config(&credentials, &config_path);
    assert!(result.is_ok());

    let auth_path = subdir.join("auth.toml");
    assert!(auth_path.exists());
}

#[test]
fn test_credentials_with_all_fields() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let credentials = Credentials {
        cookie: Some("SESSDATA=abc; bili_jct=def; DedeUserID=123".to_string()),
        access_token: Some("access_token_value".to_string()),
        refresh_token: Some("refresh_token_value".to_string()),
        expires_at: Some(9999999999),
        mid: Some(987654321),
    };

    CredentialStorage::save_to_config(&credentials, &config_path).unwrap();

    let loaded = CredentialStorage::load_from_config(&config_path)
        .unwrap()
        .unwrap();

    assert_eq!(loaded.cookie, credentials.cookie);
    assert_eq!(loaded.access_token, credentials.access_token);
    assert_eq!(loaded.refresh_token, credentials.refresh_token);
    assert_eq!(loaded.expires_at, credentials.expires_at);
    assert_eq!(loaded.mid, credentials.mid);
}

#[test]
fn test_overwrite_existing_auth_file() {
    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    // 第一次保存
    let credentials1 = Credentials {
        cookie: Some("cookie1".to_string()),
        access_token: Some("token1".to_string()),
        refresh_token: None,
        expires_at: None,
        mid: None,
    };
    CredentialStorage::save_to_config(&credentials1, &config_path).unwrap();

    // 第二次保存（覆盖）
    let credentials2 = Credentials {
        cookie: Some("cookie2".to_string()),
        access_token: Some("token2".to_string()),
        refresh_token: Some("refresh2".to_string()),
        expires_at: Some(1111111111),
        mid: Some(999),
    };
    CredentialStorage::save_to_config(&credentials2, &config_path).unwrap();

    // 验证加载的是第二次保存的内容
    let loaded = CredentialStorage::load_from_config(&config_path)
        .unwrap()
        .unwrap();

    assert_eq!(loaded.cookie, credentials2.cookie);
    assert_eq!(loaded.access_token, credentials2.access_token);
    assert_eq!(loaded.refresh_token, credentials2.refresh_token);
}

#[cfg(unix)]
#[test]
fn test_file_permissions_unix() {
    use std::os::unix::fs::PermissionsExt;

    let dir = tempdir().unwrap();
    let config_path = dir.path().join("config.toml");

    let credentials = Credentials {
        cookie: Some("test_cookie".to_string()),
        access_token: None,
        refresh_token: None,
        expires_at: None,
        mid: None,
    };

    CredentialStorage::save_to_config(&credentials, &config_path).unwrap();

    let auth_path = dir.path().join("auth.toml");
    let metadata = fs::metadata(&auth_path).unwrap();
    let permissions = metadata.permissions();

    // 验证权限为0600（仅所有者可读写）
    assert_eq!(permissions.mode() & 0o777, 0o600);
}
