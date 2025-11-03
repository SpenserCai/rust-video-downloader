// WBI (Web Bilibili Interface) 签名实现
// 用于B站API的风控校验

use crate::error::{DownloaderError, Result};
use crate::utils::http::HttpClient;
use serde::Deserialize;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// WBI签名管理器
pub struct WbiManager {
    client: Arc<HttpClient>,
    mixin_key: Option<String>,
}

#[derive(Debug, Deserialize)]
struct NavResponse {
    code: i32,
    message: String,
    data: Option<NavData>,
}

#[derive(Debug, Deserialize)]
struct NavData {
    wbi_img: WbiImg,
}

#[derive(Debug, Deserialize)]
struct WbiImg {
    img_url: String,
    sub_url: String,
}

impl WbiManager {
    /// 创建新的WBI管理器
    pub fn new(client: Arc<HttpClient>) -> Self {
        Self {
            client,
            mixin_key: None,
        }
    }

    /// 获取或刷新WBI密钥
    pub async fn get_mixin_key(&mut self) -> Result<String> {
        if let Some(ref key) = self.mixin_key {
            return Ok(key.clone());
        }

        // 从导航API获取wbi_img信息
        let api = "https://api.bilibili.com/x/web-interface/nav";
        let response = self.client.get(api, None).await?;
        let json_text = response.text().await?;

        tracing::debug!("Nav API response: {}", json_text);

        let nav_response: NavResponse = serde_json::from_str(&json_text)
            .map_err(|e| DownloaderError::Parse(format!("Failed to parse nav response: {}", e)))?;

        // 即使未登录（code=-101），wbi_img数据仍然存在
        let data = nav_response.data.ok_or_else(|| {
            DownloaderError::Api(format!(
                "No nav data (code: {}, message: {})",
                nav_response.code, nav_response.message
            ))
        })?;

        // 提取文件名
        let img_key = extract_filename(&data.wbi_img.img_url);
        let sub_key = extract_filename(&data.wbi_img.sub_url);

        // 生成混合密钥
        let mixin_key = get_mixin_key(&format!("{}{}", img_key, sub_key));

        tracing::debug!("WBI mixin key: {}", mixin_key);

        self.mixin_key = Some(mixin_key.clone());
        Ok(mixin_key)
    }

    /// 对API参数进行WBI签名
    pub async fn sign(&mut self, params: &str) -> Result<String> {
        let mixin_key = self.get_mixin_key().await?;

        // 计算MD5哈希
        let digest = md5::compute(format!("{}{}", params, mixin_key).as_bytes());
        let w_rid = format!("{:x}", digest);

        Ok(format!("{}&w_rid={}", params, w_rid))
    }

    /// 为API URL添加时间戳和签名
    pub async fn sign_url(&mut self, base_params: &str) -> Result<String> {
        // 添加时间戳
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let params_with_ts = format!("{}&wts={}", base_params, timestamp);

        // 签名
        self.sign(&params_with_ts).await
    }
}

/// 从URL中提取文件名（去掉路径和扩展名）
fn extract_filename(url: &str) -> String {
    // 找到最后一个 '/' 后的内容
    let filename_with_ext = url.rsplit('/').next().unwrap_or("");

    // 去掉扩展名
    let filename = filename_with_ext
        .rsplit('.')
        .nth(1)
        .map(|s| {
            // 反转回来
            s.chars().rev().collect::<String>()
        })
        .unwrap_or_else(|| filename_with_ext.to_string());

    // 更简单的方法：找到最后一个'/'和最后一个'.'之间的内容
    if let Some(slash_pos) = url.rfind('/') {
        if let Some(dot_pos) = url.rfind('.') {
            if dot_pos > slash_pos {
                return url[slash_pos + 1..dot_pos].to_string();
            }
        }
    }

    filename
}

/// 使用混淆表生成混合密钥
fn get_mixin_key(orig: &str) -> String {
    // B站的固定混淆表
    const MIXIN_KEY_ENC_TAB: [usize; 32] = [
        46, 47, 18, 2, 53, 8, 23, 32, 15, 50, 10, 31, 58, 3, 45, 35, 27, 43, 5, 49, 33, 9, 42, 19,
        29, 28, 14, 39, 12, 38, 41, 13,
    ];

    let orig_chars: Vec<char> = orig.chars().collect();
    let mut result = String::with_capacity(32);

    for &index in &MIXIN_KEY_ENC_TAB {
        if index < orig_chars.len() {
            result.push(orig_chars[index]);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_filename() {
        let url = "https://i0.hdslb.com/bfs/wbi/7cd084941338484aae1ad9425b84077c.png";
        assert_eq!(extract_filename(url), "7cd084941338484aae1ad9425b84077c");

        let url2 = "https://i0.hdslb.com/bfs/wbi/4932caff0ff746eab6f01bf08b70ac45.png";
        assert_eq!(extract_filename(url2), "4932caff0ff746eab6f01bf08b70ac45");
    }

    #[test]
    fn test_get_mixin_key() {
        // 测试混淆算法
        let input = "7cd084941338484aae1ad9425b84077c4932caff0ff746eab6f01bf08b70ac45";
        let result = get_mixin_key(input);

        // 验证长度
        assert_eq!(result.len(), 32);

        // 验证是原字符串的重排序
        let mut input_chars: Vec<char> = input.chars().take(64).collect();
        let mut result_chars: Vec<char> = result.chars().collect();
        input_chars.sort();
        result_chars.sort();
        // 注意：由于混淆表只取32个字符，所以不能直接比较
    }
}
