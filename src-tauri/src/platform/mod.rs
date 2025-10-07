//! Platform-specific utilities and abstractions
//!
//! This module provides cross-platform abstractions for system-specific functionality,
//! ensuring the Forbidden Library works seamlessly on Windows, macOS, and Linux.

use std::path::PathBuf;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(not(target_os = "windows"))]
pub mod unix;

/// Get the application data directory in a cross-platform way
pub fn get_app_data_dir() -> Option<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        windows::get_app_data_dir()
    }

    #[cfg(not(target_os = "windows"))]
    {
        unix::get_app_data_dir()
    }
}

/// Get the user's home directory in a cross-platform way
pub fn get_home_dir() -> Option<PathBuf> {
    dirs::home_dir()
}

/// Get the temporary directory in a cross-platform way
pub fn get_temp_dir() -> PathBuf {
    std::env::temp_dir()
}

/// Get an environment variable in a cross-platform way
pub fn get_env_var(name: &str) -> Option<String> {
    std::env::var(name).ok()
}

/// Normalize a path for the current platform
pub fn normalize_path(path: &str) -> PathBuf {
    PathBuf::from(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_home_dir() {
        let home = get_home_dir();
        assert!(home.is_some(), "Home directory should be accessible");
    }

    #[test]
    fn test_temp_dir() {
        let temp = get_temp_dir();
        assert!(temp.exists(), "Temp directory should exist");
    }

    #[test]
    fn test_path_normalization() {
        let path = normalize_path("some/path/to/file.txt");
        assert!(path.to_string_lossy().contains("file.txt"));
    }
}
