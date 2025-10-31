pub mod bilibili;

use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[allow(unused_imports)]
pub use bilibili::BilibiliAuthProvider;

/// APP签名管理器（用于TV/APP端API）
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
    /// 签名算法（关键实现细节）：
    /// 1. 将所有参数（不包括sign本身）按key的字典序排序
    /// 2. 拼接为query string格式：key1=value1&key2=value2&key3=value3
    /// 3. 在字符串末尾追加appsec（不加&符号）
    /// 4. 对整个字符串计算MD5哈希值（小写十六进制）
    /// 5. 返回MD5值作为sign参数
    ///
    /// # Arguments
    ///
    /// * `params` - 需要签名的参数
    ///
    /// # Returns
    ///
    /// 返回MD5签名字符串（小写十六进制）
    pub fn sign_params(&self, params: &HashMap<String, String>) -> String {
        // 1. 按key排序（使用BTreeMap自动排序）
        let sorted_params: std::collections::BTreeMap<&String, &String> =
            params.iter().collect();

        // 2. 拼接为query string
        let query_string = sorted_params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        // 3. 在末尾直接追加appsec（不加&）
        let sign_string = format!("{}{}", query_string, self.appsec);

        // 4. 计算MD5（小写十六进制）
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

        // 验证签名格式（应该是32位小写十六进制字符串）
        assert_eq!(sign.len(), 32);
        assert!(sign.chars().all(|c| c.is_ascii_hexdigit() && !c.is_uppercase()));

        // 验证签名的一致性（相同输入应该产生相同输出）
        let sign2 = manager.sign_params(&params);
        assert_eq!(sign, sign2);
    }

    #[test]
    fn test_timestamp() {
        let manager = AppSignManager::new_tv();
        let ts1 = manager.get_timestamp();
        std::thread::sleep(std::time::Duration::from_millis(10));
        let ts2 = manager.get_timestamp();

        // 时间戳应该递增
        assert!(ts2 >= ts1);
        // 时间戳应该是合理的（大于2020年）
        assert!(ts1 > 1577836800); // 2020-01-01
    }
}
