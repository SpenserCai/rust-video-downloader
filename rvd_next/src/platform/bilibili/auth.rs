//! Bilibili authentication handling
//!
//! This module provides authentication-related functionality for Bilibili.

use crate::error::Result;
use crate::types::Auth;

/// Build authentication headers for Bilibili requests
pub fn build_auth_headers(auth: Option<&Auth>) -> Vec<(String, String)> {
    let mut headers = Vec::new();

    if let Some(auth) = auth {
        if let Some(ref cookie) = auth.cookie {
            headers.push(("Cookie".to_string(), cookie.clone()));
        }

        if let Some(ref token) = auth.access_token {
            headers.push(("Authorization".to_string(), format!("Bearer {}", token)));
        }

        // Add platform-specific auth headers from extra
        for (key, value) in &auth.extra {
            headers.push((key.clone(), value.clone()));
        }
    }

    headers
}

/// Check if authentication is valid (basic check)
pub fn is_auth_valid(auth: Option<&Auth>) -> bool {
    if let Some(auth) = auth {
        auth.cookie.is_some() || auth.access_token.is_some()
    } else {
        false
    }
}

/// Refresh TV credentials (placeholder for future implementation)
pub async fn refresh_tv_credentials(_auth: &Auth) -> Result<Auth> {
    // TODO: Implement TV token refresh logic
    Err(crate::error::DownloaderError::Auth(
        crate::auth::AuthError::RefreshNotSupported,
    ))
}
