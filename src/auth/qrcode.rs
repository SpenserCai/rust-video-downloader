/*
 * @Author: SpenserCai
 * @Date: 2025-10-31 15:00:00
 * @version:
 * @LastEditors: SpenserCai
 * @LastEditTime: 2025-10-31 15:00:00
 * @Description: QR code display module
 */
//! 二维码显示模块
//!
//! 提供跨平台的二维码显示和保存功能

use crate::auth::types::AuthError;
use crate::error::Result;
use qrcode::QrCode;
use std::path::Path;

/// 二维码显示器
pub struct QRCodeDisplay;

impl QRCodeDisplay {
    /// 在终端显示二维码（跨平台兼容）
    ///
    /// # Arguments
    ///
    /// * `url` - 二维码URL
    ///
    /// # Returns
    ///
    /// 成功返回Ok(())，失败返回错误
    pub fn display_in_terminal(url: &str) -> Result<()> {
        let code = Self::generate_qrcode(url)?;

        // 检测终端类型并选择显示方式
        if Self::is_windows_powershell() {
            Self::display_unicode_blocks(&code)?;
        } else {
            Self::display_ansi_colors(&code)?;
        }

        Ok(())
    }

    /// 保存二维码为PNG图片文件
    ///
    /// # Arguments
    ///
    /// * `url` - 二维码URL
    /// * `path` - 保存路径
    ///
    /// # Returns
    ///
    /// 成功返回Ok(())，失败返回错误
    pub fn save_to_file(url: &str, path: &Path) -> Result<()> {
        let code = Self::generate_qrcode(url)?;

        // 使用qrcode crate的render功能生成PNG
        let image = code
            .render::<image::Luma<u8>>()
            .min_dimensions(200, 200) // 最小尺寸
            .module_dimensions(7, 7) // 每个模块7x7像素
            .build();

        image.save(path).map_err(|e| {
            AuthError::QRCodeSaveError(format!("Failed to save QR code image: {}", e))
        })?;

        tracing::debug!("QR code saved to: {}", path.display());

        Ok(())
    }

    /// 生成二维码数据
    ///
    /// # Arguments
    ///
    /// * `url` - 二维码URL
    ///
    /// # Returns
    ///
    /// 返回QrCode对象
    fn generate_qrcode(url: &str) -> Result<QrCode> {
        QrCode::new(url).map_err(|e| {
            AuthError::QRCodeDisplayError(format!("Failed to generate QR code: {}", e)).into()
        })
    }

    /// 检测是否为 Windows PowerShell
    ///
    /// # Returns
    ///
    /// 如果是PowerShell返回true，否则返回false
    #[cfg(target_os = "windows")]
    fn is_windows_powershell() -> bool {
        std::env::var("PSModulePath").is_ok()
    }

    #[cfg(not(target_os = "windows"))]
    fn is_windows_powershell() -> bool {
        false
    }

    /// 使用 Unicode 块字符显示（PowerShell）
    ///
    /// # Arguments
    ///
    /// * `code` - QrCode对象
    ///
    /// # Returns
    ///
    /// 成功返回Ok(())，失败返回错误
    fn display_unicode_blocks(code: &QrCode) -> Result<()> {
        tracing::info!("请使用手机扫描以下二维码：");
        tracing::info!("");

        // 添加上边距
        tracing::info!("    {}", "██".repeat(code.width() + 2));

        // 遍历二维码矩阵
        for y in 0..code.width() {
            let mut line = String::from("    ██"); // 左边距

            for x in 0..code.width() {
                // 反色显示：黑色模块用空格，白色模块用██
                if code[(x, y)] == qrcode::Color::Dark {
                    line.push_str("  "); // 黑色
                } else {
                    line.push_str("██"); // 白色
                }
            }

            line.push_str("██"); // 右边距
            tracing::info!("{}", line);
        }

        // 添加下边距
        tracing::info!("    {}", "██".repeat(code.width() + 2));
        tracing::info!("");

        Ok(())
    }

    /// 使用 ANSI 颜色显示（Unix终端）
    ///
    /// # Arguments
    ///
    /// * `code` - QrCode对象
    ///
    /// # Returns
    ///
    /// 成功返回Ok(())，失败返回错误
    fn display_ansi_colors(code: &QrCode) -> Result<()> {
        tracing::info!("请使用手机扫描以下二维码：");
        tracing::info!("");

        // 添加上边距（白色）
        let top_border = format!("    {}", "\x1b[47m  \x1b[0m".repeat(code.width() + 2));
        tracing::info!("{}", top_border);

        // 遍历二维码矩阵
        for y in 0..code.width() {
            let mut line = String::from("    \x1b[47m  \x1b[0m"); // 左边距（白色）

            for x in 0..code.width() {
                // 黑色模块用黑色背景，白色模块用白色背景
                if code[(x, y)] == qrcode::Color::Dark {
                    line.push_str("\x1b[40m  \x1b[0m"); // 黑色
                } else {
                    line.push_str("\x1b[47m  \x1b[0m"); // 白色
                }
            }

            line.push_str("\x1b[47m  \x1b[0m"); // 右边距（白色）
            tracing::info!("{}", line);
        }

        // 添加下边距（白色）
        let bottom_border = format!("    {}", "\x1b[47m  \x1b[0m".repeat(code.width() + 2));
        tracing::info!("{}", bottom_border);
        tracing::info!("");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_generate_qrcode() {
        let url = "https://www.bilibili.com/";
        let result = QRCodeDisplay::generate_qrcode(url);
        assert!(result.is_ok());
    }

    #[test]
    fn test_save_to_file() {
        let url = "https://www.bilibili.com/";
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_qrcode.png");

        let result = QRCodeDisplay::save_to_file(url, &file_path);
        assert!(result.is_ok());

        // 验证文件存在
        assert!(file_path.exists());

        // 验证文件大小大于0
        let metadata = fs::metadata(&file_path).unwrap();
        assert!(metadata.len() > 0);
    }

    #[test]
    fn test_is_windows_powershell() {
        // 这个测试只验证函数可以调用，不验证具体结果
        let _ = QRCodeDisplay::is_windows_powershell();
    }
}
