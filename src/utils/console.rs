//! Console utilities for cross-platform terminal support
//!
//! On Windows, this module manages console code pages to ensure proper UTF-8 support
//! for emoji and Unicode characters. The original code page is automatically restored
//! when the program exits.

#[cfg(windows)]
pub struct ConsoleGuard {
    original_output_cp: u32,
    original_input_cp: u32,
}

#[cfg(windows)]
impl ConsoleGuard {
    /// Initialize console with UTF-8 support and ANSI escape sequences on Windows.
    /// The original code page will be automatically restored when dropped.
    pub fn new() -> Self {
        use windows_sys::Win32::System::Console::{
            GetConsoleCP, GetConsoleOutputCP, SetConsoleCP, SetConsoleOutputCP,
        };

        unsafe {
            // Save original code pages
            let original_output_cp = GetConsoleOutputCP();
            let original_input_cp = GetConsoleCP();

            // Switch to UTF-8 (code page 65001)
            SetConsoleOutputCP(65001);
            SetConsoleCP(65001);

            // Enable ANSI escape sequence support
            let _ = nu_ansi_term::enable_ansi_support();

            Self {
                original_output_cp,
                original_input_cp,
            }
        }
    }
}

#[cfg(windows)]
impl Drop for ConsoleGuard {
    fn drop(&mut self) {
        use windows_sys::Win32::System::Console::{SetConsoleCP, SetConsoleOutputCP};

        unsafe {
            // Restore original code pages
            SetConsoleOutputCP(self.original_output_cp);
            SetConsoleCP(self.original_input_cp);
        }
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
