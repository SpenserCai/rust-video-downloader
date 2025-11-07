//! APP签名管理器（用于TV/APP端API）
//!
//! 提供Bilibili TV/APP端API所需的签名功能

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// APP签名管理器
pub struct AppSignManager {
    appkey: String,
    appsec: String,
}

impl AppSignManager {
    /// 创建TV端签名管理器
    pub fn new_tv() -> Self {
        Self {
            appkey: "4409e2ce8ffd12b8".to_string(),
            appsec: "59b43e04ad6965f34319062b478f83dd".to_string(),
        }
    }

    /// 对参数进行签名
    ///
    /// 签名算法：
    /// 1. 将所有参数按key的字典序排序
    /// 2. 拼接为query string格式
    /// 3. 在字符串末尾追加appsec
    /// 4. 计算MD5哈希值
    pub fn sign_params(&self, params: &HashMap<String, String>) -> String {
        let sorted_params: std::collections::BTreeMap<&String, &String> = params.iter().collect();

        let query_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let sign_string = format!("{}{}", query_string, self.appsec);
        let digest = md5::compute(sign_string.as_bytes());
        format!("{:x}", digest)
    }

    /// 获取当前Unix时间戳（秒）
    pub fn get_timestamp(&self) -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
    }

    /// 获取appkey
    pub fn appkey(&self) -> &str {
        &self.appkey
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sign_params() {
        let manager = AppSignManager::new_tv();
        let mut params = HashMap::new();
        params.insert("appkey".to_string(), "4409e2ce8ffd12b8".to_string());
        params.insert("local_id".to_string(), "0".to_string());
        params.insert("ts".to_string(), "1234567890".to_string());

        let sign = manager.sign_params(&params);
        assert_eq!(sign.len(), 32);
        assert!(sign
            .chars()
            .all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));
    }
}
