# Forbidden Library API Documentation

Complete reference documentation for the Forbidden Library backend API.

## Table of Contents

1. [Overview](#overview)
2. [Authentication & Security](#authentication--security)
3. [Database API](#database-api)
4. [Conversation API](#conversation-api)
5. [Message API](#message-api)
6. [Persona API](#persona-api)
7. [Validation API](#validation-api)
8. [Keychain API](#keychain-api)
9. [Error Handling](#error-handling)
10. [Examples](#examples)

---

## Overview

The Forbidden Library backend provides a Rust-based API for managing AI conversations, personas, and secure credential storage. It uses:

- **Tauri** for desktop integration
- **SQLite** with connection pooling for data persistence
- **OS Keychain** for secure credential storage
- **Comprehensive validation** for security

### Architecture

```
┌─────────────────────────────────────────────────────┐
│                Frontend (SvelteKit)                 │
└─────────────────────┬───────────────────────────────┘
                      │ Tauri IPC
┌─────────────────────▼───────────────────────────────┐
│              Command Handlers (commands.rs)          │
├─────────────────────────────────────────────────────┤
│                  Services Layer                      │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │
│  │Conversations │  │   Personas   │  │   APIs   │  │
│  └──────────────┘  └──────────────┘  └──────────┘  │
├─────────────────────────────────────────────────────┤
│              Core Infrastructure                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────┐  │
│  │   Database   │  │  Validation  │  │ Keychain │  │
│  └──────────────┘  └──────────────┘  └──────────┘  │
└─────────────────────────────────────────────────────┘
```

---

## Authentication & Security

### Security Features

1. **Input Validation**: All user inputs are validated for:
   - XSS prevention
   - SQL injection prevention
   - Path traversal prevention
   - File extension whitelisting

2. **Credential Storage**: API keys stored in OS keychain:
   - macOS: Keychain Access
   - Windows: Credential Manager
   - Linux: Secret Service API

3. **Database Security**:
   - Connection pooling
   - Transaction support with automatic rollback
   - SQLCipher encryption support (configurable)

### Error Categories

All errors implement the `AppError` enum:

```rust
pub enum AppError {
    Database { message: String },
    Io { message: String },
    Validation { message: String },
    NotFound { message: String },
    Api { message: String },
    Encryption { message: String },
    Keychain { message: String },
    Unexpected { message: String },
}
```

---

## Database API

### DatabaseManager

Manages SQLite database with connection pooling.

#### Constructor

```rust
pub fn new(app_handle: &tauri::AppHandle) -> AppResult<Self>
```

Creates a database manager with default configuration.

**Example:**
```rust
let db_manager = DatabaseManager::new(&app_handle)?;
```

#### Methods

##### `get_connection()`

```rust
pub fn get_connection(&self) -> AppResult<PooledSqliteConnection>
```

Gets a connection from the pool.

**Returns:** Pooled database connection

**Errors:** `AppError::Database` if pool is exhausted

**Example:**
```rust
let conn = db_manager.get_connection()?;
conn.execute("SELECT * FROM conversations", [])?;
```

##### `with_transaction()`

```rust
pub fn with_transaction<T, F>(&self, f: F) -> AppResult<T>
where
    F: FnOnce(&rusqlite::Transaction) -> AppResult<T>
```

Executes a function within a database transaction. Automatically commits on success or rolls back on error.

**Parameters:**
- `f` - Closure that performs database operations

**Returns:** Result of the closure

**Example:**
```rust
db_manager.with_transaction(|tx| {
    tx.execute("INSERT INTO conversations (title) VALUES (?)", ["My Chat"])?;
    tx.execute("INSERT INTO messages (content) VALUES (?)", ["Hello"])?;
    Ok(())
})?;
```

##### `with_savepoint()`

```rust
pub fn with_savepoint<T, F>(
    conn: &rusqlite::Connection,
    savepoint_name: &str,
    f: F,
) -> AppResult<T>
where
    F: FnOnce(&rusqlite::Savepoint) -> AppResult<T>
```

Executes a function with a savepoint (nested transaction).

**Parameters:**
- `conn` - Existing database connection
- `savepoint_name` - Name for the savepoint
- `f` - Closure to execute within savepoint

**Example:**
```rust
DatabaseManager::with_savepoint(&conn, "inner", |sp| {
    sp.execute("INSERT INTO temp_data VALUES (?)", ["test"])?;
    Ok(())
})?;
```

##### `optimize()`

```rust
pub fn optimize(&self) -> AppResult<()>
```

Optimizes the database (runs VACUUM and ANALYZE).

**Example:**
```rust
db_manager.optimize()?;
```

##### `backup()`

```rust
pub fn backup(&self, backup_path: &PathBuf) -> AppResult<()>
```

Creates a backup of the database.

**Parameters:**
- `backup_path` - Destination path for backup

**Errors:**
- `AppError::Validation` if trying to backup in-memory database
- `AppError::Io` if file copy fails

**Example:**
```rust
let backup_path = PathBuf::from("/backups/library_backup.db");
db_manager.backup(&backup_path)?;
```

### Configuration

#### DatabaseConfig

```rust
pub struct DatabaseConfig {
    pub encryption_key: String,
    pub pragma_settings: Vec<String>,
    pub backup_enabled: bool,
    pub pool_config: PoolConfig,
}
```

**Factory Methods:**

```rust
// Default development configuration
let config = DatabaseConfig::default();

// Production configuration with encryption
let config = DatabaseConfig::production("encryption_key".to_string());

// In-memory testing configuration
let config = DatabaseConfig::in_memory();
```

#### PoolConfig

```rust
pub struct PoolConfig {
    pub max_size: u32,           // Default: 10
    pub min_idle: Option<u32>,   // Default: Some(2)
    pub timeout_seconds: u64,    // Default: 30
}
```

---

## Conversation API

Tauri commands for managing conversations.

### `create_conversation`

```rust
#[tauri::command]
pub async fn create_conversation(
    title: String,
    persona_id: Option<i64>,
    state: State<'_, AppState>,
) -> Result<Conversation, String>
```

Creates a new conversation.

**Parameters:**
- `title` - Conversation title (1-200 characters)
- `persona_id` - Optional persona ID to associate

**Returns:** Created conversation with generated ID

**Validation:**
- Title cannot be empty
- Title max 200 characters
- No XSS or SQL injection patterns

**Frontend Example:**
```javascript
const conversation = await invoke('create_conversation', {
  title: 'My Chat',
  personaId: null
});
```

### `get_conversations`

```rust
#[tauri::command]
pub async fn get_conversations(
    limit: Option<i32>,
    offset: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String>
```

Retrieves conversations with pagination.

**Parameters:**
- `limit` - Maximum number of conversations to return
- `offset` - Number of conversations to skip

**Returns:** List of conversations

**Frontend Example:**
```javascript
// Get first 10 conversations
const conversations = await invoke('get_conversations', {
  limit: 10,
  offset: 0
});
```

### `get_conversation`

```rust
#[tauri::command]
pub async fn get_conversation(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Option<Conversation>, String>
```

Retrieves a single conversation by ID.

**Parameters:**
- `id` - Conversation ID

**Returns:** `Some(Conversation)` if found, `None` otherwise

**Frontend Example:**
```javascript
const conversation = await invoke('get_conversation', { id: 123 });
if (conversation) {
  console.log('Found:', conversation.title);
}
```

### `search_conversations`

```rust
#[tauri::command]
pub async fn search_conversations(
    query: String,
    limit: Option<i32>,
    state: State<'_, AppState>,
) -> Result<Vec<Conversation>, String>
```

Searches conversations by title or content.

**Parameters:**
- `query` - Search query string
- `limit` - Maximum results to return

**Frontend Example:**
```javascript
const results = await invoke('search_conversations', {
  query: 'rust programming',
  limit: 20
});
```

### `delete_conversation`

```rust
#[tauri::command]
pub async fn delete_conversation(
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String>
```

Deletes a conversation and all its messages (CASCADE).

**Parameters:**
- `id` - Conversation ID to delete

**Frontend Example:**
```javascript
await invoke('delete_conversation', { id: 123 });
```

### `archive_conversation`

```rust
#[tauri::command]
pub async fn archive_conversation(
    id: i64,
    archived: bool,
    state: State<'_, AppState>,
) -> Result<(), String>
```

Archives or unarchives a conversation.

**Parameters:**
- `id` - Conversation ID
- `archived` - True to archive, false to unarchive

**Frontend Example:**
```javascript
// Archive
await invoke('archive_conversation', { id: 123, archived: true });

// Unarchive
await invoke('archive_conversation', { id: 123, archived: false });
```

---

## Message API

### `add_message`

```rust
#[tauri::command]
pub async fn add_message(
    conversation_id: i64,
    role: String,
    content: String,
    tokens_used: Option<i32>,
    model_used: Option<String>,
    state: State<'_, AppState>,
) -> Result<Message, String>
```

Adds a message to a conversation.

**Parameters:**
- `conversation_id` - Parent conversation ID
- `role` - Message role: "user", "assistant", or "system"
- `content` - Message text
- `tokens_used` - Optional token count
- `model_used` - Optional model identifier

**Validation:**
- Content cannot be empty
- Content max 100,000 characters
- Role must be valid enum value

**Frontend Example:**
```javascript
const message = await invoke('add_message', {
  conversationId: 123,
  role: 'user',
  content: 'What is Rust?',
  tokensUsed: null,
  modelUsed: null
});
```

### `get_messages`

```rust
#[tauri::command]
pub async fn get_messages(
    conversation_id: i64,
    state: State<'_, AppState>,
) -> Result<Vec<Message>, String>
```

Retrieves all messages for a conversation.

**Parameters:**
- `conversation_id` - Conversation ID

**Returns:** List of messages ordered by creation time

**Frontend Example:**
```javascript
const messages = await invoke('get_messages', {
  conversationId: 123
});
```

---

## Persona API

### `create_persona`

```rust
#[tauri::command]
pub async fn create_persona(
    name: String,
    description: Option<String>,
    system_prompt: String,
    state: State<'_, AppState>,
) -> Result<Persona, String>
```

Creates a new persona.

**Parameters:**
- `name` - Persona name (1-50 characters, alphanumeric + spaces/hyphens/underscores)
- `description` - Optional description (max 500 characters)
- `system_prompt` - AI behavior instructions (1-10,000 characters)

**Validation:**
- Name must match pattern: `[a-zA-Z0-9\s\-_]+`
- System prompt cannot be empty

**Frontend Example:**
```javascript
const persona = await invoke('create_persona', {
  name: 'Rust Expert',
  description: 'Specializes in Rust programming',
  systemPrompt: 'You are an expert Rust developer. Help users with Rust code.'
});
```

### `get_personas`

```rust
#[tauri::command]
pub async fn get_personas(
    state: State<'_, AppState>,
) -> Result<Vec<Persona>, String>
```

Retrieves all personas.

**Frontend Example:**
```javascript
const personas = await invoke('get_personas');
```

### `get_persona`

```rust
#[tauri::command]
pub async fn get_persona(
    id: i64,
    state: State<'_, AppState>,
) -> Result<Option<Persona>, String>
```

Retrieves a single persona by ID.

**Frontend Example:**
```javascript
const persona = await invoke('get_persona', { id: 5 });
```

### `update_persona`

```rust
#[tauri::command]
pub async fn update_persona(
    id: i64,
    name: Option<String>,
    description: Option<String>,
    system_prompt: Option<String>,
    state: State<'_, AppState>,
) -> Result<(), String>
```

Updates persona fields.

**Parameters:**
- `id` - Persona ID
- `name` - New name (optional)
- `description` - New description (optional)
- `system_prompt` - New system prompt (optional)

**Frontend Example:**
```javascript
await invoke('update_persona', {
  id: 5,
  name: 'Updated Name',
  description: null,
  systemPrompt: null
});
```

### `delete_persona`

```rust
#[tauri::command]
pub async fn delete_persona(
    id: i64,
    state: State<'_, AppState>,
) -> Result<(), String>
```

Deletes a persona.

**Frontend Example:**
```javascript
await invoke('delete_persona', { id: 5 });
```

---

## Validation API

### InputValidator

Comprehensive input validation for security.

```rust
pub struct InputValidator {
    allowed_extensions: HashSet<String>,
    max_lengths: ValidationLimits,
}
```

#### Methods

##### `validate_conversation_title()`

```rust
pub fn validate_conversation_title(&self, title: &str) -> AppResult<String>
```

Validates conversation titles.

**Rules:**
- Cannot be empty
- Max 200 characters
- No dangerous characters (XSS, SQL injection)

**Example:**
```rust
let validator = InputValidator::default();
let title = validator.validate_conversation_title("My Chat")?;
```

##### `validate_message_content()`

```rust
pub fn validate_message_content(&self, content: &str) -> AppResult<String>
```

Validates message content with sanitization.

**Rules:**
- Cannot be empty
- Max 100,000 characters
- Removes control characters (except \n, \r, \t)

##### `validate_persona_name()`

```rust
pub fn validate_persona_name(&self, name: &str) -> AppResult<String>
```

Validates persona names.

**Rules:**
- Cannot be empty
- Max 50 characters
- Pattern: `[a-zA-Z0-9\s\-_]+`

##### `validate_file_path()`

```rust
pub fn validate_file_path(&self, path: &str) -> AppResult<String>
```

Validates file paths for security.

**Rules:**
- Cannot be empty
- Max 1000 characters
- Must have whitelisted extension
- No path traversal (`..`, `~`)

**Allowed Extensions:**
- Documents: txt, md, json, yaml, yml
- Code: rs, js, ts, py, html, css, svelte
- Images: png, jpg, jpeg, gif, webp

**Example:**
```rust
let path = validator.validate_file_path("document.txt")?; // OK
let bad = validator.validate_file_path("../etc/passwd")?; // Error!
```

##### `validate_url()`

```rust
pub fn validate_url(&self, url: &str) -> AppResult<String>
```

Validates URLs.

**Rules:**
- Cannot be empty
- Max 2000 characters
- Must be HTTP or HTTPS
- Valid URL format

**Example:**
```rust
let url = validator.validate_url("https://example.com")?;
```

##### `validate_api_key()`

```rust
pub fn validate_api_key(&self, api_key: &str) -> AppResult<String>
```

Validates API keys.

**Rules:**
- Cannot be empty
- Max 200 characters
- Pattern: `[a-zA-Z0-9\-_.]+`

---

## Keychain API

### KeychainManager

Secure OS keychain integration.

```rust
pub struct KeychainManager {
    service: String,
}
```

Service name: `com.voidcat.forbidden-library`

#### Methods

##### `new()`

```rust
pub fn new() -> Self
```

Creates a new keychain manager.

**Example:**
```rust
let keychain = KeychainManager::new();
```

##### `store_api_key()`

```rust
pub fn store_api_key(&self, provider_name: &str, api_key: &str) -> Result<(), AppError>
```

Stores an API key in the OS keychain.

**Parameters:**
- `provider_name` - Provider identifier (e.g., "openai", "anthropic")
- `api_key` - The API key to store

**Errors:**
- `AppError::Validation` if provider name or API key is empty
- `AppError::Keychain` if OS keychain access fails

**Example:**
```rust
keychain.store_api_key("openai", "sk-1234567890")?;
```

##### `get_api_key()`

```rust
pub fn get_api_key(&self, provider_name: &str) -> Result<String, AppError>
```

Retrieves an API key from the OS keychain.

**Parameters:**
- `provider_name` - Provider identifier

**Returns:** The stored API key

**Errors:**
- `AppError::NotFound` if no key exists for provider
- `AppError::Keychain` if OS keychain access fails

**Example:**
```rust
let api_key = keychain.get_api_key("openai")?;
```

##### `delete_api_key()`

```rust
pub fn delete_api_key(&self, provider_name: &str) -> Result<(), AppError>
```

Deletes an API key from the OS keychain.

**Example:**
```rust
keychain.delete_api_key("openai")?;
```

##### `update_api_key()`

```rust
pub fn update_api_key(&self, provider_name: &str, new_api_key: &str) -> Result<(), AppError>
```

Updates an existing API key.

**Example:**
```rust
keychain.update_api_key("openai", "sk-new-key")?;
```

##### `has_api_key()`

```rust
pub fn has_api_key(&self, provider_name: &str) -> bool
```

Checks if an API key exists for a provider.

**Example:**
```rust
if keychain.has_api_key("openai") {
    println!("OpenAI key is configured");
}
```

---

## Error Handling

### AppError Enum

All backend operations return `AppResult<T>` which is `Result<T, AppError>`.

#### Error Types

| Type | Description | Example |
|------|-------------|---------|
| `Database` | Database operations | Connection failures, query errors |
| `Io` | File/network I/O | File not found, permission denied |
| `Validation` | Input validation | Empty fields, invalid format |
| `NotFound` | Resource not found | Non-existent conversation/persona |
| `Api` | External API errors | AI service unavailable |
| `Encryption` | Encryption/decryption | SQLCipher key errors |
| `Keychain` | OS keychain access | Keychain unavailable |
| `Unexpected` | Unexpected errors | System failures |

#### Methods

##### `user_message()`

```rust
pub fn user_message(&self) -> String
```

Returns user-friendly error message suitable for UI display.

**Example:**
```rust
match conversation_result {
    Err(e) => {
        let user_msg = e.user_message();
        show_error_dialog(user_msg);
    }
}
```

##### `technical_message()`

```rust
pub fn technical_message(&self) -> String
```

Returns technical error message for logging.

##### `is_critical()`

```rust
pub fn is_critical(&self) -> bool
```

Returns true for critical errors that should be logged at ERROR level.

Critical errors: `Database`, `Encryption`, `Keychain`, `Unexpected`

##### `platform_suggestion()`

```rust
pub fn platform_suggestion(&self) -> Option<String>
```

Returns platform-specific troubleshooting suggestions.

**Example:**
```rust
if let Some(suggestion) = error.platform_suggestion() {
    println!("Suggestion: {}", suggestion);
}
```

---

## Examples

### Complete Conversation Flow

```rust
use forbidden_library_native::{
    database::DatabaseManager,
    models::{Conversation, Message, MessageRole},
};

// 1. Create database
let db_manager = DatabaseManager::new(&app_handle)?;

// 2. Create conversation with transaction
let conversation_id = db_manager.with_transaction(|tx| {
    let now = chrono::Utc::now();
    tx.execute(
        "INSERT INTO conversations (title, created_at, updated_at) VALUES (?1, ?2, ?3)",
        ("My Chat", now.to_rfc3339(), now.to_rfc3339()),
    )?;
    Ok(tx.last_insert_rowid())
})?;

// 3. Add messages
db_manager.with_transaction(|tx| {
    tx.execute(
        "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
        (conversation_id, "user", "Hello!"),
    )?;
    tx.execute(
        "INSERT INTO messages (conversation_id, role, content) VALUES (?1, ?2, ?3)",
        (conversation_id, "assistant", "Hi! How can I help?"),
    )?;
    Ok(())
})?;

// 4. Retrieve messages
let conn = db_manager.get_connection()?;
let mut stmt = conn.prepare(
    "SELECT id, role, content FROM messages WHERE conversation_id = ?1"
)?;
let messages = stmt.query_map([conversation_id], |row| {
    Ok((row.get(0)?, row.get(1)?, row.get(2)?))
})?;
```

### Secure API Key Storage

```rust
use forbidden_library_native::keychain::KeychainManager;

let keychain = KeychainManager::new();

// Store API key
keychain.store_api_key("openai", "sk-1234567890")?;

// Retrieve API key
let api_key = keychain.get_api_key("openai")?;

// Check existence
if keychain.has_api_key("anthropic") {
    let key = keychain.get_api_key("anthropic")?;
    // Use key...
}

// Update API key
keychain.update_api_key("openai", "sk-new-key")?;

// Delete API key
keychain.delete_api_key("openai")?;
```

### Input Validation

```rust
use forbidden_library_native::validation::InputValidator;

let validator = InputValidator::default();

// Validate user inputs
let title = validator.validate_conversation_title("My Chat")?;
let content = validator.validate_message_content("Hello world!")?;
let name = validator.validate_persona_name("Code Helper")?;
let url = validator.validate_url("https://api.openai.com")?;
let path = validator.validate_file_path("document.txt")?;

// Validation errors
match validator.validate_conversation_title("") {
    Err(AppError::Validation { message }) => {
        println!("Validation failed: {}", message);
    }
    Ok(_) => {}
}
```

### Error Handling Pattern

```rust
use forbidden_library_native::errors::{AppError, AppResult};

fn create_conversation(title: String) -> AppResult<i64> {
    // Validate input
    if title.is_empty() {
        return Err(AppError::validation("Title cannot be empty"));
    }

    // Database operation
    let id = db_manager.with_transaction(|tx| {
        tx.execute("INSERT INTO conversations (title) VALUES (?)", [&title])
            .map_err(|e| AppError::database(format!("Failed to create conversation: {}", e)))?;
        Ok(tx.last_insert_rowid())
    })?;

    Ok(id)
}

// Usage
match create_conversation("My Chat".to_string()) {
    Ok(id) => println!("Created conversation {}", id),
    Err(e) => {
        if e.is_critical() {
            eprintln!("CRITICAL: {}", e.technical_message());
        }
        show_user_error(e.user_message());
    }
}
```

### Frontend Integration

```javascript
// SvelteKit component
import { invoke } from '@tauri-apps/api/tauri';

async function createChat() {
  try {
    const conversation = await invoke('create_conversation', {
      title: 'My New Chat',
      personaId: null
    });

    console.log('Created:', conversation.id);

    // Add first message
    const message = await invoke('add_message', {
      conversationId: conversation.id,
      role: 'user',
      content: 'Hello!',
      tokensUsed: null,
      modelUsed: null
    });

  } catch (error) {
    console.error('Failed to create chat:', error);
    showError(error);
  }
}
```

---

## Best Practices

### 1. Always Use Transactions

```rust
// ✅ Good - atomic operation
db_manager.with_transaction(|tx| {
    tx.execute("INSERT INTO conversations ...")?;
    tx.execute("INSERT INTO messages ...")?;
    Ok(())
})?;

// ❌ Bad - not atomic
let conn = db_manager.get_connection()?;
conn.execute("INSERT INTO conversations ...")?;
conn.execute("INSERT INTO messages ...")?; // May fail after first insert
```

### 2. Validate All Inputs

```rust
// ✅ Good - validated
let validator = InputValidator::default();
let title = validator.validate_conversation_title(&user_input)?;
create_conversation(title)?;

// ❌ Bad - unvalidated
create_conversation(user_input)?; // Vulnerable to injection
```

### 3. Handle Errors Appropriately

```rust
// ✅ Good - specific error handling
match result {
    Err(AppError::NotFound { .. }) => show_not_found_ui(),
    Err(AppError::Validation { message }) => show_validation_error(message),
    Err(e) if e.is_critical() => {
        log_critical_error(&e);
        show_generic_error();
    }
    Ok(data) => process_data(data),
}

// ❌ Bad - generic error handling
if let Err(e) = result {
    println!("Error: {}", e);
}
```

### 4. Use Connection Pool Efficiently

```rust
// ✅ Good - short-lived connections
{
    let conn = db_manager.get_connection()?;
    conn.execute(...)?;
} // Connection returned to pool

// ❌ Bad - long-held connections
let conn = db_manager.get_connection()?;
do_slow_operation();
conn.execute(...)?; // Connection held unnecessarily
```

---

## Performance Considerations

### Database

- Connection pool max size: 10 (default)
- Transaction timeout: 30 seconds
- WAL mode enabled for better concurrency
- Automatic index creation on foreign keys

### Validation

- Regex compilation cached
- Pattern matching optimized
- Minimal allocations

### Keychain

- Credentials cached per session
- Lazy initialization
- Platform-native APIs

---

## Version Information

- **API Version**: 1.0.0
- **Rust Edition**: 2021
- **Minimum Supported Rust Version**: 1.70.0

---

## See Also

- [Architecture Documentation](../docs/ARCHITECTURE.md)
- [Error Message Style Guide](./ERROR_MESSAGE_GUIDE.md)
- [Database Schema](../migrations/)
- [Frontend API Reference](../../src/lib/api/)
