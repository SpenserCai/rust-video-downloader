use crate::auth::types::{AuthError, Credentials, LoginStatus, QRCodeData};
use crate::auth::AuthProvider;
use crate::error::Result;
use crate::platform::bilibili::ApiMode;
use crate::utils::http::HttpClient;
use async_trait::async_trait;
use reqwest::Response;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

use super::AppSignManager;

/// 哔哩哔哩认证提供者
#[allow(dead_code)]
pub struct BilibiliAuthProvider {
    client: Arc<HttpClient>,
    api_mode: ApiMode,
    sign_manager: Option<AppSignManager>,
}

impl BilibiliAuthProvider {
    /// 创建新的哔哩哔哩认证提供者
    #[allow(dead_code)]
    ///
    /// # Arguments
    ///
    /// * `client` - HTTP客户端
    /// * `api_mode` - API模式（Web或TV）
    pub fn new(client: Arc<HttpClient>, api_mode: ApiMode) -> Self {
        let sign_manager = match api_mode {
            ApiMode::TV => Some(AppSignManager::new_tv()),
            _ => None,
        };

        Self {
            client,
            api_mode,
            sign_manager,
        }
    }

    /// Web端申请二维码
    ///
    /// API: https://passport.bilibili.com/x/passport-login/web/qrcode/generate
    #[allow(dead_code)]
    async fn request_web_qrcode(&self) -> Result<QRCodeData> {
        tracing::debug!("Requesting Web QR code");

        let url = "https://passport.bilibili.com/x/passport-login/web/qrcode/generate";
        let response = self.client.get(url, None).await?;

        let json: Value = response.json().await.map_err(|e| {
            AuthError::InvalidResponse(format!("Failed to parse JSON response: {}", e))
        })?;

        // 检查响应码
        let code = json["code"]
            .as_i64()
            .ok_or_else(|| AuthError::InvalidResponse("Missing 'code' field".to_string()))?;

        if code != 0 {
            let message = json["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();
            return Err(AuthError::InvalidResponse(format!(
                "API returned error code {}: {}",
                code, message
            ))
            .into());
        }

        // 提取二维码数据
        let url = json["data"]["url"]
            .as_str()
            .ok_or_else(|| AuthError::InvalidResponse("Missing 'data.url' field".to_string()))?
            .to_string();

        let qrcode_key = json["data"]["qrcode_key"]
            .as_str()
            .ok_or_else(|| {
                AuthError::InvalidResponse("Missing 'data.qrcode_key' field".to_string())
            })?
            .to_string();

        tracing::debug!("Web QR code generated successfully");

        Ok(QRCodeData {
            url,
            key: qrcode_key,
        })
    }

    /// Web端轮询登录状态
    ///
    /// API: https://passport.bilibili.com/x/passport-login/web/qrcode/poll
    #[allow(dead_code)]
    async fn poll_web_login(&self, qrcode_key: &str) -> Result<LoginStatus> {
        let url = format!(
            "https://passport.bilibili.com/x/passport-login/web/qrcode/poll?qrcode_key={}",
            qrcode_key
        );

        // 发送请求并保存完整的Response对象
        let response = self.client.get(&url, None).await?;

        // 先提取Cookie（在消费response body之前）
        let cookies = self.extract_cookies_from_response(&response)?;

        // 然后解析JSON
        let json: Value = response.json().await.map_err(|e| {
            AuthError::InvalidResponse(format!("Failed to parse JSON response: {}", e))
        })?;

        // 检查data.code字段（数字类型）
        let data_code = json["data"]["code"]
            .as_i64()
            .ok_or_else(|| AuthError::InvalidResponse("Missing 'data.code' field".to_string()))?;

        match data_code {
            0 => {
                // 登录成功
                tracing::debug!("Web login successful");

                // 如果Cookie为空，说明提取失败
                if cookies.is_empty() {
                    return Err(AuthError::InvalidResponse(
                        "Login successful but no cookies found".to_string(),
                    )
                    .into());
                }

                Ok(LoginStatus::Success(Credentials {
                    cookie: Some(cookies),
                    access_token: None,
                    refresh_token: None,
                    expires_at: None,
                    mid: None,
                }))
            }
            86101 => {
                // 未扫码
                Ok(LoginStatus::Pending)
            }
            86090 => {
                // 已扫码未确认
                Ok(LoginStatus::Scanned)
            }
            86038 => {
                // 二维码已失效
                Ok(LoginStatus::Expired)
            }
            _ => {
                let message = json["data"]["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string();
                Ok(LoginStatus::Failed(format!(
                    "Unexpected code {}: {}",
                    data_code, message
                )))
            }
        }
    }

    /// 从响应头提取所有Cookie（Web端使用）
    ///
    /// 从HTTP响应头的Set-Cookie字段提取：SESSDATA, bili_jct, DedeUserID, DedeUserID__ckMd5, sid
    ///
    /// # Arguments
    ///
    /// * `response` - HTTP响应对象
    ///
    /// # Returns
    ///
    /// 返回拼接好的Cookie字符串
    #[allow(dead_code)]
    fn extract_cookies_from_response(&self, response: &Response) -> Result<String> {
        let mut cookie_map: HashMap<String, String> = HashMap::new();

        // 遍历所有Set-Cookie响应头
        for cookie_header in response.headers().get_all("set-cookie") {
            if let Ok(cookie_str) = cookie_header.to_str() {
                // 解析Cookie字符串，只提取name=value部分
                if let Some(name_value) = cookie_str.split(';').next() {
                    if let Some((name, value)) = name_value.split_once('=') {
                        cookie_map.insert(name.trim().to_string(), value.trim().to_string());
                    }
                }
            }
        }

        // 按固定顺序提取需要的Cookie
        let required_cookies = ["SESSDATA", "bili_jct", "DedeUserID", "DedeUserID__ckMd5", "sid"];
        let mut cookie_parts = Vec::new();

        for cookie_name in &required_cookies {
            if let Some(cookie_value) = cookie_map.get(*cookie_name) {
                cookie_parts.push(format!("{}={}", cookie_name, cookie_value));
            }
        }

        // 拼接为字符串，格式：name1=value1; name2=value2
        let cookies = cookie_parts.join("; ");

        tracing::debug!(
            "Extracted {} cookies from response",
            cookie_parts.len()
        );

        Ok(cookies)
    }

    /// TV端申请二维码
    ///
    /// API: https://passport.snm0516.aisee.tv/x/passport-tv-login/qrcode/auth_code
    #[allow(dead_code)]
    async fn request_tv_qrcode(&self) -> Result<QRCodeData> {
        tracing::debug!("Requesting TV QR code");

        let sign_manager = self
            .sign_manager
            .as_ref()
            .ok_or_else(|| AuthError::LoginFailed("Sign manager not initialized".to_string()))?;

        // 准备参数
        let mut params = HashMap::new();
        params.insert("appkey".to_string(), sign_manager.appkey().to_string());
        params.insert("local_id".to_string(), "0".to_string());
        params.insert("ts".to_string(), sign_manager.get_timestamp().to_string());

        // 生成签名
        let sign = sign_manager.sign_params(&params);
        params.insert("sign".to_string(), sign);

        // 构建请求体（application/x-www-form-urlencoded）
        let body = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let url = "https://passport.snm0516.aisee.tv/x/passport-tv-login/qrcode/auth_code";

        // 设置请求头
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded"
                .parse()
                .map_err(|e| AuthError::Network(format!("Invalid header: {}", e)))?,
        );

        let response = self.client.post(url, &body, Some(headers)).await?;

        let json: Value = response.json().await.map_err(|e| {
            AuthError::InvalidResponse(format!("Failed to parse JSON response: {}", e))
        })?;

        // 检查响应码
        let code = json["code"]
            .as_i64()
            .ok_or_else(|| AuthError::InvalidResponse("Missing 'code' field".to_string()))?;

        if code != 0 {
            let message = json["message"]
                .as_str()
                .unwrap_or("Unknown error")
                .to_string();
            return Err(AuthError::InvalidResponse(format!(
                "API returned error code {}: {}",
                code, message
            ))
            .into());
        }

        // 提取二维码数据
        let url = json["data"]["url"]
            .as_str()
            .ok_or_else(|| AuthError::InvalidResponse("Missing 'data.url' field".to_string()))?
            .to_string();

        let auth_code = json["data"]["auth_code"]
            .as_str()
            .ok_or_else(|| {
                AuthError::InvalidResponse("Missing 'data.auth_code' field".to_string())
            })?
            .to_string();

        tracing::debug!("TV QR code generated successfully");

        Ok(QRCodeData {
            url,
            key: auth_code,
        })
    }

    /// TV端轮询登录状态
    ///
    /// API: https://passport.bilibili.com/x/passport-tv-login/qrcode/poll
    /// 注意：不是snm0516域名
    #[allow(dead_code)]
    async fn poll_tv_login(&self, auth_code: &str) -> Result<LoginStatus> {
        let sign_manager = self
            .sign_manager
            .as_ref()
            .ok_or_else(|| AuthError::LoginFailed("Sign manager not initialized".to_string()))?;

        // 准备参数（每次轮询都要更新ts）
        let mut params = HashMap::new();
        params.insert("appkey".to_string(), sign_manager.appkey().to_string());
        params.insert("auth_code".to_string(), auth_code.to_string());
        params.insert("local_id".to_string(), "0".to_string());
        params.insert("ts".to_string(), sign_manager.get_timestamp().to_string());

        // 生成签名
        let sign = sign_manager.sign_params(&params);
        params.insert("sign".to_string(), sign);

        // 构建请求体
        let body = params
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");

        let url = "https://passport.bilibili.com/x/passport-tv-login/qrcode/poll";

        // 设置请求头
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "Content-Type",
            "application/x-www-form-urlencoded"
                .parse()
                .map_err(|e| AuthError::Network(format!("Invalid header: {}", e)))?,
        );

        let response = self.client.post(url, &body, Some(headers)).await?;

        let json: Value = response.json().await.map_err(|e| {
            AuthError::InvalidResponse(format!("Failed to parse JSON response: {}", e))
        })?;

        // 检查根级别的code字段（字符串类型）
        let code_str = json["code"]
            .as_str()
            .ok_or_else(|| AuthError::InvalidResponse("Missing 'code' field".to_string()))?;

        match code_str {
            "0" => {
                // 登录成功
                tracing::debug!("TV login successful");
                let credentials = self.extract_tv_credentials(&json)?;
                Ok(LoginStatus::Success(credentials))
            }
            "86039" => {
                // 未扫码（TV端特有）
                Ok(LoginStatus::Pending)
            }
            "86090" => {
                // 已扫码未确认
                Ok(LoginStatus::Scanned)
            }
            "86038" => {
                // 二维码已失效
                Ok(LoginStatus::Expired)
            }
            _ => {
                let message = json["message"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string();
                Ok(LoginStatus::Failed(format!(
                    "Unexpected code {}: {}",
                    code_str, message
                )))
            }
        }
    }

    /// 从TV端响应中提取Cookie和Token
    ///
    /// TV端返回结构：data.cookie_info.cookies数组 + data.access_token + data.refresh_token
    #[allow(dead_code)]
    fn extract_tv_credentials(&self, json: &Value) -> Result<Credentials> {
        // 提取access_token
        let access_token = json["data"]["access_token"]
            .as_str()
            .ok_or_else(|| {
                AuthError::InvalidResponse("Missing 'data.access_token' field".to_string())
            })?
            .to_string();

        // 提取refresh_token
        let refresh_token = json["data"]["refresh_token"]
            .as_str()
            .ok_or_else(|| {
                AuthError::InvalidResponse("Missing 'data.refresh_token' field".to_string())
            })?
            .to_string();

        // 提取mid（用户ID）
        let mid = json["data"]["mid"].as_u64();

        // 提取expires_in并计算expires_at
        let expires_at = json["data"]["expires_in"].as_u64().map(|expires_in| {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs()
                + expires_in
        });

        // 提取Cookie
        let cookies_array = json["data"]["cookie_info"]["cookies"]
            .as_array()
            .ok_or_else(|| {
                AuthError::InvalidResponse("Missing 'data.cookie_info.cookies' field".to_string())
            })?;

        let mut cookie_map: HashMap<String, String> = HashMap::new();
        for cookie_obj in cookies_array {
            if let (Some(name), Some(value)) = (
                cookie_obj["name"].as_str(),
                cookie_obj["value"].as_str(),
            ) {
                cookie_map.insert(name.to_string(), value.to_string());
            }
        }

        // 按固定顺序提取需要的Cookie
        let required_cookies = ["SESSDATA", "bili_jct", "DedeUserID", "DedeUserID__ckMd5", "sid"];
        let mut cookie_parts = Vec::new();

        for cookie_name in &required_cookies {
            if let Some(cookie_value) = cookie_map.get(*cookie_name) {
                cookie_parts.push(format!("{}={}", cookie_name, cookie_value));
            }
        }

        // 拼接为字符串
        let cookies = if cookie_parts.is_empty() {
            None
        } else {
            Some(cookie_parts.join("; "))
        };

        tracing::debug!(
            "Extracted TV credentials: {} cookies, access_token present",
            cookie_parts.len()
        );

        Ok(Credentials {
            cookie: cookies,
            access_token: Some(access_token),
            refresh_token: Some(refresh_token),
            expires_at,
            mid,
        })
    }
}

#[async_trait]
impl AuthProvider for BilibiliAuthProvider {
    async fn request_qrcode(&self) -> Result<QRCodeData> {
        match self.api_mode {
            ApiMode::Web => self.request_web_qrcode().await,
            ApiMode::TV => self.request_tv_qrcode().await,
            _ => Err(AuthError::LoginFailed(format!(
                "Unsupported API mode for login: {:?}",
                self.api_mode
            ))
            .into()),
        }
    }

    async fn poll_login_status(&self, key: &str) -> Result<LoginStatus> {
        match self.api_mode {
            ApiMode::Web => self.poll_web_login(key).await,
            ApiMode::TV => self.poll_tv_login(key).await,
            _ => Err(AuthError::LoginFailed(format!(
                "Unsupported API mode for login: {:?}",
                self.api_mode
            ))
            .into()),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_cookie_extraction() {
        // 这个测试需要mock HTTP响应，暂时跳过
        // 实际测试将在集成测试中进行
    }
}
