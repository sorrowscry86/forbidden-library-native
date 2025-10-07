//! Comprehensive error handling system for Forbidden Library
//!
//! This module defines a unified error type system using the thiserror crate
//! to provide consistent, type-safe error handling across all backend components.
//! All errors are designed to be user-friendly and properly categorized.

use thiserror::Error;

/// Unified application error type for consistent error handling
/// All backend operations should return AppResult<T> instead of various Result types
#[derive(Error, Debug)]
pub enum AppError {
    /// Database-related errors (connection, queries, schema)
    #[error("Database error: {message}")]
    Database { message: String },

    /// I/O errors (file operations, network requests)
    #[error("I/O error: {message}")]
    Io { message: String },

    /// Input validation errors (user input, parameter validation)
    #[error("Validation error: {message}")]
    Validation { message: String },

    /// Resource not found errors (conversations, messages, personas)
    #[error("Not found: {message}")]
    NotFound { message: String },

    /// External API errors (AI service calls, network issues)
    #[error("API error: {message}")]
    Api { message: String },

    /// Encryption/decryption errors (SQLCipher, data security)
    #[error("Encryption error: {message}")]
    Encryption { message: String },

    /// Unexpected errors (system failures, unhandled cases)
    #[error("Unexpected error: {message}")]
    Unexpected { message: String },
}

/// Type alias for consistent Result usage across the application
pub type AppResult<T> = Result<T, AppError>;

// Implement From traits for common error types for easy conversion
impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::Database {
            message: err.to_string(),
        }
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        AppError::Io {
            message: err.to_string(),
        }
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Api {
            message: err.to_string(),
        }
    }
}

// String conversions for compatibility with Tauri command interface
impl From<String> for AppError {
    fn from(message: String) -> Self {
        AppError::Unexpected { message }
    }
}

impl From<&str> for AppError {
    fn from(message: &str) -> Self {
        AppError::Unexpected {
            message: message.to_string(),
        }
    }
}

impl AppError {
    /// Create a database error with a custom message
    pub fn database(message: impl Into<String>) -> Self {
        AppError::Database {
            message: message.into(),
        }
    }

    /// Create a validation error with a custom message
    pub fn validation(message: impl Into<String>) -> Self {
        AppError::Validation {
            message: message.into(),
        }
    }

    /// Create a not found error with a custom message
    pub fn not_found(message: impl Into<String>) -> Self {
        AppError::NotFound {
            message: message.into(),
        }
    }

    /// Create an API error with a custom message
    pub fn api(message: impl Into<String>) -> Self {
        AppError::Api {
            message: message.into(),
        }
    }

    /// Create an encryption error with a custom message
    pub fn encryption(message: impl Into<String>) -> Self {
        AppError::Encryption {
            message: message.into(),
        }
    }

    /// Create an I/O error with a custom message
    pub fn io(message: impl Into<String>) -> Self {
        AppError::Io {
            message: message.into(),
        }
    }

    /// Create an unexpected error with a custom message
    pub fn unexpected(message: impl Into<String>) -> Self {
        AppError::Unexpected {
            message: message.into(),
        }
    }

    /// Check if this error should be logged at error level (vs warning)
    pub fn is_critical(&self) -> bool {
        matches!(
            self,
            AppError::Database { .. } | AppError::Encryption { .. } | AppError::Unexpected { .. }
        )
    }

    /// Get user-friendly error message for display in UI
    pub fn user_message(&self) -> String {
        match self {
            AppError::Database { .. } => {
                "A database error occurred. Please try again or contact support.".to_string()
            }
            AppError::Io { .. } => {
                "A file or network error occurred. Please check your connection and try again."
                    .to_string()
            }
            AppError::Validation { message } => message.clone(),
            AppError::NotFound { message } => message.clone(),
            AppError::Api { .. } => {
                "The AI service is temporarily unavailable. Please try again later.".to_string()
            }
            AppError::Encryption { .. } => {
                "A security error occurred. Please restart the application.".to_string()
            }
            AppError::Unexpected { .. } => {
                "An unexpected error occurred. Please try again or contact support.".to_string()
            }
        }
    }

    /// Get technical error message for logging
    pub fn technical_message(&self) -> String {
        match self {
            AppError::Database { message } => format!("Database error: {}", message),
            AppError::Io { message } => format!("I/O error: {}", message),
            AppError::Validation { message } => format!("Validation error: {}", message),
            AppError::NotFound { message } => format!("Not found: {}", message),
            AppError::Api { message } => format!("API error: {}", message),
            AppError::Encryption { message } => format!("Encryption error: {}", message),
            AppError::Unexpected { message } => format!("Unexpected error: {}", message),
        }
    }
    
    /// Get platform-specific error handling suggestions
    #[cfg(target_os = "windows")]
    pub fn platform_suggestion(&self) -> Option<String> {
        match self {
            AppError::Io { message } if message.contains("Access is denied") => {
                Some("Try running as administrator or check file permissions.".to_string())
            }
            AppError::Io { message } if message.contains("The system cannot find the path") => {
                Some("Ensure the directory exists or create it first.".to_string())
            }
            AppError::Database { message } if message.contains("locked") => {
                Some("Close other applications that might be accessing the database.".to_string())
            }
            _ => None,
        }
    }
    
    #[cfg(not(target_os = "windows"))]
    pub fn platform_suggestion(&self) -> Option<String> {
        match self {
            AppError::Io { message } if message.contains("Permission denied") => {
                Some("Check file permissions or try running with sudo.".to_string())
            }
            AppError::Database { message } if message.contains("locked") => {
                Some("Close other applications that might be accessing the database.".to_string())
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let db_error = AppError::database("Connection failed");
        assert!(matches!(db_error, AppError::Database { .. }));
        assert!(db_error.is_critical());
    }

    #[test]
    fn test_error_conversion() {
        let sqlite_error = rusqlite::Error::InvalidDatabaseName("test".to_string());
        let app_error: AppError = sqlite_error.into();
        assert!(matches!(app_error, AppError::Database { .. }));
    }

    #[test]
    fn test_user_friendly_messages() {
        let validation_error = AppError::validation("Invalid email format");
        assert_eq!(validation_error.user_message(), "Invalid email format");

        let database_error = AppError::database("Connection timeout");
        assert_eq!(
            database_error.user_message(),
            "A database error occurred. Please try again or contact support."
        );
    }
}
