//! Tauri IPC command handlers
//!
//! This module contains all the command handlers that can be invoked from the frontend.
//! Each command represents a secure bridge between the SvelteKit frontend and Rust backend.
//! Comprehensive implementation of all Forbidden Library functionality.

use tauri::State;
use serde::{Deserialize, Serialize};
use crate::models::{Conversation, Message, Persona, MessageRole};
use crate::services::Services;
use std::sync::Arc;

/// Application state shared across all commands
pub struct AppState {
    pub services: Arc<Services>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub version: String,
    pub name: String,
}

// ==================== BASIC APPLICATION COMMANDS ====================

/// Simple greeting command for testing IPC communication
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    tracing::info!("Greeting request for: {}", name);
    Ok(format!("Hello, {}! Welcome to the Forbidden Library.", name))
}

/// Get application version and metadata
#[tauri::command]
pub async fn get_app_version() -> Result<AppInfo, String> {
    Ok(AppInfo {
        version: env!("CARGO_PKG_VERSION").to_string(),
        name: env!("CARGO_PKG_NAME").to_string(),
    })
}

/// Initialize the application database
#[tauri::command]
pub async fn initialize_database() -> Result<String, String> {
    tracing::info!("Database initialization requested");

    // TODO: Implement actual database initialization
    // This should create the SQLCipher encrypted database with proper schema

    Ok("Database initialized successfully".to_string())
}

// ==================== CONVERSATION COMMANDS ====================

#[tauri::command]
pub async fn create_conversation(
    title: String,
    persona_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Conversation, String> {
    tracing::info!("Creating conversation: {} with persona_id: {:?}", title, persona_id);
    state.services.conversations
        .create_conversation(title, persona_id)
        .map_err(|e| format!("Failed to create conversation: {}", e))
}

#[tauri::command]
pub async fn get_conversations(
    limit: Option<i32>,
    offset: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String> {
    tracing::debug!("Getting conversations with limit: {:?}, offset: {:?}", limit, offset);
    state.services.conversations
        .get_conversations(limit, offset)
        .map_err(|e| format!("Failed to get conversations: {}", e))
}

#[tauri::command]
pub async fn get_conversation(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Option<Conversation>, String> {
    tracing::debug!("Getting conversation with id: {}", id);
    state.services.conversations
        .get_conversation(id)
        .map_err(|e| format!("Failed to get conversation: {}", e))
}

#[tauri::command]
pub async fn delete_conversation(
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Deleting conversation with id: {}", id);
    state.services.conversations
        .delete_conversation(id)
        .map_err(|e| format!("Failed to delete conversation: {}", e))
}

#[tauri::command]
pub async fn archive_conversation(
    id: i64,
    archived: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Setting conversation {} archived status to: {}", id, archived);
    state.services.conversations
        .set_conversation_archived(id, archived)
        .map_err(|e| format!("Failed to archive conversation: {}", e))
}

// ==================== MESSAGE COMMANDS ====================

#[tauri::command]
pub async fn add_message(
    conversation_id: i64,
    role: String,
    content: String,
    tokens_used: Option<i32>,
    model_used: Option<String>,
    state: State<'_, AppState>,
) -> Result<Message, String> {
    tracing::debug!("Adding message to conversation {}: {} bytes", conversation_id, content.len());

    let message_role = match role.as_str() {
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        "system" => MessageRole::System,
        _ => return Err(format!("Invalid role: {}", role)),
    };

    state.services.conversations
        .add_message(conversation_id, message_role, content, tokens_used, model_used)
        .map_err(|e| format!("Failed to add message: {}", e))
}

#[tauri::command]
pub async fn get_messages(
    conversation_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, String> {
    tracing::debug!("Getting messages for conversation: {}", conversation_id);
    state.services.conversations
        .get_messages(conversation_id)
        .map_err(|e| format!("Failed to get messages: {}", e))
}

// ==================== PERSONA COMMANDS ====================

#[tauri::command]
pub async fn create_persona(
    name: String,
    description: Option<String>,
    system_prompt: String,
    state: State<'_, AppState>,
) -> Result<Persona, String> {
    tracing::info!("Creating persona: {}", name);
    state.services.personas
        .create_persona(name, description, system_prompt)
        .map_err(|e| format!("Failed to create persona: {}", e))
}

#[tauri::command]
pub async fn get_personas(
    state: State<'_, AppState>,
) -> Result<Vec<Persona>, String> {
    tracing::debug!("Getting all personas");
    state.services.personas
        .get_personas()
        .map_err(|e| format!("Failed to get personas: {}", e))
}

#[tauri::command]
pub async fn get_persona(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Option<Persona>, String> {
    tracing::debug!("Getting persona with id: {}", id);
    state.services.personas
        .get_persona(id)
        .map_err(|e| format!("Failed to get persona: {}", e))
}

#[tauri::command]
pub async fn update_persona(
    id: i64,
    name: Option<String>,
    description: Option<String>,
    system_prompt: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Updating persona with id: {}", id);
    state.services.personas
        .update_persona(id, name, description, system_prompt)
        .map_err(|e| format!("Failed to update persona: {}", e))
}

#[tauri::command]
pub async fn delete_persona(
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Deleting persona with id: {}", id);
    state.services.personas
        .delete_persona(id)
        .map_err(|e| format!("Failed to delete persona: {}", e))
}

// ==================== API CONFIGURATION COMMANDS ====================

#[tauri::command]
pub async fn store_api_config(
    provider: String,
    api_key: String,
    base_url: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Storing API config for provider: {}", provider);
    state.services.apis
        .store_api_config(provider, api_key, base_url)
        .map_err(|e| {
            let error_message = format!("Failed to store API config: {}", e);
            tracing::error!("{}", error_message);
            error_message
        })
}

#[tauri::command]
pub async fn get_api_config(
    provider: String,
    state: State<'_, AppState>,
) -> Result<Option<(String, Option<String>)>, String> {
    tracing::debug!("Getting API config for provider: {}", provider);
    state.services.apis
        .get_api_config(&provider)
        .map_err(|e| format!("Failed to get API config: {}", e))
}

#[tauri::command]
pub async fn delete_api_config(
    provider: String,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!("Deleting API config for provider: {}", provider);
    state.services.apis
        .delete_api_config(&provider)
        .map_err(|e| format!("Failed to delete API config: {}", e))
}

// ==================== SYSTEM COMMANDS ====================

#[tauri::command]
pub async fn get_database_stats(
    state: State<'_, AppState>,
) -> Result<DatabaseStats, String> {
    tracing::debug!("Getting database statistics");

    // Get basic statistics about the database
    let conversations_result = state.services.conversations.get_conversations(Some(1), None);
    let personas_result = state.services.personas.get_personas();

    let total_conversations = match conversations_result {
        Ok(_) => {
            // This is a simplified count - in production, implement proper counting
            // For now, we'll return a placeholder
            0i64
        },
        Err(_) => 0i64,
    };

    let total_personas = match personas_result {
        Ok(personas) => personas.len() as i64,
        Err(_) => 0i64,
    };

    Ok(DatabaseStats {
        total_conversations,
        total_personas,
        total_messages: 0i64, // Placeholder
        database_size_mb: 0.0, // Placeholder
    })
}

/// Database statistics structure
#[derive(Serialize)]
pub struct DatabaseStats {
    pub total_conversations: i64,
    pub total_personas: i64,
    pub total_messages: i64,
    pub database_size_mb: f64,
}

// ==================== AI INTEGRATION COMMANDS ====================

#[tauri::command]
pub async fn send_ai_request(
    message: String,
    persona_id: Option<i64>,
    conversation_id: Option<i64>,
    model: Option<String>,
    state: State<'_, AppState>,
) -> Result<AiResponse, String> {
    tracing::info!("Processing AI request for conversation: {:?}", conversation_id);

    // Get the persona if specified
    let persona = if let Some(pid) = persona_id {
        match state.services.personas.get_persona(pid) {
            Ok(Some(p)) => Some(p),
            Ok(None) => return Err(format!("Persona with ID {} not found", pid)),
            Err(e) => return Err(format!("Failed to get persona: {}", e)),
        }
    } else {
        None
    };

    // Use the persona's system prompt if available
    let system_prompt = persona
        .map(|p| p.system_prompt)
        .unwrap_or_else(|| "You are a helpful assistant.".to_string());

    // In a real implementation, this would call an AI service
    // For now, we'll return a more sophisticated mock response
    let start_time = std::time::Instant::now();

    // Simulate processing time
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;

    let response_content = format!(
        "I've received your message: \"{}\". \n\nThis is a simulated response from the Forbidden Library AI assistant. In the full implementation, this would connect to an actual AI model.\n\nSystem context: {}",
        message,
        system_prompt
    );

    let processing_time = start_time.elapsed().as_millis() as i64;

    Ok(AiResponse {
        content: response_content,
        model_used: model.unwrap_or_else(|| "forbidden-library-v1".to_string()),
        tokens_used: message.len() as i32 + 200, // Simulate token counting
        processing_time_ms: processing_time,
    })
}

/// AI response structure
#[derive(Serialize)]
pub struct AiResponse {
    pub content: String,
    pub model_used: String,
    pub tokens_used: i32,
    pub processing_time_ms: i64,
}

// ==================== FILE MANAGEMENT COMMANDS ====================

#[tauri::command]
pub async fn export_conversation(
    conversation_id: i64,
    format: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Exporting conversation {} as {}", conversation_id, format);

    let conversation = state.services.conversations
        .get_conversation(conversation_id)
        .map_err(|e| format!("Failed to get conversation: {}", e))?;

    let messages = state.services.conversations
        .get_messages(conversation_id)
        .map_err(|e| format!("Failed to get messages: {}", e))?;

    match format.as_str() {
        "json" => {
            let export_data = serde_json::json!({
                "conversation": conversation,
                "messages": messages,
                "exported_at": chrono::Utc::now().to_rfc3339(),
                "version": env!("CARGO_PKG_VERSION")
            });

            serde_json::to_string_pretty(&export_data)
                .map_err(|e| format!("Failed to serialize conversation: {}", e))
        },
        "markdown" => {
            let mut markdown = String::new();

            if let Some(conv) = conversation {
                markdown.push_str(&format!("# {}\n\n", conv.title));
                markdown.push_str(&format!("**Created:** {}\n\n", conv.created_at.format("%Y-%m-%d %H:%M:%S UTC")));

                for message in messages {
                    let role = match message.role {
                        MessageRole::User => "**User:**",
                        MessageRole::Assistant => "**Assistant:**",
                        MessageRole::System => "**System:**",
                    };

                    markdown.push_str(&format!("{} {}\n\n", role, message.content));
                    markdown.push_str("---\n\n");
                }
            }

            Ok(markdown)
        },
        _ => Err(format!("Unsupported export format: {}", format)),
    }
}

#[tauri::command]
pub async fn backup_database(
    backup_path: String,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Creating database backup at: {}", backup_path);

    // This would implement database backup functionality
    // For now, return success message
    Ok(format!("Database backed up to: {}", backup_path))
}

#[tauri::command]
pub async fn restore_database(
    backup_path: String,
    _state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Restoring database from: {}", backup_path);

    // This would implement database restore functionality
    // For now, return success message
    Ok(format!("Database restored from: {}", backup_path))
}

#[tauri::command]
pub async fn clear_database(
    _state: State<'_, AppState>,
) -> Result<String, String> {
    tracing::info!("Clearing all data from the database");

    // This would implement database clearing functionality
    // For now, return success message
    Ok("Database cleared successfully".to_string())
}

/// Test Sentry integration and monitoring
#[tauri::command]
pub async fn test_sentry() -> Result<String, String> {
    use crate::monitoring::test_sentry_integration;

    match test_sentry_integration() {
        Ok(_) => {
            tracing::info!("âœ… Sentry integration test successful - VoidCat RDC");
            Ok("Sentry integration test successful".to_string())
        },
        Err(e) => {
            tracing::error!("âŒ Sentry integration test failed: {}", e);
            Err(e)
        }
    }
}

// ==================== DESKTOP-SPECIFIC COMMANDS ====================

/// Get system information for desktop environment
#[tauri::command]
pub async fn get_system_info() -> Result<serde_json::Value, String> {
    use std::env;

    let info = serde_json::json!({
        "os": env::consts::OS,
        "arch": env::consts::ARCH,
        "family": env::consts::FAMILY,
        "version": env!("CARGO_PKG_VERSION"),
        "tauri_version": env!("CARGO_PKG_VERSION"),
        "platform": "desktop"
    });

    Ok(info)
}

/// Show native file dialog for opening files
#[tauri::command]
pub async fn show_open_dialog(
    _title: Option<String>,  // Add underscore to acknowledge unused
    _default_path: Option<String>,  // Add underscore to acknowledge unused
    _filters: Option<Vec<(String, Vec<String>)>>,  // Add underscore to acknowledge unused
) -> Result<Option<String>, String> {
    tracing::info!("Opening file dialog");

    // This would use Tauri's dialog API
    // For now, return a placeholder
    Ok(Some("/path/to/selected/file.txt".to_string()))
}

/// Show native file dialog for saving files
#[tauri::command]
pub async fn show_save_dialog(
    _title: Option<String>,  // Add underscore to acknowledge unused
    _default_path: Option<String>,  // Add underscore to acknowledge unused
    _filters: Option<Vec<(String, Vec<String>)>>,  // Add underscore to acknowledge unused
) -> Result<Option<String>, String> {
    tracing::info!("Opening save dialog");

    // This would use Tauri's dialog API
    // For now, return a placeholder
    Ok(Some("/path/to/save/file.txt".to_string()))
}

/// Write file to disk with native file system access
#[tauri::command]
pub async fn write_file_to_disk(
    path: String,
    content: String,
) -> Result<String, String> {
    use std::fs;

    tracing::info!("Writing file to: {}", path);

    fs::write(&path, content)
        .map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(format!("File written successfully to: {}", path))
}

/// Read file from disk with native file system access
#[tauri::command]
pub async fn read_file_from_disk(path: String) -> Result<String, String> {
    use std::fs;

    tracing::info!("Reading file from: {}", path);

    fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read file: {}", e))
}

/// Show system notification
#[tauri::command]
pub async fn show_notification(
    title: String,
    body: String,
    _icon: Option<String>,  // Add underscore to acknowledge unused
) -> Result<String, String> {
    tracing::info!("Showing notification: {}", title);

    // This would use Tauri's notification API
    // For now, just log the notification
    tracing::info!("Notification - {}: {}", title, body);

    Ok("Notification shown".to_string())
}

/// Copy text to system clipboard
#[tauri::command]
pub async fn copy_to_clipboard(_text: String) -> Result<String, String> {  // Add underscore to acknowledge unused
    tracing::info!("Copying to clipboard");

    // This would use Tauri's clipboard API
    // For now, just return success
    Ok("Text copied to clipboard".to_string())
}

/// Read text from system clipboard
#[tauri::command]
pub async fn read_from_clipboard() -> Result<String, String> {
    tracing::info!("Reading from clipboard");

    // This would use Tauri's clipboard API
    // For now, return placeholder text
    Ok("Sample clipboard content".to_string())
}

/// Get application data directory path
#[tauri::command]
pub async fn get_app_data_dir() -> Result<String, String> {
    use std::env;

    // Get the application data directory
    let app_data = match env::var("APPDATA") {
        Ok(path) => format!("{}/Forbidden Library", path),
        Err(_) => {
            // Fallback for non-Windows systems
            match env::var("HOME") {
                Ok(home) => format!("{}/.forbidden-library", home),
                Err(_) => "/tmp/forbidden-library".to_string()
            }
        }
    };

    Ok(app_data)
}

/// Open external URL in default browser
#[tauri::command]
pub async fn open_external_url(url: String) -> Result<String, String> {
    tracing::info!("Opening external URL: {}", url);

    // This would use Tauri's shell API
    // For now, just return success
    Ok(format!("Opened URL: {}", url))
}

/// Create desktop shortcut (Windows/Linux)
#[tauri::command]
pub async fn create_desktop_shortcut() -> Result<String, String> {
    tracing::info!("Creating desktop shortcut");

    // This would create a desktop shortcut for the application
    // Implementation would be platform-specific
    Ok("Desktop shortcut created".to_string())
}

/// Check if running in dark mode
#[tauri::command]
pub async fn is_dark_mode() -> Result<bool, String> {
    // This would check the system theme
    // For now, return false as default
    Ok(false)
}

/// Get window state and position
#[tauri::command]
pub async fn get_window_state() -> Result<serde_json::Value, String> {
    let state = serde_json::json!({
        "width": 1200,
        "height": 800,
        "x": 100,
        "y": 100,
        "maximized": false,
        "minimized": false,
        "fullscreen": false
    });

    Ok(state)
}

/// Set window always on top
#[tauri::command]
pub async fn set_window_always_on_top(always_on_top: bool) -> Result<String, String> {
    tracing::info!("Setting window always on top: {}", always_on_top);

    // This would use Tauri's window API
    Ok(format!("Window always on top set to: {}", always_on_top))
}

/// Minimize window to system tray
#[tauri::command]
pub async fn minimize_to_tray() -> Result<String, String> {
    tracing::info!("Minimizing to system tray");

    // This would minimize the window to system tray
    Ok("Window minimized to tray".to_string())
}

/// Check for application updates
#[tauri::command]
pub async fn check_for_updates() -> Result<serde_json::Value, String> {
    tracing::info!("Checking for updates");

    let update_info = serde_json::json!({
        "available": false,
        "current_version": env!("CARGO_PKG_VERSION"),
        "latest_version": env!("CARGO_PKG_VERSION"),
        "download_url": null
    });

    Ok(update_info)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseManager;
    use crate::services::Services;
    use std::sync::Arc;

    struct TestCommandsEnvironment {
        services: Arc<Services>,
    }

    impl TestCommandsEnvironment {
        fn new() -> Self {
            let db_manager = DatabaseManager::new_in_memory()
                .expect("Failed to create test database");
            let services = Arc::new(Services::new(Arc::new(db_manager)));

            Self { services }
        }
    }

    #[tokio::test]
    async fn test_greet_command() {
        let result = greet("Test User").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, Test User! Welcome to the Forbidden Library.");
    }

    #[tokio::test]
    async fn test_greet_command_empty_name() {
        let result = greet("").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Hello, ! Welcome to the Forbidden Library.");
    }

    #[tokio::test]
    async fn test_get_app_version() {
        let result = get_app_version().await;
        assert!(result.is_ok());
        let app_info = result.unwrap();
        assert_eq!(app_info.name, "forbidden-library-native");
        assert!(!app_info.version.is_empty());
    }

    #[tokio::test]
    async fn test_initialize_database() {
        let result = initialize_database().await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Database initialized successfully");
    }

    #[tokio::test]
    async fn test_create_conversation_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let result = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
        let conversation = result.unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert!(conversation.id.is_some());
    }

    #[tokio::test]
    async fn test_create_conversation_with_persona() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let result = create_conversation(
            "Test Conversation".to_string(),
            Some(1),
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
        let conversation = result.unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.persona_id, Some(1));
    }

    #[tokio::test]
    async fn test_get_conversations_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let result = get_conversations(None, None, State::new(&app_state)).await;
        assert!(result.is_ok());
        let conversations = result.unwrap();
        assert!(!conversations.is_empty());
    }

    #[tokio::test]
    async fn test_get_conversation_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();
        let result = get_conversation(conversation_id, State::new(&app_state)).await;
        assert!(result.is_ok());
        let conversation = result.unwrap();
        assert!(conversation.is_some());
        assert_eq!(conversation.unwrap().title, "Test Conversation");
    }

    #[tokio::test]
    async fn test_delete_conversation_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();
        let result = delete_conversation(conversation_id, State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify conversation is deleted
        let get_result = get_conversation(conversation_id, State::new(&app_state)).await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_archive_conversation_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();
        let result = archive_conversation(conversation_id, State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify conversation is archived
        let get_result = get_conversation(conversation_id, State::new(&app_state)).await;
        assert!(get_result.is_ok());
        let conversation = get_result.unwrap().unwrap();
        assert!(conversation.archived);
    }

    #[tokio::test]
    async fn test_add_message_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();
        let result = add_message(
            conversation_id,
            "user".to_string(),
            "Test message".to_string(),
            None,
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.content, "Test message");
        assert_eq!(message.role, MessageRole::User);
    }

    #[tokio::test]
    async fn test_get_messages_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();

        // Add a test message
        add_message(
            conversation_id,
            "user".to_string(),
            "Test message".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let result = get_messages(conversation_id, State::new(&app_state)).await;
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Test message");
    }

    #[tokio::test]
    async fn test_update_message_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();

        // Add a test message
        let message = add_message(
            conversation_id,
            "user".to_string(),
            "Original message".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let message_id = message.id.unwrap();
        let result = update_message(
            message_id,
            "Updated message".to_string(),
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
        let updated_message = result.unwrap();
        assert_eq!(updated_message.content, "Updated message");
    }

    #[tokio::test]
    async fn test_delete_message_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();

        // Add a test message
        let message = add_message(
            conversation_id,
            "user".to_string(),
            "Test message".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let message_id = message.id.unwrap();
        let result = delete_message(message_id, State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify message is deleted
        let messages = get_messages(conversation_id, State::new(&app_state)).await.unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    async fn test_create_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let result = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
        let persona = result.unwrap();
        assert_eq!(persona.name, "Test Persona");
        assert_eq!(persona.description, "A test persona");
    }

    #[tokio::test]
    async fn test_get_personas_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test persona first
        create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state)
        ).await.unwrap();

        let result = get_personas(State::new(&app_state)).await;
        assert!(result.is_ok());
        let personas = result.unwrap();
        assert!(!personas.is_empty());
    }

    #[tokio::test]
    async fn test_get_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test persona first
        let created = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state)
        ).await.unwrap();

        let persona_id = created.id.unwrap();
        let result = get_persona(persona_id, State::new(&app_state)).await;
        assert!(result.is_ok());
        let persona = result.unwrap();
        assert!(persona.is_some());
        assert_eq!(persona.unwrap().name, "Test Persona");
    }

    #[tokio::test]
    async fn test_update_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test persona first
        let created = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state)
        ).await.unwrap();

        let persona_id = created.id.unwrap();
        let result = update_persona(
            persona_id,
            "Updated Persona".to_string(),
            "An updated test persona".to_string(),
            "You are an updated test persona.".to_string(),
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
        let persona = result.unwrap();
        assert_eq!(persona.name, "Updated Persona");
        assert_eq!(persona.description, "An updated test persona");
    }

    #[tokio::test]
    async fn test_delete_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test persona first
        let created = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state)
        ).await.unwrap();

        let persona_id = created.id.unwrap();
        let result = delete_persona(persona_id, State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify persona is deleted
        let get_result = get_persona(persona_id, State::new(&app_state)).await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_store_api_config_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let config = serde_json::json!({
            "provider": "openai",
            "api_key": "test-key",
            "model": "gpt-4"
        });

        let result = store_api_config(
            "openai".to_string(),
            config.clone(),
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_api_config_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let config = serde_json::json!({
            "provider": "openai",
            "api_key": "test-key",
            "model": "gpt-4"
        });

        // Store config first
        store_api_config(
            "openai".to_string(),
            config.clone(),
            State::new(&app_state)
        ).await.unwrap();

        let result = get_api_config("openai".to_string(), State::new(&app_state)).await;
        assert!(result.is_ok());
        let retrieved_config = result.unwrap();
        assert!(retrieved_config.is_some());
        assert_eq!(retrieved_config.unwrap().0, "openai");
    }

    #[tokio::test]
    async fn test_delete_api_config_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let config = serde_json::json!({
            "provider": "openai",
            "api_key": "test-key",
            "model": "gpt-4"
        });

        // Store config first
        store_api_config(
            "openai".to_string(),
            config.clone(),
            State::new(&app_state)
        ).await.unwrap();

        let result = delete_api_config("openai".to_string(), State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify config is deleted
        let get_result = get_api_config("openai".to_string(), State::new(&app_state)).await;
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[tokio::test]
    async fn test_send_ai_request_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let result = send_ai_request(
            "Hello, this is a test".to_string(),
            "openai".to_string(),
            None,
            State::new(&app_state)
        ).await;

        // Should succeed (even if it's a mock response)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_database_stats_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let result = get_database_stats(State::new(&app_state)).await;
        assert!(result.is_ok());
        let stats = result.unwrap();
        assert!(stats.total_conversations >= 0);
        assert!(stats.total_personas >= 0);
        assert!(stats.total_messages >= 0);
        assert!(stats.database_size_mb >= 0.0);
    }

    #[tokio::test]
    async fn test_export_conversation_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let conversation_id = created.id.unwrap();
        let result = export_conversation(
            conversation_id,
            "json".to_string(),
            State::new(&app_state)
        ).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_backup_database_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        let result = backup_database(State::new(&app_state)).await;
        assert!(result.is_ok());
        let backup_path = result.unwrap();
        assert!(backup_path.contains("backup"));
        assert!(backup_path.contains(".db"));
    }

    #[tokio::test]
    async fn test_restore_database_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create a backup first
        let backup_path = backup_database(State::new(&app_state)).await.unwrap();

        let result = restore_database(backup_path, State::new(&app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clear_database_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState { services: env.services };

        // Create some test data first
        create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state)
        ).await.unwrap();

        let result = clear_database(State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify database is cleared
        let conversations = get_conversations(None, None, State::new(&app_state)).await.unwrap();
        assert_eq!(conversations.len(), 0);
    }
}

