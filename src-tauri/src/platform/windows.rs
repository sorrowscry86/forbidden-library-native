//! Windows-specific platform utilities
//!
//! This module provides Windows-specific implementations for system operations.

use std::path::PathBuf;

/// Get the Windows AppData directory for the application
pub fn get_app_data_dir() -> Option<PathBuf> {
    if let Some(appdata) = dirs::data_dir() {
        Some(appdata.join("Forbidden Library"))
    } else {
        // Fallback to APPDATA environment variable
        std::env::var("APPDATA")
            .ok()
            .map(|path| PathBuf::from(path).join("Forbidden Library"))
    }
}

/// Get Windows-specific special folders
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

/// Windows special folder types
#[derive(Debug, Clone, Copy)]
pub enum SpecialFolder {
    Desktop,
    Documents,
    Downloads,
    Pictures,
    Videos,
    Music,
}

/// Run a PowerShell command (Windows only)
pub fn run_powershell_command(command: &str) -> std::io::Result<String> {
    use std::process::Command;
    
    let output = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-NonInteractive")
        .arg("-Command")
        .arg(command)
        .output()?;
    
    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ))
    }
}

/// Check if running with administrator privileges
pub fn is_admin() -> bool {
    // This is a simplified check - in production you'd want to use Windows API
    std::env::var("USERNAME")
        .map(|user| user.to_lowercase() == "administrator")
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_data_dir() {
        let app_data = get_app_data_dir();
        assert!(app_data.is_some(), "AppData directory should be accessible");
        if let Some(path) = app_data {
            assert!(path.to_string_lossy().contains("Forbidden Library"));
        }
    }

    #[test]
    fn test_special_folders() {
        // These might not all be available in CI/test environments
        let desktop = get_special_folder(SpecialFolder::Desktop);
        let documents = get_special_folder(SpecialFolder::Documents);
        
        // At least one should be available
        assert!(
            desktop.is_some() || documents.is_some(),
            "At least one special folder should be accessible"
        );
    }

    #[test]
    fn test_powershell_command() {
        // Simple PowerShell command that should work
        let result = run_powershell_command("echo 'test'");
        assert!(result.is_ok(), "PowerShell should be available on Windows");
    }
}
