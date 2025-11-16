//! Comprehensive input validation system for Forbidden Library
//!
//! This module provides centralized validation for all user inputs and parameters
//! to ensure data integrity and security across the application.

use crate::errors::{AppError, AppResult};
use regex::Regex;
use std::collections::HashSet;
use std::sync::OnceLock;

/// Cached regex patterns for performance optimization
/// These are compiled once at first use and reused throughout the application
static PERSONA_NAME_REGEX: OnceLock<Regex> = OnceLock::new();
static API_KEY_REGEX: OnceLock<Regex> = OnceLock::new();
static URL_REGEX: OnceLock<Regex> = OnceLock::new();
static EMAIL_REGEX: OnceLock<Regex> = OnceLock::new();
static UUID_REGEX: OnceLock<Regex> = OnceLock::new();

/// Get or initialize the persona name regex pattern
fn get_persona_name_regex() -> &'static Regex {
    PERSONA_NAME_REGEX.get_or_init(|| Regex::new(r"^[a-zA-Z0-9\s\-_]+$").unwrap())
}

/// Get or initialize the API key regex pattern
fn get_api_key_regex() -> &'static Regex {
    API_KEY_REGEX.get_or_init(|| Regex::new(r"^[a-zA-Z0-9\-_.]+$").unwrap())
}

/// Get or initialize the URL regex pattern
fn get_url_regex() -> &'static Regex {
    URL_REGEX.get_or_init(|| Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap())
}

/// Get or initialize the email regex pattern
fn get_email_regex() -> &'static Regex {
    EMAIL_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap()
    })
}

/// Get or initialize the UUID regex pattern
fn get_uuid_regex() -> &'static Regex {
    UUID_REGEX.get_or_init(|| {
        Regex::new(r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$")
            .unwrap()
    })
}

/// Comprehensive input validator for the Forbidden Library application
/// Validates all user inputs according to VoidCat RDC security standards
pub struct InputValidator {
    /// Allowed file extensions for file operations
    allowed_extensions: HashSet<String>,
    /// Maximum allowed string lengths for various fields
    max_lengths: ValidationLimits,
}

/// Validation limits for different types of input
#[derive(Debug, Clone)]
pub struct ValidationLimits {
    pub conversation_title: usize,
    pub message_content: usize,
    pub persona_name: usize,
    pub persona_description: usize,
    pub system_prompt: usize,
    pub api_key: usize,
    pub file_path: usize,
    pub url: usize,
}

impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            conversation_title: 200,
            message_content: 100_000, // 100KB for large messages
            persona_name: 50,
            persona_description: 500,
            system_prompt: 10_000, // 10KB for complex system prompts
            api_key: 200,
            file_path: 1000,
            url: 2000,
        }
    }
}

impl Default for InputValidator {
    fn default() -> Self {
        let mut allowed_extensions = HashSet::new();

        // Document formats
        allowed_extensions.insert("txt".to_string());
        allowed_extensions.insert("md".to_string());
        allowed_extensions.insert("json".to_string());
        allowed_extensions.insert("yaml".to_string());
        allowed_extensions.insert("yml".to_string());

        // Code files (for development features)
        allowed_extensions.insert("rs".to_string());
        allowed_extensions.insert("js".to_string());
        allowed_extensions.insert("ts".to_string());
        allowed_extensions.insert("py".to_string());
        allowed_extensions.insert("html".to_string());
        allowed_extensions.insert("css".to_string());
        allowed_extensions.insert("svelte".to_string());

        // Image formats (for avatars)
        allowed_extensions.insert("png".to_string());
        allowed_extensions.insert("jpg".to_string());
        allowed_extensions.insert("jpeg".to_string());
        allowed_extensions.insert("gif".to_string());
        allowed_extensions.insert("webp".to_string());

        Self {
            allowed_extensions,
            max_lengths: ValidationLimits::default(),
        }
    }
}

impl InputValidator {
    /// Create a new validator with custom limits
    pub fn new(limits: ValidationLimits) -> Self {
        Self {
            allowed_extensions: Self::default().allowed_extensions,
            max_lengths: limits,
        }
    }

    /// Validate conversation title
    pub fn validate_conversation_title(&self, title: &str) -> AppResult<String> {
        let trimmed = title.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("Conversation title cannot be empty"));
        }

        if trimmed.len() > self.max_lengths.conversation_title {
            return Err(AppError::validation(format!(
                "Conversation title cannot exceed {} characters",
                self.max_lengths.conversation_title
            )));
        }

        // Check for potentially harmful characters
        if self.contains_dangerous_chars(trimmed) {
            return Err(AppError::validation(
                "Conversation title contains invalid characters",
            ));
        }

        Ok(trimmed.to_string())
    }

    /// Validate message content
    pub fn validate_message_content(&self, content: &str) -> AppResult<String> {
        if content.trim().is_empty() {
            return Err(AppError::validation("Message content cannot be empty"));
        }

        if content.len() > self.max_lengths.message_content {
            return Err(AppError::validation(format!(
                "Message content cannot exceed {} characters",
                self.max_lengths.message_content
            )));
        }

        // Basic content sanitization - remove null bytes and other control characters
        let sanitized = content
            .chars()
            .filter(|c| !c.is_control() || *c == '\n' || *c == '\r' || *c == '\t')
            .collect();

        Ok(sanitized)
    }

    /// Validate persona name
    pub fn validate_persona_name(&self, name: &str) -> AppResult<String> {
        let trimmed = name.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("Persona name cannot be empty"));
        }

        if trimmed.len() > self.max_lengths.persona_name {
            return Err(AppError::validation(format!(
                "Persona name cannot exceed {} characters",
                self.max_lengths.persona_name
            )));
        }

        // Persona names should be alphanumeric with spaces, hyphens, and underscores
        if !get_persona_name_regex().is_match(trimmed) {
            return Err(AppError::validation(
                "Persona name can only contain letters, numbers, spaces, hyphens, and underscores",
            ));
        }

        Ok(trimmed.to_string())
    }

    /// Validate persona description
    pub fn validate_persona_description(&self, description: &str) -> AppResult<String> {
        let trimmed = description.trim();

        if trimmed.len() > self.max_lengths.persona_description {
            return Err(AppError::validation(format!(
                "Persona description cannot exceed {} characters",
                self.max_lengths.persona_description
            )));
        }

        if self.contains_dangerous_chars(trimmed) {
            return Err(AppError::validation(
                "Persona description contains invalid characters",
            ));
        }

        Ok(trimmed.to_string())
    }

    /// Validate system prompt
    pub fn validate_system_prompt(&self, prompt: &str) -> AppResult<String> {
        if prompt.trim().is_empty() {
            return Err(AppError::validation("System prompt cannot be empty"));
        }

        if prompt.len() > self.max_lengths.system_prompt {
            return Err(AppError::validation(format!(
                "System prompt cannot exceed {} characters",
                self.max_lengths.system_prompt
            )));
        }

        // System prompts can contain most characters but not null bytes
        let sanitized = prompt.chars().filter(|c| *c != '\0').collect();

        Ok(sanitized)
    }

    /// Validate API key format
    pub fn validate_api_key(&self, api_key: &str) -> AppResult<String> {
        let trimmed = api_key.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("API key cannot be empty"));
        }

        if trimmed.len() > self.max_lengths.api_key {
            return Err(AppError::validation(format!(
                "API key cannot exceed {} characters",
                self.max_lengths.api_key
            )));
        }

        // API keys should be alphanumeric with some special characters
        if !get_api_key_regex().is_match(trimmed) {
            return Err(AppError::validation("API key contains invalid characters"));
        }

        Ok(trimmed.to_string())
    }

    /// Validate file path and extension
    pub fn validate_file_path(&self, path: &str) -> AppResult<String> {
        let trimmed = path.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("File path cannot be empty"));
        }

        if trimmed.len() > self.max_lengths.file_path {
            return Err(AppError::validation(format!(
                "File path cannot exceed {} characters",
                self.max_lengths.file_path
            )));
        }

        // Extract file extension
        if let Some(extension) = std::path::Path::new(trimmed)
            .extension()
            .and_then(|ext| ext.to_str())
        {
            if !self.allowed_extensions.contains(&extension.to_lowercase()) {
                return Err(AppError::validation(format!(
                    "File extension '{}' is not allowed",
                    extension
                )));
            }
        } else {
            return Err(AppError::validation("File must have a valid extension"));
        }

        // Check for path traversal attempts
        if trimmed.contains("..") || trimmed.contains("~") {
            return Err(AppError::validation("Path traversal is not allowed"));
        }

        Ok(trimmed.to_string())
    }

    /// Validate URL format
    pub fn validate_url(&self, url: &str) -> AppResult<String> {
        let trimmed = url.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("URL cannot be empty"));
        }

        if trimmed.len() > self.max_lengths.url {
            return Err(AppError::validation(format!(
                "URL cannot exceed {} characters",
                self.max_lengths.url
            )));
        }

        // Basic URL validation
        if !get_url_regex().is_match(trimmed) {
            return Err(AppError::validation("Invalid URL format"));
        }

        Ok(trimmed.to_string())
    }

    /// Validate email format (for contact information)
    pub fn validate_email(&self, email: &str) -> AppResult<String> {
        let trimmed = email.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("Email cannot be empty"));
        }

        if !get_email_regex().is_match(trimmed) {
            return Err(AppError::validation("Invalid email format"));
        }

        Ok(trimmed.to_lowercase())
    }

    /// Validate UUID format
    pub fn validate_uuid(&self, uuid: &str) -> AppResult<String> {
        let trimmed = uuid.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("UUID cannot be empty"));
        }

        if !get_uuid_regex().is_match(trimmed) {
            return Err(AppError::validation("Invalid UUID format"));
        }

        Ok(trimmed.to_lowercase())
    }

    /// Validate integer within a range
    pub fn validate_integer_range(
        &self,
        value: i64,
        min: i64,
        max: i64,
        field_name: &str,
    ) -> AppResult<i64> {
        if value < min || value > max {
            return Err(AppError::validation(format!(
                "{} must be between {} and {}",
                field_name, min, max
            )));
        }

        Ok(value)
    }

    /// Validate message role
    pub fn validate_message_role(&self, role: &str) -> AppResult<String> {
        let valid_roles = ["user", "assistant", "system"];
        let trimmed = role.trim().to_lowercase();

        if !valid_roles.contains(&trimmed.as_str()) {
            return Err(AppError::validation(format!(
                "Invalid message role. Must be one of: {}",
                valid_roles.join(", ")
            )));
        }

        Ok(trimmed)
    }

    /// Check for dangerous characters that could be used for injection attacks
    fn contains_dangerous_chars(&self, input: &str) -> bool {
        // Look for SQL injection patterns, script tags, etc.
        let dangerous_patterns = [
            "<script",
            "</script>",
            "javascript:",
            "vbscript:",
            "onload=",
            "onerror=",
            "onclick=",
            "onmouseover=",
            "DROP TABLE",
            "INSERT INTO",
            "DELETE FROM",
            "UPDATE SET",
            "UNION SELECT",
            "'",
            "\"",
            ";",
            "--",
            "/*",
            "*/",
        ];

        let input_lower = input.to_lowercase();
        dangerous_patterns
            .iter()
            .any(|pattern| input_lower.contains(pattern))
    }

    /// Validate and sanitize JSON string
    pub fn validate_json(&self, json_str: &str) -> AppResult<String> {
        let trimmed = json_str.trim();

        if trimmed.is_empty() {
            return Err(AppError::validation("JSON cannot be empty"));
        }

        // Parse to validate JSON format
        match serde_json::from_str::<serde_json::Value>(trimmed) {
            Ok(_) => Ok(trimmed.to_string()),
            Err(e) => Err(AppError::validation(format!("Invalid JSON format: {}", e))),
        }
    }
}

/// Global validator instance for use throughout the application
pub static VALIDATOR: std::sync::OnceLock<InputValidator> = std::sync::OnceLock::new();

/// Get the global validator instance
pub fn get_validator() -> &'static InputValidator {
    VALIDATOR.get_or_init(InputValidator::default)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conversation_title_validation() {
        let validator = InputValidator::default();

        // Valid titles
        assert!(validator
            .validate_conversation_title("My Conversation")
            .is_ok());
        assert!(validator.validate_conversation_title("  Trimmed  ").is_ok());

        // Invalid titles
        assert!(validator.validate_conversation_title("").is_err());
        assert!(validator.validate_conversation_title("   ").is_err());
        assert!(validator
            .validate_conversation_title(&"x".repeat(300))
            .is_err());
        assert!(validator
            .validate_conversation_title("Title with <script>")
            .is_err());
    }

    #[test]
    fn test_persona_name_validation() {
        let validator = InputValidator::default();

        // Valid names
        assert!(validator.validate_persona_name("Assistant-1").is_ok());
        assert!(validator.validate_persona_name("My_Bot").is_ok());
        assert!(validator.validate_persona_name("Helper 2024").is_ok());

        // Invalid names
        assert!(validator.validate_persona_name("").is_err());
        assert!(validator.validate_persona_name("Name@Special").is_err());
        assert!(validator.validate_persona_name("Name<tag>").is_err());
    }

    #[test]
    fn test_api_key_validation() {
        let validator = InputValidator::default();

        // Valid API keys
        assert!(validator.validate_api_key("sk-1234567890abcdef").is_ok());
        assert!(validator.validate_api_key("api_key_123-456_789").is_ok());

        // Invalid API keys
        assert!(validator.validate_api_key("").is_err());
        assert!(validator.validate_api_key("key with spaces").is_err());
        assert!(validator.validate_api_key("key@special").is_err());
    }

    #[test]
    fn test_file_path_validation() {
        let validator = InputValidator::default();

        // Valid paths
        assert!(validator.validate_file_path("document.txt").is_ok());
        assert!(validator.validate_file_path("folder/file.md").is_ok());
        assert!(validator.validate_file_path("image.png").is_ok());

        // Invalid paths
        assert!(validator.validate_file_path("").is_err());
        assert!(validator.validate_file_path("file.exe").is_err());
        assert!(validator.validate_file_path("../../../etc/passwd").is_err());
        assert!(validator.validate_file_path("~/secrets.txt").is_err());
    }

    #[test]
    fn test_url_validation() {
        let validator = InputValidator::default();

        // Valid URLs
        assert!(validator.validate_url("https://example.com").is_ok());
        assert!(validator.validate_url("http://localhost:3000/api").is_ok());

        // Invalid URLs
        assert!(validator.validate_url("").is_err());
        assert!(validator.validate_url("not-a-url").is_err());
        assert!(validator.validate_url("ftp://example.com").is_err());
    }

    #[test]
    fn test_dangerous_chars_detection() {
        let validator = InputValidator::default();

        assert!(validator.contains_dangerous_chars("<script>alert('xss')</script>"));
        assert!(validator.contains_dangerous_chars("'; DROP TABLE users; --"));
        assert!(validator.contains_dangerous_chars("onclick=malicious()"));
        assert!(!validator.contains_dangerous_chars("Safe content here"));
    }
}
