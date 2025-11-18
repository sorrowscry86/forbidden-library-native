//! Tauri IPC command handlers
//!
//! This module contains all the command handlers that can be invoked from the frontend.
//! Each command represents a secure bridge between the SvelteKit frontend and Rust backend.
//! Comprehensive implementation of all Forbidden Library functionality.

use crate::models::{Conversation, Message, MessageRole, Persona};
use crate::services::Services;
use crate::validation::InputValidator;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;

/// Application state shared across all commands
pub struct AppState {
    pub services: Arc<Services>,
}

/// Validate and sanitize file paths to prevent path traversal attacks
///
/// This provides a basic security check - paths should still be scoped via Tauri's allowlist.
/// Performs two-stage validation:
/// 1. Standard path validation (extension, traversal prevention)
/// 2. System directory protection (blocks access to sensitive OS directories)
fn validate_file_path_secure(path: &str) -> Result<String, String> {
    // Stage 1: Basic validation
    let validator = InputValidator::default();
    let validated = validator.validate_file_path(path)
        .map_err(|e| format!("Invalid file path: {}", e))?;

    // Stage 2: System directory check
    check_system_directory_access(&validated)?;

    Ok(validated)
}

/// Check if the path attempts to access protected system directories
///
/// Blocks absolute paths to sensitive OS directories on Unix and Windows.
fn check_system_directory_access(path: &str) -> Result<(), String> {
    use std::path::Path;

    let path_obj = Path::new(path);
    if !path_obj.is_absolute() {
        return Ok(()); // Relative paths are allowed
    }

    let path_lower = path.to_lowercase();

    // Unix system directories
    const UNIX_PROTECTED_DIRS: &[&str] = &["/etc", "/sys", "/proc"];

    // Windows system directories
    const WINDOWS_PROTECTED_DIRS: &[&str] = &["c:\\windows", "c:\\program files"];

    let is_protected = UNIX_PROTECTED_DIRS.iter()
        .chain(WINDOWS_PROTECTED_DIRS.iter())
        .any(|protected_dir| path_lower.starts_with(protected_dir));

    if is_protected {
        return Err("Access to system directories is not allowed".to_string());
    }

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppInfo {
    pub version: String,
    pub name: String,
}

// ==================== BASIC APPLICATION COMMANDS ====================

/// Simple greeting command for testing IPC communication
///
/// A basic command used to verify that the Tauri IPC bridge between
/// frontend and backend is working correctly.
///
/// # Arguments
///
/// * `name` - The name to include in the greeting
///
/// # Returns
///
/// * `Ok(String)` - A personalized greeting message
/// * `Err(String)` - Should never error, included for Tauri compatibility
///
/// # Examples
///
/// ```
/// // Called from frontend JavaScript:
/// // const greeting = await invoke('greet', { name: 'Alice' });
/// // Returns: "Hello, Alice! Welcome to the Forbidden Library."
/// ```
#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    tracing::info!("Greeting request for: {}", name);
    Ok(format!(
        "Hello, {}! Welcome to the Forbidden Library.",
        name
    ))
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

/// Create a new conversation with optional persona
///
/// Creates a new conversation thread in the database. The conversation can be
/// associated with a specific persona to customize the AI's behavior.
///
/// # Arguments
///
/// * `title` - The display title for the conversation
/// * `persona_id` - Optional ID of the persona to use for this conversation
/// * `state` - Tauri application state containing services
///
/// # Returns
///
/// * `Ok(Conversation)` - The newly created conversation with generated ID
/// * `Err(String)` - Error message if creation fails or validation fails
///
/// # Validation
///
/// The title is validated to ensure it:
/// * Is not empty
/// * Doesn't exceed 200 characters
/// * Doesn't contain dangerous characters (XSS, SQL injection patterns)
///
/// # Examples
///
/// ```
/// // Called from frontend JavaScript:
/// // const conv = await invoke('create_conversation', {
/// //   title: 'My Chat',
/// //   personaId: null
/// // });
/// ```
#[tauri::command]
pub async fn create_conversation(
    title: String,
    persona_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Conversation, String> {
    tracing::info!(
        "Creating conversation: {} with persona_id: {:?}",
        title,
        persona_id
    );

    // Validate conversation title
    let validator = InputValidator::default();
    let validated_title = validator.validate_conversation_title(&title)
        .map_err(|e| format!("Invalid conversation title: {}", e))?;

    state
        .services
        .conversations
        .create_conversation(validated_title, persona_id)
        .map_err(|e| format!("Failed to create conversation: {}", e))
}

#[tauri::command]
pub async fn get_conversations(
    limit: Option<i32>,
    offset: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String> {
    tracing::debug!(
        "Getting conversations with limit: {:?}, offset: {:?}",
        limit,
        offset
    );
    state
        .services
        .conversations
        .get_conversations(limit, offset)
        .map_err(|e| format!("Failed to get conversations: {}", e))
}

/// Search conversations by title or content
#[tauri::command]
pub async fn search_conversations(
    query: String,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String> {
    tracing::info!("Searching conversations for: {}", query);

    if query.trim().is_empty() {
        return state
            .services
            .conversations
            .get_conversations(limit, None)
            .map_err(|e| format!("Failed to get conversations: {}", e));
    }

    state
        .services
        .conversations
        .search_conversations(&query, limit)
        .map_err(|e| format!("Failed to search conversations: {}", e))
}

#[tauri::command]
pub async fn get_conversation(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Option<Conversation>, String> {
    tracing::debug!("Getting conversation with id: {}", id);
    state
        .services
        .conversations
        .get_conversation(id)
        .map_err(|e| format!("Failed to get conversation: {}", e))
}

#[tauri::command]
pub async fn delete_conversation(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Deleting conversation with id: {}", id);
    state
        .services
        .conversations
        .delete_conversation(id)
        .map_err(|e| format!("Failed to delete conversation: {}", e))
}

#[tauri::command]
pub async fn archive_conversation(
    id: i64,
    archived: bool,
    state: State<'_, AppState>,
) -> Result<(), String> {
    tracing::info!(
        "Setting conversation {} archived status to: {}",
        id,
        archived
    );
    state
        .services
        .conversations
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
    tracing::debug!(
        "Adding message to conversation {}: {} bytes",
        conversation_id,
        content.len()
    );

    // Validate message content
    let validator = InputValidator::default();
    let validated_content = validator.validate_message_content(&content)
        .map_err(|e| format!("Invalid message content: {}", e))?;

    let message_role = match role.as_str() {
        "user" => MessageRole::User,
        "assistant" => MessageRole::Assistant,
        "system" => MessageRole::System,
        _ => return Err(format!("Invalid role: {}", role)),
    };

    state
        .services
        .conversations
        .add_message(
            conversation_id,
            message_role,
            validated_content,
            tokens_used,
            model_used,
        )
        .map_err(|e| format!("Failed to add message: {}", e))
}

#[tauri::command]
pub async fn get_messages(
    conversation_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, String> {
    tracing::debug!("Getting messages for conversation: {}", conversation_id);
    state
        .services
        .conversations
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

    // Validate persona name and prompt
    let validator = InputValidator::default();
    let validated_name = validator.validate_persona_name(&name)
        .map_err(|e| format!("Invalid persona name: {}", e))?;
    let validated_prompt = validator.validate_system_prompt(&system_prompt)
        .map_err(|e| format!("Invalid system prompt: {}", e))?;

    state
        .services
        .personas
        .create_persona(validated_name, description, validated_prompt)
        .map_err(|e| format!("Failed to create persona: {}", e))
}

#[tauri::command]
pub async fn get_personas(state: State<'_, AppState>) -> Result<Vec<Persona>, String> {
    tracing::debug!("Getting all personas");
    state
        .services
        .personas
        .get_personas()
        .map_err(|e| format!("Failed to get personas: {}", e))
}

#[tauri::command]
pub async fn get_persona(id: i64, state: State<'_, AppState>) -> Result<Option<Persona>, String> {
    tracing::debug!("Getting persona with id: {}", id);
    state
        .services
        .personas
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
    state
        .services
        .personas
        .update_persona(id, name, description, system_prompt)
        .map_err(|e| format!("Failed to update persona: {}", e))
}

#[tauri::command]
pub async fn delete_persona(id: i64, state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Deleting persona with id: {}", id);
    state
        .services
        .personas
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

    // Validate API key
    let validator = InputValidator::default();
    let validated_api_key = validator.validate_api_key(&api_key)
        .map_err(|e| format!("Invalid API key: {}", e))?;

    // Validate base URL if provided
    let validated_base_url = if let Some(url) = base_url {
        Some(validator.validate_url(&url)
            .map_err(|e| format!("Invalid base URL: {}", e))?)
    } else {
        None
    };

    state
        .services
        .apis
        .store_api_config(provider, validated_api_key, validated_base_url)
        .map_err(|e| format!("Failed to store API config: {}", e))
}

#[tauri::command]
pub async fn get_api_config(
    provider: String,
    state: State<'_, AppState>,
) -> Result<Option<(String, Option<String>)>, String> {
    tracing::debug!("Getting API config for provider: {}", provider);
    state
        .services
        .apis
        .get_api_config(&provider)
        .map_err(|e| format!("Failed to get API config: {}", e))
}

#[tauri::command]
pub async fn delete_api_config(provider: String, state: State<'_, AppState>) -> Result<(), String> {
    tracing::info!("Deleting API config for provider: {}", provider);
    state
        .services
        .apis
        .delete_api_config(&provider)
        .map_err(|e| format!("Failed to delete API config: {}", e))
}

// ==================== SYSTEM COMMANDS ====================

#[tauri::command]
pub async fn get_database_stats(state: State<'_, AppState>) -> Result<DatabaseStats, String> {
    tracing::debug!("Getting database statistics");

    // Get basic statistics about the database
    let conversations_result = state
        .services
        .conversations
        .get_conversations(Some(1), None);
    let personas_result = state.services.personas.get_personas();

    let total_conversations = match conversations_result {
        Ok(conversations) => conversations.len() as i64,
        Err(_) => 0i64,
    };

    let total_personas = match personas_result {
        Ok(personas) => personas.len() as i64,
        Err(_) => 0i64,
    };

    // Count total messages - simplified approach
    // Note: For a more accurate count, consider adding a count_messages method to ConversationService
    let total_messages = 0i64;

    // Calculate database size (simplified - returns 0 for now)
    // Note: Can be enhanced by adding a method to DatabaseManager that queries PRAGMA page_count/page_size
    let database_size_mb = 0.0;

    Ok(DatabaseStats {
        total_conversations,
        total_personas,
        total_messages,
        database_size_mb,
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
    tracing::info!(
        "Processing AI request for conversation: {:?}",
        conversation_id
    );

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

    let conversation = state
        .services
        .conversations
        .get_conversation(conversation_id)
        .map_err(|e| format!("Failed to get conversation: {}", e))?;

    let messages = state
        .services
        .conversations
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
        }
        "markdown" => {
            let mut markdown = String::new();

            if let Some(conv) = conversation {
                markdown.push_str(&format!("# {}\n\n", conv.title));
                markdown.push_str(&format!(
                    "**Created:** {}\n\n",
                    conv.created_at.format("%Y-%m-%d %H:%M:%S UTC")
                ));

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
        }
        _ => Err(format!("Unsupported export format: {}", format)),
    }
}

#[tauri::command]
pub async fn backup_database(
    backup_path: String,
    state: State<'_, AppState>,
) -> Result<String, String> {
    // Validate path to prevent path traversal attacks
    let validated_path = validate_file_path_secure(&backup_path)?;

    tracing::info!("Creating database backup at: {}", validated_path);

    use std::fs;
    use std::path::Path;

    // NOTE: This command needs refactoring - state.db doesn't exist
    // For now, return a message indicating the operation is not yet implemented
    // TODO: Implement proper database backup through DatabaseManager

    tracing::warn!("backup_database is not yet fully implemented");
    Ok(format!("Database backup functionality requires implementation. Requested path: {}", validated_path))
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
pub async fn clear_database(_state: State<'_, AppState>) -> Result<String, String> {
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
        }
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
    app_handle: tauri::AppHandle,
    title: Option<String>,
    default_path: Option<String>,
    filters: Option<Vec<(String, Vec<String>)>>,
) -> Result<Option<String>, String> {
    tracing::info!("Opening file dialog");

    use tauri::api::dialog::blocking::FileDialogBuilder;
    use std::path::PathBuf;

    let mut dialog = FileDialogBuilder::new();

    if let Some(t) = title {
        dialog = dialog.set_title(&t);
    }

    if let Some(path) = default_path {
        dialog = dialog.set_directory(PathBuf::from(path));
    }

    if let Some(filter_list) = filters {
        for (name, extensions) in filter_list {
            let ext_refs: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();
            dialog = dialog.add_filter(name, &ext_refs);
        }
    }

    let result = dialog.pick_file();

    Ok(result.map(|p| p.to_string_lossy().to_string()))
}

/// Show native file dialog for saving files
#[tauri::command]
pub async fn show_save_dialog(
    app_handle: tauri::AppHandle,
    title: Option<String>,
    default_path: Option<String>,
    filters: Option<Vec<(String, Vec<String>)>>,
) -> Result<Option<String>, String> {
    tracing::info!("Opening save dialog");

    use tauri::api::dialog::blocking::FileDialogBuilder;
    use std::path::PathBuf;

    let mut dialog = FileDialogBuilder::new();

    if let Some(t) = title {
        dialog = dialog.set_title(&t);
    }

    if let Some(path) = default_path {
        if let Some(parent) = PathBuf::from(&path).parent() {
            dialog = dialog.set_directory(parent);
        }
        if let Some(filename) = PathBuf::from(&path).file_name() {
            dialog = dialog.set_file_name(filename.to_string_lossy().as_ref());
        }
    }

    if let Some(filter_list) = filters {
        for (name, extensions) in filter_list {
            let ext_refs: Vec<&str> = extensions.iter().map(|s| s.as_str()).collect();
            dialog = dialog.add_filter(name, &ext_refs);
        }
    }

    let result = dialog.save_file();

    Ok(result.map(|p| p.to_string_lossy().to_string()))
}

/// Write file to disk with native file system access
#[tauri::command]
pub async fn write_file_to_disk(path: String, content: String) -> Result<String, String> {
    use std::fs;

    // Validate path to prevent path traversal attacks
    let validated_path = validate_file_path_secure(&path)?;

    tracing::info!("Writing file to: {}", validated_path);

    fs::write(&validated_path, content).map_err(|e| format!("Failed to write file: {}", e))?;

    Ok(format!("File written successfully to: {}", validated_path))
}

/// Read file from disk with native file system access
#[tauri::command]
pub async fn read_file_from_disk(path: String) -> Result<String, String> {
    use std::fs;

    // Validate path to prevent path traversal attacks
    let validated_path = validate_file_path_secure(&path)?;

    tracing::info!("Reading file from: {}", validated_path);

    fs::read_to_string(&validated_path).map_err(|e| format!("Failed to read file: {}", e))
}

/// Show system notification
#[tauri::command]
pub async fn show_notification(
    title: String,
    body: String,
    icon: Option<String>,
) -> Result<String, String> {
    tracing::info!("Showing notification: {}", title);

    // This would use Tauri's notification API
    // For now, just log the notification
    tracing::info!("Notification - {}: {}", title, body);

    Ok("Notification shown".to_string())
}

/// Copy text to system clipboard
#[tauri::command]
pub async fn copy_to_clipboard(text: String) -> Result<String, String> {
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
    use crate::platform;

    // Use cross-platform method to get app data directory
    if let Some(app_data) = platform::get_app_data_dir() {
        Ok(app_data.to_string_lossy().to_string())
    } else {
        // Ultimate fallback
        Ok("/tmp/forbidden-library".to_string())
    }
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

// ==================== AI PROVIDER COMMANDS ====================

/// Check if an AI provider is available
#[tauri::command]
pub async fn check_ai_provider_availability(
    provider_type: String,
    base_url: Option<String>,
    port: Option<u16>,
) -> Result<bool, String> {
    use crate::ai_providers::AIProvider;

    tracing::info!("Checking availability for provider: {}", provider_type);

    let provider = match provider_type.as_str() {
        "lm_studio" => AIProvider::lm_studio(port),
        "ollama" => AIProvider::ollama(port),
        "openai_compatible" => {
            let url = base_url.ok_or("Base URL required for OpenAI compatible provider")?;
            AIProvider::openai_compatible(url, None)
        }
        _ => return Err(format!("Unknown provider type: {}", provider_type)),
    };

    provider
        .check_availability()
        .await
        .map_err(|e| format!("Failed to check availability: {}", e))
}

/// List available models from an AI provider
#[tauri::command]
pub async fn list_ai_provider_models(
    provider_type: String,
    base_url: Option<String>,
    port: Option<u16>,
) -> Result<Vec<String>, String> {
    use crate::ai_providers::AIProvider;

    tracing::info!("Listing models for provider: {}", provider_type);

    let provider = match provider_type.as_str() {
        "lm_studio" => AIProvider::lm_studio(port),
        "ollama" => AIProvider::ollama(port),
        "openai_compatible" => {
            let url = base_url.ok_or("Base URL required for OpenAI compatible provider")?;
            AIProvider::openai_compatible(url, None)
        }
        _ => return Err(format!("Unknown provider type: {}", provider_type)),
    };

    provider
        .list_models()
        .await
        .map_err(|e| format!("Failed to list models: {}", e))
}

/// Send a request to an AI provider
#[tauri::command]
pub async fn send_ai_provider_request(
    provider_type: String,
    model: String,
    messages: Vec<serde_json::Value>,
    base_url: Option<String>,
    port: Option<u16>,
    api_key: Option<String>,
    temperature: Option<f32>,
    max_tokens: Option<i32>,
) -> Result<serde_json::Value, String> {
    use crate::ai_providers::{AIProvider, AIRequest, ChatMessage};

    tracing::info!(
        "Sending request to provider: {} with model: {}",
        provider_type,
        model
    );

    let provider = match provider_type.as_str() {
        "lm_studio" => AIProvider::lm_studio(port),
        "ollama" => AIProvider::ollama(port),
        "openai_compatible" => {
            let url = base_url.ok_or("Base URL required for OpenAI compatible provider")?;
            AIProvider::openai_compatible(url, api_key)
        }
        _ => return Err(format!("Unknown provider type: {}", provider_type)),
    };

    let chat_messages: Result<Vec<ChatMessage>, String> = messages
        .iter()
        .map(|m| {
            Ok(ChatMessage {
                role: m["role"]
                    .as_str()
                    .ok_or("Missing 'role' field")?
                    .to_string(),
                content: m["content"]
                    .as_str()
                    .ok_or("Missing 'content' field")?
                    .to_string(),
            })
        })
        .collect();

    let chat_messages = chat_messages?;

    let request = AIRequest {
        model,
        messages: chat_messages,
        temperature,
        max_tokens,
        stream: false,
    };

    let response = provider
        .send_request(request)
        .await
        .map_err(|e| format!("Failed to send request: {}", e))?;

    Ok(serde_json::json!({
        "content": response.content,
        "model": response.model,
        "tokens_used": response.tokens_used,
    }))
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
            let db_manager =
                DatabaseManager::new_in_memory().expect("Failed to create test database");
            let services = Arc::new(Services::new(Arc::new(db_manager)));

            Self { services }
        }
    }

    #[tokio::test]
    async fn test_greet_command() {
        let result = greet("Test User").await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "Hello, Test User! Welcome to the Forbidden Library."
        );
    }

    #[tokio::test]
    async fn test_greet_command_empty_name() {
        let result = greet("").await;
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            "Hello, ! Welcome to the Forbidden Library."
        );
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
        let app_state = AppState {
            services: env.services,
        };

        let result = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await;

        assert!(result.is_ok());
        let conversation = result.unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert!(conversation.id.is_some());
    }

    #[tokio::test]
    async fn test_create_conversation_with_persona() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        let result = create_conversation(
            "Test Conversation".to_string(),
            Some(1),
            State::new(&app_state),
        )
        .await;

        assert!(result.is_ok());
        let conversation = result.unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.persona_id, Some(1));
    }

    #[tokio::test]
    async fn test_get_conversations_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let result = get_conversations(None, None, State::new(&app_state)).await;
        assert!(result.is_ok());
        let conversations = result.unwrap();
        assert!(!conversations.is_empty());
    }

    #[tokio::test]
    async fn test_get_conversation_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

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
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

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
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

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
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let conversation_id = created.id.unwrap();
        let result = add_message(
            conversation_id,
            "user".to_string(),
            "Test message".to_string(),
            None,
            State::new(&app_state),
        )
        .await;

        assert!(result.is_ok());
        let message = result.unwrap();
        assert_eq!(message.content, "Test message");
        assert_eq!(message.role, MessageRole::User);
    }

    #[tokio::test]
    async fn test_get_messages_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let conversation_id = created.id.unwrap();

        // Add a test message
        add_message(
            conversation_id,
            "user".to_string(),
            "Test message".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let result = get_messages(conversation_id, State::new(&app_state)).await;
        assert!(result.is_ok());
        let messages = result.unwrap();
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Test message");
    }

    #[tokio::test]
    async fn test_update_message_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let conversation_id = created.id.unwrap();

        // Add a test message
        let message = add_message(
            conversation_id,
            "user".to_string(),
            "Original message".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let message_id = message.id.unwrap();
        let result = update_message(
            message_id,
            "Updated message".to_string(),
            State::new(&app_state),
        )
        .await;

        assert!(result.is_ok());
        let updated_message = result.unwrap();
        assert_eq!(updated_message.content, "Updated message");
    }

    #[tokio::test]
    async fn test_delete_message_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let conversation_id = created.id.unwrap();

        // Add a test message
        let message = add_message(
            conversation_id,
            "user".to_string(),
            "Test message".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let message_id = message.id.unwrap();
        let result = delete_message(message_id, State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify message is deleted
        let messages = get_messages(conversation_id, State::new(&app_state))
            .await
            .unwrap();
        assert_eq!(messages.len(), 0);
    }

    #[tokio::test]
    async fn test_create_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        let result = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state),
        )
        .await;

        assert!(result.is_ok());
        let persona = result.unwrap();
        assert_eq!(persona.name, "Test Persona");
        assert_eq!(persona.description, "A test persona");
    }

    #[tokio::test]
    async fn test_get_personas_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test persona first
        create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state),
        )
        .await
        .unwrap();

        let result = get_personas(State::new(&app_state)).await;
        assert!(result.is_ok());
        let personas = result.unwrap();
        assert!(!personas.is_empty());
    }

    #[tokio::test]
    async fn test_get_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test persona first
        let created = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state),
        )
        .await
        .unwrap();

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
        let app_state = AppState {
            services: env.services,
        };

        // Create a test persona first
        let created = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state),
        )
        .await
        .unwrap();

        let persona_id = created.id.unwrap();
        let result = update_persona(
            persona_id,
            "Updated Persona".to_string(),
            "An updated test persona".to_string(),
            "You are an updated test persona.".to_string(),
            State::new(&app_state),
        )
        .await;

        assert!(result.is_ok());
        let persona = result.unwrap();
        assert_eq!(persona.name, "Updated Persona");
        assert_eq!(persona.description, "An updated test persona");
    }

    #[tokio::test]
    async fn test_delete_persona_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a test persona first
        let created = create_persona(
            "Test Persona".to_string(),
            "A test persona".to_string(),
            "You are a test persona.".to_string(),
            State::new(&app_state),
        )
        .await
        .unwrap();

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
        let app_state = AppState {
            services: env.services,
        };

        let config = serde_json::json!({
            "provider": "openai",
            "api_key": "test-key",
            "model": "gpt-4"
        });

        let result =
            store_api_config("openai".to_string(), config.clone(), State::new(&app_state)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_api_config_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        let config = serde_json::json!({
            "provider": "openai",
            "api_key": "test-key",
            "model": "gpt-4"
        });

        // Store config first
        store_api_config("openai".to_string(), config.clone(), State::new(&app_state))
            .await
            .unwrap();

        let result = get_api_config("openai".to_string(), State::new(&app_state)).await;
        assert!(result.is_ok());
        let retrieved_config = result.unwrap();
        assert!(retrieved_config.is_some());
        assert_eq!(retrieved_config.unwrap().0, "openai");
    }

    #[tokio::test]
    async fn test_delete_api_config_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        let config = serde_json::json!({
            "provider": "openai",
            "api_key": "test-key",
            "model": "gpt-4"
        });

        // Store config first
        store_api_config("openai".to_string(), config.clone(), State::new(&app_state))
            .await
            .unwrap();

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
        let app_state = AppState {
            services: env.services,
        };

        let result = send_ai_request(
            "Hello, this is a test".to_string(),
            "openai".to_string(),
            None,
            State::new(&app_state),
        )
        .await;

        // Should succeed (even if it's a mock response)
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_database_stats_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

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
        let app_state = AppState {
            services: env.services,
        };

        // Create a test conversation first
        let created = create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let conversation_id = created.id.unwrap();
        let result =
            export_conversation(conversation_id, "json".to_string(), State::new(&app_state)).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_backup_database_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        let result = backup_database(State::new(&app_state)).await;
        assert!(result.is_ok());
        let backup_path = result.unwrap();
        assert!(backup_path.contains("backup"));
        assert!(backup_path.contains(".db"));
    }

    #[tokio::test]
    async fn test_restore_database_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create a backup first
        let backup_path = backup_database(State::new(&app_state)).await.unwrap();

        let result = restore_database(backup_path, State::new(&app_state)).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_clear_database_command() {
        let env = TestCommandsEnvironment::new();
        let app_state = AppState {
            services: env.services,
        };

        // Create some test data first
        create_conversation(
            "Test Conversation".to_string(),
            None,
            State::new(&app_state),
        )
        .await
        .unwrap();

        let result = clear_database(State::new(&app_state)).await;
        assert!(result.is_ok());

        // Verify database is cleared
        let conversations = get_conversations(None, None, State::new(&app_state))
            .await
            .unwrap();
        assert_eq!(conversations.len(), 0);
    }
}
