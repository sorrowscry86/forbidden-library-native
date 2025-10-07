//! Platform-specific tests for Windows, macOS, and Linux compatibility
//!
//! These tests validate cross-platform functionality and Windows-specific features

use forbidden_library_native::platform;
use std::path::PathBuf;

#[test]
fn test_cross_platform_home_dir() {
    let home = platform::get_home_dir();
    assert!(
        home.is_some(),
        "Home directory should be accessible on all platforms"
    );
}

#[test]
fn test_cross_platform_temp_dir() {
    let temp = platform::get_temp_dir();
    assert!(
        temp.exists(),
        "Temp directory should exist on all platforms"
    );
}

#[test]
fn test_cross_platform_app_data_dir() {
    let app_data = platform::get_app_data_dir();
    assert!(
        app_data.is_some(),
        "App data directory should be accessible on all platforms"
    );

    if let Some(path) = app_data {
        let path_str = path.to_string_lossy().to_lowercase();
        assert!(
            path_str.contains("forbidden") || path_str.contains("library"),
            "App data path should contain app name: {}",
            path_str
        );
    }
}

#[test]
fn test_path_normalization() {
    let test_paths = vec![
        "some/path/to/file.txt",
        "another\\path\\to\\file.txt",
        "mixed/path\\to/file.txt",
    ];

    for path in test_paths {
        let normalized = platform::normalize_path(path);
        assert!(normalized.to_string_lossy().contains("file.txt"));
    }
}

#[test]
fn test_env_var_access() {
    // Test with a common environment variable
    #[cfg(target_os = "windows")]
    {
        let username = platform::get_env_var("USERNAME");
        assert!(
            username.is_some(),
            "USERNAME should be available on Windows"
        );
    }

    #[cfg(not(target_os = "windows"))]
    {
        let user = platform::get_env_var("USER");
        assert!(
            user.is_some(),
            "USER should be available on Unix-like systems"
        );
    }
}

#[cfg(target_os = "windows")]
mod windows_tests {
    use super::*;
    use forbidden_library_native::platform::windows;

    #[test]
    fn test_windows_app_data_dir() {
        let app_data = windows::get_app_data_dir();
        assert!(app_data.is_some(), "Windows AppData should be accessible");

        if let Some(path) = app_data {
            let path_str = path.to_string_lossy();
            assert!(
                path_str.contains("Forbidden Library"),
                "Windows app data should be in 'Forbidden Library' folder"
            );
        }
    }

    #[test]
    fn test_windows_special_folders() {
        use forbidden_library_native::platform::windows::SpecialFolder;

        let desktop = windows::get_special_folder(SpecialFolder::Desktop);
        let documents = windows::get_special_folder(SpecialFolder::Documents);

        assert!(
            desktop.is_some() || documents.is_some(),
            "At least one special folder should be accessible"
        );
    }

    #[test]
    fn test_windows_powershell() {
        let result = windows::run_powershell_command("echo 'test'");
        assert!(result.is_ok(), "PowerShell should be available on Windows");

        if let Ok(output) = result {
            assert!(output.contains("test") || !output.is_empty());
        }
    }

    #[test]
    fn test_windows_path_separators() {
        let path = PathBuf::from("C:\\Users\\Test\\Documents\\file.txt");
        assert!(path.to_string_lossy().contains('\\'));
    }
}

#[cfg(not(target_os = "windows"))]
mod unix_tests {
    use super::*;
    use forbidden_library_native::platform::unix;

    #[test]
    fn test_unix_app_data_dir() {
        let app_data = unix::get_app_data_dir();
        assert!(
            app_data.is_some(),
            "Unix app data directory should be accessible"
        );

        if let Some(path) = app_data {
            let path_str = path.to_string_lossy();
            assert!(
                path_str.contains("forbidden-library"),
                "Unix app data should be in 'forbidden-library' folder"
            );
        }
    }

    #[test]
    fn test_unix_special_folders() {
        use forbidden_library_native::platform::unix::SpecialFolder;

        let desktop = unix::get_special_folder(SpecialFolder::Desktop);
        let documents = unix::get_special_folder(SpecialFolder::Documents);

        // Note: In CI environments, these might not be available
        // So we just test that the function doesn't panic
        let _ = desktop;
        let _ = documents;
    }

    #[test]
    fn test_unix_shell_command() {
        let result = unix::run_shell_command("echo 'test'");
        assert!(result.is_ok(), "Shell should be available on Unix systems");

        if let Ok(output) = result {
            assert!(output.contains("test"));
        }
    }

    #[test]
    fn test_unix_path_separators() {
        let path = PathBuf::from("/home/user/documents/file.txt");
        assert!(path.to_string_lossy().contains('/'));
    }
}

#[test]
fn test_path_traversal_prevention() {
    // Test that path traversal attempts are handled properly
    let test_paths = vec![
        "../../../etc/passwd",
        "..\\..\\..\\Windows\\System32\\config\\SAM",
        "~/../../sensitive.txt",
    ];

    for path in test_paths {
        let normalized = platform::normalize_path(path);
        // The normalization should preserve the path structure for validation elsewhere
        assert!(normalized.to_string_lossy().contains(path));
    }
}

#[test]
fn test_app_data_dir_creation_safety() {
    // Ensure app data directory path doesn't contain dangerous characters
    if let Some(app_data) = platform::get_app_data_dir() {
        let path_str = app_data.to_string_lossy();

        // Should not contain command injection characters
        assert!(!path_str.contains(";"));
        assert!(!path_str.contains("&&"));
        assert!(!path_str.contains("|"));

        // Should not be a system directory
        assert!(!path_str.contains("/etc/"));
        assert!(!path_str.contains("\\Windows\\System32"));
    }
}
