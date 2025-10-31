use rvd::auth::providers::AppSignManager;
use std::collections::HashMap;

#[test]
fn test_app_sign_manager_tv() {
    let manager = AppSignManager::new_tv();
    
    // 测试基本签名功能
    let mut params = HashMap::new();
    params.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params.insert("local_id".to_string(), "0".to_string());
    params.insert("ts".to_string(), "1234567890".to_string());
    
    let sign = manager.sign_params(&params);
    
    // 验证签名格式
    assert_eq!(sign.len(), 32, "Sign should be 32 characters (MD5 hex)");
    assert!(
        sign.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()),
        "Sign should be lowercase hexadecimal"
    );
}

#[test]
fn test_sign_consistency() {
    let manager = AppSignManager::new_tv();
    
    let mut params = HashMap::new();
    params.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params.insert("local_id".to_string(), "0".to_string());
    params.insert("ts".to_string(), "1234567890".to_string());
    
    // 相同的输入应该产生相同的签名
    let sign1 = manager.sign_params(&params);
    let sign2 = manager.sign_params(&params);
    
    assert_eq!(sign1, sign2, "Same input should produce same signature");
}

#[test]
fn test_sign_order_independence() {
    let manager = AppSignManager::new_tv();
    
    // 测试参数顺序不影响签名（因为内部会排序）
    let mut params1 = HashMap::new();
    params1.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params1.insert("local_id".to_string(), "0".to_string());
    params1.insert("ts".to_string(), "1234567890".to_string());
    
    let mut params2 = HashMap::new();
    params2.insert("ts".to_string(), "1234567890".to_string());
    params2.insert("local_id".to_string(), "0".to_string());
    params2.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    
    let sign1 = manager.sign_params(&params1);
    let sign2 = manager.sign_params(&params2);
    
    assert_eq!(
        sign1, sign2,
        "Parameter order should not affect signature"
    );
}

#[test]
fn test_sign_different_params() {
    let manager = AppSignManager::new_tv();
    
    let mut params1 = HashMap::new();
    params1.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params1.insert("local_id".to_string(), "0".to_string());
    params1.insert("ts".to_string(), "1234567890".to_string());
    
    let mut params2 = HashMap::new();
    params2.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params2.insert("local_id".to_string(), "0".to_string());
    params2.insert("ts".to_string(), "9876543210".to_string()); // 不同的时间戳
    
    let sign1 = manager.sign_params(&params1);
    let sign2 = manager.sign_params(&params2);
    
    assert_ne!(
        sign1, sign2,
        "Different parameters should produce different signatures"
    );
}

#[test]
fn test_timestamp_generation() {
    let manager = AppSignManager::new_tv();
    
    let ts1 = manager.get_timestamp();
    std::thread::sleep(std::time::Duration::from_millis(10));
    let ts2 = manager.get_timestamp();
    
    // 时间戳应该递增
    assert!(ts2 >= ts1, "Timestamp should be monotonically increasing");
    
    // 时间戳应该是合理的（大于2020年1月1日）
    assert!(
        ts1 > 1577836800,
        "Timestamp should be after 2020-01-01 ({})",
        ts1
    );
    
    // 时间戳应该小于2100年1月1日（合理性检查）
    assert!(
        ts1 < 4102444800,
        "Timestamp should be before 2100-01-01 ({})",
        ts1
    );
}

#[test]
fn test_sign_with_special_characters() {
    let manager = AppSignManager::new_tv();
    
    // 测试包含特殊字符的参数
    let mut params = HashMap::new();
    params.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
    params.insert("local_id".to_string(), "0".to_string());
    params.insert("ts".to_string(), "1234567890".to_string());
    params.insert("test".to_string(), "value with spaces".to_string());
    
    let sign = manager.sign_params(&params);
    
    // 应该能够处理特殊字符而不崩溃
    assert_eq!(sign.len(), 32);
    assert!(sign.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_sign_empty_params() {
    let manager = AppSignManager::new_tv();
    
    // 测试空参数
    let params = HashMap::new();
    let sign = manager.sign_params(&params);
    
    // 空参数应该只签名appsec本身
    assert_eq!(sign.len(), 32);
    assert!(sign.chars().all(|c| c.is_ascii_hexdigit()));
}

#[test]
fn test_appkey_getter() {
    let manager = AppSignManager::new_tv();
    
    // 验证appkey getter
    assert_eq!(manager.appkey(), "4409e2ce8ffd12b8");
}
