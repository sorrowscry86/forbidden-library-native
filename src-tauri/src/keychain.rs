
//! Secure OS keychain integration for API key storage
//!
//! This module provides cross-platform access to the OS native credential storage:
//! - macOS: Keychain
//! - Windows: Credential Manager
//! - Linux: Secret Service (libsecret)
//!
//! API keys are stored securely and never written to disk in plain text.

use keyring::Entry;
use serde::{Deserialize, Serialize};
use crate::error::AppError;

/// Service identifier for keychain entries
const SERVICE_NAME: &str = "com.voidcat.forbidden-library";

/// Keychain manager for secure API key storage
#[derive(Debug, Clone)]
pub struct KeychainManager {
    service: String,
}

impl Default for KeychainManager {
    fn default() -> Self {
        Self::new()
    }
}

impl KeychainManager {
    /// Create a new keychain manager
    pub fn new() -> Self {
        Self {
            service: SERVICE_NAME.to_string(),
        }
    }

    /// Create a new keychain manager with custom service name (for testing)
    #[cfg(test)]
    pub fn with_service(service: String) -> Self {
        Self { service }
    }

    /// Store an API key in the OS keychain
    ///
    /// # Arguments
    /// * `provider_name` - Unique identifier for the provider (e.g., "openai", "anthropic")
    /// * `api_key` - The API key to store securely
    ///
    /// # Returns
    /// * `Ok(())` if the key was stored successfully
    /// * `Err(AppError)` if storage failed
    pub fn store_api_key(&self, provider_name: &str, api_key: &str) -> Result<(), AppError> {
        if provider_name.is_empty() {
            return Err(AppError::validation("Provider name cannot be empty"));
        }

        if api_key.is_empty() {
            return Err(AppError::validation("API key cannot be empty"));
        }

        // Create keyring entry for this provider
        let entry = Entry::new(&self.service, provider_name)
            .map_err(|e| AppError::keychain(format!("Failed to create keychain entry: {}", e)))?;

        // Store the API key
        entry
            .set_password(api_key)
            .map_err(|e| AppError::keychain(format!("Failed to store API key: {}", e)))?;

        tracing::info!("Stored API key for provider: {}", provider_name);
        Ok(())
    }

    /// Retrieve an API key from the OS keychain
    ///
    /// # Arguments
    /// * `provider_name` - Unique identifier for the provider
    ///
    /// # Returns
    /// * `Ok(String)` containing the API key if found
    /// * `Err(AppError)` if the key doesn't exist or retrieval failed
    pub fn get_api_key(&self, provider_name: &str) -> Result<String, AppError> {
        if provider_name.is_empty() {
            return Err(AppError::validation("Provider name cannot be empty"));
        }

        let entry = Entry::new(&self.service, provider_name)
            .map_err(|e| AppError::keychain(format!("Failed to create keychain entry: {}", e)))?;

        entry
            .get_password()
            .map_err(|e| {
                match e {
                    keyring::Error::NoEntry => {
                        AppError::not_found(format!("No API key found for provider: {}", provider_name))
                    }
                    _ => AppError::keychain(format!("Failed to retrieve API key: {}", e))
                }
            })
    }

    /// Delete an API key from the OS keychain
    ///
    /// # Arguments
    /// * `provider_name` - Unique identifier for the provider
    ///
    /// # Returns
    /// * `Ok(())` if the key was deleted successfully
    /// * `Err(AppError)` if deletion failed or key doesn't exist
    pub fn delete_api_key(&self, provider_name: &str) -> Result<(), AppError> {
        if provider_name.is_empty() {
            return Err(AppError::validation("Provider name cannot be empty"));
        }

        let entry = Entry::new(&self.service, provider_name)
            .map_err(|e| AppError::keychain(format!("Failed to create keychain entry: {}", e)))?;

        entry
            .delete_credential()
            .map_err(|e| {
                match e {
                    keyring::Error::NoEntry => {
                        AppError::not_found(format!("No API key found for provider: {}", provider_name))
                    }
                    _ => AppError::keychain(format!("Failed to delete API key: {}", e))
                }
            })?;

        tracing::info!("Deleted API key for provider: {}", provider_name);
        Ok(())
    }

    /// Check if an API key exists for a provider
    ///
    /// # Arguments
    /// * `provider_name` - Unique identifier for the provider
    ///
    /// # Returns
    /// * `true` if an API key exists
    /// * `false` if no key is stored
    pub fn has_api_key(&self, provider_name: &str) -> bool {
        self.get_api_key(provider_name).is_ok()
    }

    /// Update an existing API key
    ///
    /// # Arguments
    /// * `provider_name` - Unique identifier for the provider
    /// * `new_api_key` - The new API key to store
    ///
    /// # Returns
    /// * `Ok(())` if the key was updated successfully
    /// * `Err(AppError)` if update failed
    pub fn update_api_key(&self, provider_name: &str, new_api_key: &str) -> Result<(), AppError> {
        // The keyring library's set_password overwrites existing entries,
        // so we can just call store_api_key
        self.store_api_key(provider_name, new_api_key)
    }
}

/// API key metadata stored in database (without the actual key)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyMetadata {
    pub provider_name: String,
    pub key_stored: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub last_used: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_manager() -> KeychainManager {
        // Use a unique service name for tests to avoid conflicts
        KeychainManager::with_service(format!(
            "com.voidcat.forbidden-library.test.{}",
            uuid::Uuid::new_v4()
        ))
    }

    fn cleanup_test_key(manager: &KeychainManager, provider: &str) {
        let _ = manager.delete_api_key(provider);
    }

    #[test]
    fn test_store_and_retrieve_api_key() {
        let manager = get_test_manager();
        let provider = "test-provider";
        let api_key = "test-api-key-12345";

        // Clean up any existing test data
        cleanup_test_key(&manager, provider);

        // Store the key
        let result = manager.store_api_key(provider, api_key);
        assert!(result.is_ok(), "Failed to store API key: {:?}", result);

        // Retrieve the key
        let retrieved = manager.get_api_key(provider);
        assert!(retrieved.is_ok(), "Failed to retrieve API key: {:?}", retrieved);
        assert_eq!(retrieved.unwrap(), api_key);

        // Clean up
        cleanup_test_key(&manager, provider);
    }

    #[test]
    fn test_delete_api_key() {
        let manager = get_test_manager();
        let provider = "test-provider-delete";
        let api_key = "test-api-key-delete";

        cleanup_test_key(&manager, provider);

        // Store and then delete
        manager.store_api_key(provider, api_key).unwrap();
        assert!(manager.has_api_key(provider));

        let result = manager.delete_api_key(provider);
        assert!(result.is_ok(), "Failed to delete API key: {:?}", result);

        // Verify it's gone
        assert!(!manager.has_api_key(provider));
    }

    #[test]
    fn test_update_api_key() {
        let manager = get_test_manager();
        let provider = "test-provider-update";
        let old_key = "old-api-key";
        let new_key = "new-api-key";

        cleanup_test_key(&manager, provider);

        // Store initial key
        manager.store_api_key(provider, old_key).unwrap();

        // Update with new key
        let result = manager.update_api_key(provider, new_key);
        assert!(result.is_ok(), "Failed to update API key: {:?}", result);

        // Verify new key is stored
        let retrieved = manager.get_api_key(provider).unwrap();
        assert_eq!(retrieved, new_key);

        cleanup_test_key(&manager, provider);
    }

    #[test]
    fn test_has_api_key() {
        let manager = get_test_manager();
        let provider = "test-provider-exists";

        cleanup_test_key(&manager, provider);

        // Should not exist initially
        assert!(!manager.has_api_key(provider));

        // Store a key
        manager.store_api_key(provider, "test-key").unwrap();

        // Should exist now
        assert!(manager.has_api_key(provider));

        cleanup_test_key(&manager, provider);
    }

    #[test]
    fn test_empty_provider_name() {
        let manager = get_test_manager();

        let result = manager.store_api_key("", "test-key");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_empty_api_key() {
        let manager = get_test_manager();

        let result = manager.store_api_key("test-provider", "");
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AppError::Validation(_)));
    }

    #[test]
    fn test_get_nonexistent_key() {
        let manager = get_test_manager();
        let provider = "nonexistent-provider";

        cleanup_test_key(&manager, provider);

        let result = manager.get_api_key(provider);
        assert!(result.is_err());
    }
}
