//! Console utilities for cross-platform terminal support
//!
//! On Windows, this module configures the console for proper UTF-8 and emoji support.
//! This includes setting the code page, enabling virtual terminal processing, and
//! configuring the console output mode.

#[cfg(windows)]
pub struct ConsoleGuard;

#[cfg(windows)]
impl ConsoleGuard {
    /// Initialize console with full UTF-8 and emoji support on Windows.
    ///
    /// This function:
    /// 1. Sets console code page to UTF-8 (65001)
    /// 2. Enables ANSI escape sequence support
    /// 3. Enables virtual terminal processing for better Unicode support
    /// 4. Configures console mode for proper emoji rendering
    ///
    /// Note: The console code page is NOT restored on exit, following modern Windows
    /// best practices where UTF-8 is the recommended default encoding.
    pub fn new() -> Self {
        use windows_sys::Win32::Foundation::INVALID_HANDLE_VALUE;
        use windows_sys::Win32::System::Console::{
            GetConsoleMode, GetStdHandle, SetConsoleCP, SetConsoleMode, SetConsoleOutputCP,
            ENABLE_PROCESSED_OUTPUT, ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            ENABLE_WRAP_AT_EOL_OUTPUT, STD_OUTPUT_HANDLE,
        };

        unsafe {
            // Set console code page to UTF-8 (65001)
            SetConsoleOutputCP(65001);
            SetConsoleCP(65001);

            // Get stdout handle
            let stdout_handle = GetStdHandle(STD_OUTPUT_HANDLE);

            if stdout_handle != INVALID_HANDLE_VALUE && !stdout_handle.is_null() {
                let mut mode: u32 = 0;

                // Get current console mode
                if GetConsoleMode(stdout_handle, &mut mode) != 0 {
                    // Enable virtual terminal processing and other necessary flags
                    // ENABLE_VIRTUAL_TERMINAL_PROCESSING: Enables ANSI escape sequences and better Unicode support
                    // ENABLE_PROCESSED_OUTPUT: Enables processing of control characters
                    // ENABLE_WRAP_AT_EOL_OUTPUT: Enables automatic line wrapping
                    mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING
                        | ENABLE_PROCESSED_OUTPUT
                        | ENABLE_WRAP_AT_EOL_OUTPUT;

                    SetConsoleMode(stdout_handle, mode);
                }
            }

            // Also enable ANSI support through nu-ansi-term (fallback/compatibility)
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

/// Check if the current terminal likely supports emoji display
///
/// This is a heuristic check that considers:
/// - On Windows: Whether we're running in Windows Terminal (best emoji support)
/// - On other platforms: Assumes emoji support is available
///
/// Note: This is not 100% accurate but provides a reasonable guess for fallback behavior.
#[cfg(windows)]
#[allow(dead_code)]
pub fn supports_emoji() -> bool {
    // Windows Terminal sets the WT_SESSION environment variable
    // It has the best emoji support on Windows
    if std::env::var("WT_SESSION").is_ok() {
        return true;
    }

    // Check if we're in a modern terminal that might support emoji
    // VS Code terminal sets TERM_PROGRAM
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        if term_program.contains("vscode") {
            return true;
        }
    }

    // For traditional PowerShell/CMD, emoji support depends on font
    // We'll assume it's available after our console setup, but users
    // may need to change their font to "Cascadia Code" or "Segoe UI Emoji"
    true
}

#[cfg(not(windows))]
#[allow(dead_code)]
pub fn supports_emoji() -> bool {
    // Most modern Unix terminals support emoji
    true
}
