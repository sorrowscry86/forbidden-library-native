//! Comprehensive input validation system for Forbidden Library
//!
//! This module provides centralized validation for all user inputs and parameters
//! to ensure data integrity and security across the application.

use crate::errors::{AppError, AppResult};
use regex::Regex;
use std::collections::HashSet;

/// Comprehensive input validator for the Forbidden Library application
///
/// Provides centralized validation for all user inputs to ensure data integrity
/// and security. Validates strings, file paths, URLs, and other inputs according
/// to security best practices.
///
/// # Features
///
/// * Length validation for all string inputs
/// * Format validation (email, URL, UUID, etc.)
/// * Security validation (SQL injection, XSS prevention)
/// * File path security (path traversal prevention)
/// * Whitelisted file extensions
///
/// # Examples
///
/// ```
/// use forbidden_library_native::validation::InputValidator;
///
/// let validator = InputValidator::default();
///
/// // Validate conversation title
/// let title = validator.validate_conversation_title("My Chat").unwrap();
/// assert_eq!(title, "My Chat");
///
/// // Validate URL
/// let url = validator.validate_url("https://example.com").unwrap();
/// ```
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

    /// Validate and sanitize a conversation title
    ///
    /// Ensures the title is not empty, within length limits, and doesn't contain
    /// potentially dangerous characters.
    ///
    /// # Arguments
    ///
    /// * `title` - The conversation title to validate
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The trimmed and validated title
    /// * `Err(AppError::Validation)` - If validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Title is empty or whitespace only
    /// * Title exceeds maximum length (200 characters)
    /// * Title contains dangerous characters (SQL injection, XSS patterns)
    ///
    /// # Examples
    ///
    /// ```
    /// use forbidden_library_native::validation::InputValidator;
    ///
    /// let validator = InputValidator::default();
    ///
    /// // Valid title
    /// assert!(validator.validate_conversation_title("My Chat").is_ok());
    ///
    /// // Invalid - empty
    /// assert!(validator.validate_conversation_title("").is_err());
    ///
    /// // Invalid - XSS attempt
    /// assert!(validator.validate_conversation_title("<script>alert('xss')</script>").is_err());
    /// ```
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
        let valid_regex = Regex::new(r"^[a-zA-Z0-9\s\-_]+$").unwrap();
        if !valid_regex.is_match(trimmed) {
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
        let valid_regex = Regex::new(r"^[a-zA-Z0-9\-_.]+$").unwrap();
        if !valid_regex.is_match(trimmed) {
            return Err(AppError::validation("API key contains invalid characters"));
        }

        Ok(trimmed.to_string())
    }

    /// Validate file path and extension for security
    ///
    /// Ensures the file path has a valid whitelisted extension and doesn't contain
    /// path traversal sequences that could access unauthorized directories.
    ///
    /// # Arguments
    ///
    /// * `path` - The file path to validate
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The validated file path
    /// * `Err(AppError::Validation)` - If validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * Path is empty
    /// * Path exceeds maximum length (1000 characters)
    /// * File extension is not in the whitelist
    /// * Path contains traversal sequences (`..`, `~`)
    ///
    /// # Security
    ///
    /// This method prevents path traversal attacks by rejecting paths containing
    /// `..` or `~` sequences. Only whitelisted file extensions are allowed to
    /// prevent execution of arbitrary files.
    ///
    /// # Examples
    ///
    /// ```
    /// use forbidden_library_native::validation::InputValidator;
    ///
    /// let validator = InputValidator::default();
    ///
    /// // Valid paths
    /// assert!(validator.validate_file_path("document.txt").is_ok());
    /// assert!(validator.validate_file_path("folder/file.md").is_ok());
    ///
    /// // Invalid - path traversal
    /// assert!(validator.validate_file_path("../../../etc/passwd").is_err());
    ///
    /// // Invalid - dangerous extension
    /// assert!(validator.validate_file_path("malware.exe").is_err());
    /// ```
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

    /// Validate URL format and protocol
    ///
    /// Ensures the URL has a valid HTTP or HTTPS scheme and proper format.
    /// Only allows http:// and https:// protocols for security.
    ///
    /// # Arguments
    ///
    /// * `url` - The URL string to validate
    ///
    /// # Returns
    ///
    /// * `Ok(String)` - The validated URL
    /// * `Err(AppError::Validation)` - If validation fails
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// * URL is empty
    /// * URL exceeds maximum length (2000 characters)
    /// * URL doesn't match HTTP/HTTPS format
    /// * URL contains invalid characters
    ///
    /// # Examples
    ///
    /// ```
    /// use forbidden_library_native::validation::InputValidator;
    ///
    /// let validator = InputValidator::default();
    ///
    /// // Valid URLs
    /// assert!(validator.validate_url("https://example.com").is_ok());
    /// assert!(validator.validate_url("http://localhost:3000/api").is_ok());
    ///
    /// // Invalid - wrong protocol
    /// assert!(validator.validate_url("ftp://example.com").is_err());
    ///
    /// // Invalid - not a URL
    /// assert!(validator.validate_url("not-a-url").is_err());
    /// ```
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
        let url_regex = Regex::new(r"^https?://[^\s/$.?#].[^\s]*$").unwrap();
        if !url_regex.is_match(trimmed) {
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

        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
        if !email_regex.is_match(trimmed) {
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

        let uuid_regex = Regex::new(
            r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$",
        )
        .unwrap();
        if !uuid_regex.is_match(trimmed) {
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
    ///
    /// Categorizes dangerous patterns for better maintainability:
    /// - XSS patterns (script tags, event handlers)
    /// - SQL injection patterns (SQL keywords, comment markers)
    /// - Special characters that could be misused
    fn contains_dangerous_chars(&self, input: &str) -> bool {
        let input_lower = input.to_lowercase();

        Self::contains_xss_patterns(&input_lower)
            || Self::contains_sql_injection_patterns(&input_lower)
            || Self::contains_dangerous_special_chars(&input_lower)
    }

    /// Check for XSS (Cross-Site Scripting) attack patterns
    fn contains_xss_patterns(input_lower: &str) -> bool {
        const XSS_PATTERNS: &[&str] = &[
            "<script",
            "</script>",
            "javascript:",
            "vbscript:",
            "onload=",
            "onerror=",
            "onclick=",
            "onmouseover=",
        ];

        XSS_PATTERNS.iter().any(|pattern| input_lower.contains(pattern))
    }

    /// Check for SQL injection attack patterns
    fn contains_sql_injection_patterns(input_lower: &str) -> bool {
        const SQL_PATTERNS: &[&str] = &[
            "drop table",
            "insert into",
            "delete from",
            "update set",
            "union select",
        ];

        SQL_PATTERNS.iter().any(|pattern| input_lower.contains(pattern))
    }

    /// Check for dangerous special characters
    fn contains_dangerous_special_chars(input_lower: &str) -> bool {
        const DANGEROUS_CHARS: &[&str] = &[
            "'",
            "\"",
            ";",
            "--",
            "/*",
            "*/",
        ];

        DANGEROUS_CHARS.iter().any(|pattern| input_lower.contains(pattern))
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
