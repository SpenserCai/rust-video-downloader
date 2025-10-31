//! Console utilities for cross-platform terminal support
//!
//! On Windows, this module sets the console to UTF-8 mode to ensure proper display
//! of emoji and Unicode characters. The code page is NOT restored on exit, as modern
//! Windows systems work better with UTF-8 as the default.

#[cfg(windows)]
pub struct ConsoleGuard;

#[cfg(windows)]
impl ConsoleGuard {
    /// Initialize console with UTF-8 support and ANSI escape sequences on Windows.
    ///
    /// Note: The console code page is set to UTF-8 (65001) and is NOT restored on exit.
    /// This is intentional and follows modern Windows best practices, as UTF-8 is the
    /// recommended encoding for console applications on Windows 10/11.
    pub fn new() -> Self {
        use windows_sys::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};

        unsafe {
            // Switch to UTF-8 (code page 65001)
            // We don't restore the original code page because:
            // 1. Modern Windows (10/11) works best with UTF-8
            // 2. Restoring can cause already-output UTF-8 text to display incorrectly
            // 3. Most modern terminal applications expect UTF-8
            SetConsoleOutputCP(65001);
            SetConsoleCP(65001);

            // Enable ANSI escape sequence support
            let _ = nu_ansi_term::enable_ansi_support();
        }

        Self
    }
}

/// On non-Windows platforms, ConsoleGuard is a no-op
#[cfg(not(windows))]
pub struct ConsoleGuard;

#[cfg(not(windows))]
impl ConsoleGuard {
    /// Create a new ConsoleGuard (no-op on non-Windows platforms)
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(windows))]
impl Default for ConsoleGuard {
    fn default() -> Self {
        Self::new()
    }
}
