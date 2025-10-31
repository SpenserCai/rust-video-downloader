use rvd::auth::providers::{AppSignManager, BilibiliAuthProvider};
use rvd::auth::AuthProvider;
use rvd::platform::bilibili::ApiMode;
use rvd::utils::http::HttpClient;
use std::collections::HashMap;
use std::sync::Arc;

#[test]
fn test_app_sign_manager_tv() {
    let manager = AppSignManager::new_tv();

    // 测试签名算法的正确性
    let mut params = HashMap::new();
    params.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params.insert("local_id".to_string(), "0".to_string());
    params.insert("ts".to_string(), "1234567890".to_string());

    let sign = manager.sign_params(&params);

    // 验证签名格式（应该是32位小写十六进制字符串）
    assert_eq!(sign.len(), 32);
    assert!(sign.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));

    // 验证签名的一致性（相同输入应该产生相同输出）
    let sign2 = manager.sign_params(&params);
    assert_eq!(sign, sign2);

    // 验证已知的签名结果
    // 根据签名算法：appkey=4409e2ce8ffd12b8&local_id=0&ts=1234567890 + appsec
    // 应该产生特定的MD5值
    let expected_string = "appkey=4409e2ce8ffd12b8&local_id=0&ts=123456789059b43e04ad6965f34319062b478f83dd";
    let expected_sign = format!("{:x}", md5::compute(expected_string.as_bytes()));
    assert_eq!(sign, expected_sign);
}

#[test]
fn test_app_sign_manager_timestamp() {
    let manager = AppSignManager::new_tv();
    let ts1 = manager.get_timestamp();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let ts2 = manager.get_timestamp();

    // 时间戳应该递增
    assert!(ts2 >= ts1);
    // 时间戳应该是合理的（大于2020年）
    assert!(ts1 > 1577836800); // 2020-01-01
}

#[test]
fn test_bilibili_auth_provider_creation() {
    let client = Arc::new(HttpClient::new().unwrap());

    // 测试Web模式
    let web_provider = BilibiliAuthProvider::new(client.clone(), ApiMode::Web);
    assert!(std::mem::size_of_val(&web_provider) > 0);

    // 测试TV模式
    let tv_provider = BilibiliAuthProvider::new(client.clone(), ApiMode::TV);
    assert!(std::mem::size_of_val(&tv_provider) > 0);
}

#[tokio::test]
async fn test_bilibili_auth_provider_unsupported_mode() {
    let client = Arc::new(HttpClient::new().unwrap());
    let provider = BilibiliAuthProvider::new(client, ApiMode::App);

    // App模式应该返回错误
    let result = provider.request_qrcode().await;
    assert!(result.is_err());

    if let Err(e) = result {
        let error_msg = format!("{}", e);
        assert!(error_msg.contains("Unsupported API mode"));
    }
}

// 注意：实际的Web和TV端登录测试需要真实的网络请求
// 这些测试应该在集成测试中进行，或者使用mock HTTP响应
// 这里我们只测试基本的结构和逻辑

#[test]
fn test_sign_params_order() {
    let manager = AppSignManager::new_tv();

    // 测试参数顺序不影响签名结果（因为内部会排序）
    let mut params1 = HashMap::new();
    params1.insert("z_param".to_string(), "value3".to_string());
    params1.insert("a_param".to_string(), "value1".to_string());
    params1.insert("m_param".to_string(), "value2".to_string());

    let mut params2 = HashMap::new();
    params2.insert("a_param".to_string(), "value1".to_string());
    params2.insert("m_param".to_string(), "value2".to_string());
    params2.insert("z_param".to_string(), "value3".to_string());

    let sign1 = manager.sign_params(&params1);
    let sign2 = manager.sign_params(&params2);

    assert_eq!(sign1, sign2);
}

#[test]
fn test_sign_params_no_ampersand_before_appsec() {
    let manager = AppSignManager::new_tv();

    let mut params = HashMap::new();
    params.insert("test".to_string(), "value".to_string());

    let sign = manager.sign_params(&params);

    // 验证签名是基于 "test=value" + appsec（不是 "test=value&" + appsec）
    let expected_string = format!("test=value{}", "59b43e04ad6965f34319062b478f83dd");
    let expected_sign = format!("{:x}", md5::compute(expected_string.as_bytes()));

    assert_eq!(sign, expected_sign);
}
