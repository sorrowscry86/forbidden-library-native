//! Unix-specific platform utilities (macOS, Linux)
//!
//! This module provides Unix/Linux/macOS-specific implementations for system operations.

use std::path::PathBuf;

/// Get the application data directory for Unix-like systems
pub fn get_app_data_dir() -> Option<PathBuf> {
    if let Some(data_dir) = dirs::data_dir() {
        Some(data_dir.join("forbidden-library"))
    } else {
        // Fallback to HOME directory
        dirs::home_dir().map(|home| home.join(".forbidden-library"))
    }
}

/// Get Unix-specific directories
pub fn get_special_folder(folder_type: SpecialFolder) -> Option<PathBuf> {
    match folder_type {
        SpecialFolder::Desktop => dirs::desktop_dir(),
        SpecialFolder::Documents => dirs::document_dir(),
        SpecialFolder::Downloads => dirs::download_dir(),
        SpecialFolder::Pictures => dirs::picture_dir(),
        SpecialFolder::Videos => dirs::video_dir(),
        SpecialFolder::Music => dirs::audio_dir(),
    }
}

/// Unix special folder types
#[derive(Debug, Clone, Copy)]
pub enum SpecialFolder {
    Desktop,
    Documents,
    Downloads,
    Pictures,
    Videos,
    Music,
}

/// Run a shell command (Unix/Linux/macOS)
pub fn run_shell_command(command: &str) -> std::io::Result<String> {
    use std::process::Command;

    let output = Command::new("sh").arg("-c").arg(command).output()?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

/// Check if running with root privileges
pub fn is_root() -> bool {
    std::env::var("USER")
        .map(|user| user == "root")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_data_dir() {
        let app_data = get_app_data_dir();
        assert!(app_data.is_some(), "Data directory should be accessible");
        if let Some(path) = app_data {
            assert!(path.to_string_lossy().contains("forbidden-library"));
        }
    }

    #[test]
    fn test_special_folders() {
        let home = dirs::home_dir();
        assert!(home.is_some(), "Home directory should be accessible");
    }

    #[test]
    fn test_shell_command() {
        let result = run_shell_command("echo 'test'");
        assert!(result.is_ok(), "Shell should be available on Unix");
    }
}
