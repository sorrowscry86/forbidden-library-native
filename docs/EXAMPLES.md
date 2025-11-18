# Forbidden Library Usage Examples

Practical examples and tutorials for common tasks in the Forbidden Library.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Conversation Management](#basic-conversation-management)
3. [Working with Personas](#working-with-personas)
4. [Message Operations](#message-operations)
5. [Secure Credential Storage](#secure-credential-storage)
6. [Advanced Database Operations](#advanced-database-operations)
7. [Error Handling Patterns](#error-handling-patterns)
8. [Frontend Integration](#frontend-integration)
9. [Testing Patterns](#testing-patterns)
10. [Performance Optimization](#performance-optimization)

---

## Getting Started

### Installation and Setup

#### Backend Setup

```bash
# Navigate to the Tauri backend
cd src-tauri

# Build the project
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

#### Frontend Setup

```bash
# Install dependencies
pnpm install

# Run development server
pnpm dev

# Run tests
pnpm test

# Build for production
pnpm build
```

### Project Structure

```
forbidden-library-native/
├── src/                    # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── components/    # Svelte components
│   │   ├── stores/        # State management
│   │   └── api/           # API client
│   └── routes/            # Pages
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── commands.rs    # Tauri commands
│   │   ├── database/      # Database layer
│   │   ├── models.rs      # Data models
│   │   ├── validation.rs  # Input validation
│   │   └── keychain.rs    # Credential storage
│   └── tests/             # Integration tests
└── docs/                  # Documentation
```

---

## Basic Conversation Management

### Example 1: Create Your First Conversation

**Backend (Rust):**

```rust
use forbidden_library_native::{
    database::DatabaseManager,
    models::{Conversation, Message, MessageRole},
};

fn create_first_conversation(db: &DatabaseManager) -> Result<i64, Box<dyn std::error::Error>> {
    // Create conversation in a transaction
    let conversation_id = db.with_transaction(|tx| {
        let now = chrono::Utc::now();

        tx.execute(
            "INSERT INTO conversations (uuid, title, created_at, updated_at, archived)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            (
                uuid::Uuid::new_v4().to_string(),
                "My First Chat",
                now.to_rfc3339(),
                now.to_rfc3339(),
                false,
            ),
        )?;

        Ok(tx.last_insert_rowid())
    })?;

    println!("✅ Created conversation with ID: {}", conversation_id);
    Ok(conversation_id)
}
```

**Frontend (Svelte):**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  async function createConversation() {
    try {
      const conversation = await invoke('create_conversation', {
        title: 'My First Chat',
        personaId: null
      });

      console.log('Created:', conversation);
      return conversation.id;
    } catch (error) {
      console.error('Failed to create conversation:', error);
      throw error;
    }
  }
</script>

<button on:click={createConversation}>
  Create New Chat
</button>
```

### Example 2: List All Conversations

**Backend (Rust):**

```rust
use forbidden_library_native::database::DatabaseManager;

fn list_conversations(db: &DatabaseManager) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let conn = db.get_connection()?;

    let mut stmt = conn.prepare(
        "SELECT id, title, created_at, archived
         FROM conversations
         ORDER BY created_at DESC"
    )?;

    let conversations = stmt.query_map([], |row| {
        Ok(format!(
            "ID: {}, Title: {}, Archived: {}",
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, bool>(3)?
        ))
    })?.collect::<Result<Vec<_>, _>>()?;

    for conv in &conversations {
        println!("{}", conv);
    }

    Ok(conversations)
}
```

**Frontend (Svelte):**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';

  let conversations = [];
  let loading = true;

  onMount(async () => {
    try {
      conversations = await invoke('get_conversations', {
        limit: 50,
        offset: 0
      });
    } catch (error) {
      console.error('Failed to load conversations:', error);
    } finally {
      loading = false;
    }
  });
</script>

{#if loading}
  <p>Loading conversations...</p>
{:else if conversations.length === 0}
  <p>No conversations yet. Create your first one!</p>
{:else}
  <ul>
    {#each conversations as conv}
      <li>
        <strong>{conv.title}</strong>
        <small>{new Date(conv.created_at).toLocaleDateString()}</small>
      </li>
    {/each}
  </ul>
{/if}
```

### Example 3: Search Conversations

```typescript
// utils/search.ts
import { invoke } from '@tauri-apps/api/tauri';

export async function searchConversations(query: string) {
  if (!query.trim()) {
    return await invoke('get_conversations', { limit: 50, offset: 0 });
  }

  return await invoke('search_conversations', {
    query: query.trim(),
    limit: 20
  });
}

// Component usage
let searchQuery = '';
let results = [];

async function handleSearch() {
  results = await searchConversations(searchQuery);
}
```

---

## Working with Personas

### Example 4: Create a Custom Persona

**Backend (Rust):**

```rust
use forbidden_library_native::{
    database::DatabaseManager,
    validation::InputValidator,
};

fn create_coding_assistant(db: &DatabaseManager) -> Result<i64, Box<dyn std::error::Error>> {
    let validator = InputValidator::default();

    // Validate inputs
    let name = validator.validate_persona_name("Rust Expert")?;
    let description = "Specializes in Rust programming and best practices";
    let system_prompt = validator.validate_system_prompt(
        "You are an expert Rust developer with deep knowledge of the language, \
         ecosystem, and best practices. Help users write idiomatic, safe, and \
         efficient Rust code. Explain concepts clearly and provide examples."
    )?;

    // Create persona
    let persona_id = db.with_transaction(|tx| {
        let now = chrono::Utc::now();

        tx.execute(
            "INSERT INTO personas (id, name, description, system_prompt, created_at, updated_at, active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            (
                uuid::Uuid::new_v4().to_string(),
                name,
                description,
                system_prompt,
                now.to_rfc3339(),
                now.to_rfc3339(),
                true,
            ),
        )?;

        Ok(tx.last_insert_rowid())
    })?;

    println!("✅ Created Rust Expert persona with ID: {}", persona_id);
    Ok(persona_id)
}
```

**Frontend (Svelte):**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let formData = {
    name: '',
    description: '',
    systemPrompt: ''
  };

  async function createPersona() {
    try {
      const persona = await invoke('create_persona', {
        name: formData.name,
        description: formData.description || null,
        systemPrompt: formData.systemPrompt
      });

      console.log('✅ Created persona:', persona);
      // Reset form
      formData = { name: '', description: '', systemPrompt: '' };
    } catch (error) {
      console.error('❌ Failed to create persona:', error);
      alert(`Error: ${error}`);
    }
  }
</script>

<form on:submit|preventDefault={createPersona}>
  <label>
    Name:
    <input type="text" bind:value={formData.name} required maxlength="50" />
  </label>

  <label>
    Description:
    <textarea bind:value={formData.description} maxlength="500" />
  </label>

  <label>
    System Prompt:
    <textarea bind:value={formData.systemPrompt} required maxlength="10000" />
  </label>

  <button type="submit">Create Persona</button>
</form>
```

### Example 5: Use Persona in Conversation

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let personas = [];
  let selectedPersonaId = null;

  async function loadPersonas() {
    personas = await invoke('get_personas');
  }

  async function createConversationWithPersona(title: string) {
    return await invoke('create_conversation', {
      title,
      personaId: selectedPersonaId
    });
  }
</script>

<select bind:value={selectedPersonaId}>
  <option value={null}>No Persona</option>
  {#each personas as persona}
    <option value={persona.id}>{persona.name}</option>
  {/each}
</select>

<button on:click={() => createConversationWithPersona('New Chat')}>
  Create Chat with Selected Persona
</button>
```

---

## Message Operations

### Example 6: Send and Receive Messages

**Complete Chat Flow:**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let conversationId: number | null = null;
  let messages = [];
  let inputText = '';
  let sending = false;

  async function sendMessage() {
    if (!inputText.trim() || sending || !conversationId) return;

    const userMessage = inputText.trim();
    inputText = ''; // Clear input immediately for better UX
    sending = true;

    try {
      // Add user message
      const userMsg = await invoke('add_message', {
        conversationId,
        role: 'user',
        content: userMessage,
        tokensUsed: null,
        modelUsed: null
      });

      messages = [...messages, userMsg];

      // TODO: Send to AI service and get response
      // For now, simulate a response
      await new Promise(resolve => setTimeout(resolve, 1000));

      const assistantMsg = await invoke('add_message', {
        conversationId,
        role: 'assistant',
        content: 'This is a simulated response.',
        tokensUsed: 50,
        modelUsed: 'gpt-4'
      });

      messages = [...messages, assistantMsg];

    } catch (error) {
      console.error('Failed to send message:', error);
      inputText = userMessage; // Restore message on error
      alert(`Error: ${error}`);
    } finally {
      sending = false;
    }
  }

  async function loadMessages() {
    if (!conversationId) return;

    try {
      messages = await invoke('get_messages', { conversationId });
    } catch (error) {
      console.error('Failed to load messages:', error);
    }
  }
</script>

<div class="chat-container">
  <div class="messages">
    {#each messages as message}
      <div class="message {message.role}">
        <strong>{message.role}:</strong>
        <p>{message.content}</p>
        {#if message.tokens_used}
          <small>{message.tokens_used} tokens</small>
        {/if}
      </div>
    {/each}
  </div>

  <form on:submit|preventDefault={sendMessage}>
    <input
      type="text"
      bind:value={inputText}
      placeholder="Type your message..."
      disabled={sending}
    />
    <button type="submit" disabled={sending || !inputText.trim()}>
      {sending ? 'Sending...' : 'Send'}
    </button>
  </form>
</div>
```

### Example 7: Message Pagination

```typescript
// Load messages in batches
async function loadMoreMessages(conversationId: number, offset: number = 0) {
  const BATCH_SIZE = 20;

  const conn = db.get_connection()?;
  const mut stmt = conn.prepare(
    "SELECT id, role, content, created_at, tokens_used
     FROM messages
     WHERE conversation_id = ?1
     ORDER BY created_at ASC
     LIMIT ?2 OFFSET ?3"
  )?;

  return stmt.query_map([conversationId, BATCH_SIZE, offset], |row| {
    Ok(Message {
      id: row.get(0)?,
      role: row.get(1)?,
      content: row.get(2)?,
      // ... map other fields
    })
  })?;
}
```

---

## Secure Credential Storage

### Example 8: Store AI Provider API Keys

**Backend (Rust):**

```rust
use forbidden_library_native::keychain::KeychainManager;

fn setup_api_keys() -> Result<(), Box<dyn std::error::Error>> {
    let keychain = KeychainManager::new();

    // Store OpenAI key
    keychain.store_api_key("openai", "sk-proj-xxxxxxxxxxxxx")?;
    println!("✅ Stored OpenAI API key");

    // Store Anthropic key
    keychain.store_api_key("anthropic", "sk-ant-xxxxxxxxxxxxx")?;
    println!("✅ Stored Anthropic API key");

    // Verify storage
    if keychain.has_api_key("openai") {
        println!("✅ OpenAI key is configured");
    }

    Ok(())
}

fn get_api_key_safely(provider: &str) -> Result<String, Box<dyn std::error::Error>> {
    let keychain = KeychainManager::new();

    match keychain.get_api_key(provider) {
        Ok(key) => {
            println!("✅ Retrieved key for {}", provider);
            Ok(key)
        }
        Err(e) => {
            eprintln!("❌ Failed to get key for {}: {}", provider, e);
            Err(Box::new(e))
        }
    }
}
```

**Frontend (Svelte):**

```svelte
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';

  let apiKeys = {
    openai: '',
    anthropic: '',
    google: ''
  };

  async function saveApiKey(provider: string, apiKey: string) {
    try {
      await invoke('store_api_config', {
        provider,
        apiKey,
        baseUrl: null
      });

      console.log(`✅ Saved API key for ${provider}`);
      apiKeys[provider] = ''; // Clear input
    } catch (error) {
      console.error(`❌ Failed to save ${provider} key:`, error);
      alert(`Error: ${error}`);
    }
  }

  async function testApiKey(provider: string) {
    try {
      const config = await invoke('get_api_config', { provider });
      if (config) {
        console.log(`✅ ${provider} key is configured`);
        return true;
      } else {
        console.log(`⚠️ No ${provider} key found`);
        return false;
      }
    } catch (error) {
      console.error(`❌ Error checking ${provider} key:`, error);
      return false;
    }
  }
</script>

<div class="api-config">
  <h3>API Configuration</h3>

  {#each Object.entries(apiKeys) as [provider, value]}
    <div class="provider-config">
      <label>
        {provider.toUpperCase()}:
        <input
          type="password"
          bind:value={apiKeys[provider]}
          placeholder="Enter API key"
        />
      </label>
      <button on:click={() => saveApiKey(provider, apiKeys[provider])}>
        Save
      </button>
      <button on:click={() => testApiKey(provider)}>
        Test
      </button>
    </div>
  {/each}
</div>

<style>
  .provider-config {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
    align-items: center;
  }

  input[type="password"] {
    flex: 1;
    padding: 0.5rem;
    font-family: monospace;
  }
</style>
```

---

## Advanced Database Operations

### Example 9: Bulk Operations with Transactions

```rust
use forbidden_library_native::database::DatabaseManager;

fn bulk_create_messages(
    db: &DatabaseManager,
    conversation_id: i64,
    messages: &[(String, String)], // (role, content)
) -> Result<Vec<i64>, Box<dyn std::error::Error>> {
    let message_ids = db.with_transaction(|tx| {
        let mut ids = Vec::new();

        for (role, content) in messages {
            tx.execute(
                "INSERT INTO messages (conversation_id, role, content, created_at)
                 VALUES (?1, ?2, ?3, ?4)",
                (conversation_id, role, content, chrono::Utc::now().to_rfc3339()),
            )?;

            ids.push(tx.last_insert_rowid());
        }

        Ok(ids)
    })?;

    println!("✅ Created {} messages", message_ids.len());
    Ok(message_ids)
}

// Usage
let messages = vec![
    ("user".to_string(), "Hello!".to_string()),
    ("assistant".to_string(), "Hi there!".to_string()),
    ("user".to_string(), "How are you?".to_string()),
    ("assistant".to_string(), "I'm doing great, thanks!".to_string()),
];

bulk_create_messages(&db, conversation_id, &messages)?;
```

### Example 10: Nested Transactions with Savepoints

```rust
fn complex_conversation_update(db: &DatabaseManager) -> Result<(), Box<dyn std::error::Error>> {
    db.with_transaction(|tx| {
        // Update conversation
        tx.execute(
            "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
            (chrono::Utc::now().to_rfc3339(), 123),
        )?;

        // Try to add experimental data in a savepoint
        let experimental_result = DatabaseManager::with_savepoint(tx, "experimental", |sp| {
            sp.execute(
                "INSERT INTO experimental_features (conversation_id, data) VALUES (?1, ?2)",
                (123, "test_data"),
            )?;

            // Simulate validation
            if should_reject_experimental_data() {
                return Err(AppError::validation("Experimental data rejected"));
            }

            Ok(())
        });

        // If experimental data fails, we can still continue with main transaction
        match experimental_result {
            Ok(_) => println!("✅ Experimental data added"),
            Err(e) => println!("⚠️ Experimental data skipped: {}", e),
        }

        // Main transaction continues regardless
        tx.execute(
            "INSERT INTO activity_log (conversation_id, action) VALUES (?1, ?2)",
            (123, "updated"),
        )?;

        Ok(())
    })?;

    Ok(())
}
```

---

## Error Handling Patterns

### Example 11: Comprehensive Error Handling

```rust
use forbidden_library_native::errors::{AppError, AppResult};

fn handle_conversation_operation(db: &DatabaseManager) -> AppResult<()> {
    match create_conversation_with_validation(db) {
        Ok(id) => {
            println!("✅ Success! Created conversation {}", id);
            Ok(())
        }
        Err(AppError::Validation { message }) => {
            eprintln!("⚠️ Validation error: {}", message);
            // Show user-friendly message
            show_validation_error(&message);
            Err(AppError::validation(message))
        }
        Err(AppError::Database { message }) => {
            eprintln!("❌ Database error: {}", message);
            // Log for debugging
            log_database_error(&message);
            // Show generic error to user
            show_error_dialog("Failed to save conversation. Please try again.");
            Err(AppError::database(message))
        }
        Err(e) if e.is_critical() => {
            eprintln!("🚨 CRITICAL ERROR: {}", e.technical_message());
            // Log to monitoring system
            report_critical_error(&e);
            // Show system error
            show_error_dialog(&e.user_message());
            Err(e)
        }
        Err(e) => {
            eprintln!("⚠️ Error: {}", e);
            show_error_dialog(&e.user_message());
            Err(e)
        }
    }
}
```

**Frontend Error Handling:**

```typescript
// utils/errorHandler.ts
import { AppError } from '../types';

export function handleApiError(error: unknown): string {
  console.error('API Error:', error);

  // Check if it's a string error from Tauri
  if (typeof error === 'string') {
    // Parse error category if formatted as "Category: message"
    const match = error.match(/^(\w+) error: (.+)$/);
    if (match) {
      const [, category, message] = match;

      switch (category) {
        case 'Validation':
          return message; // Show exact validation message
        case 'NotFound':
          return message; // Show exact not found message
        case 'Database':
          return 'A database error occurred. Please try again.';
        case 'Keychain':
          return 'Failed to access secure storage. Please check your system settings.';
        default:
          return 'An unexpected error occurred. Please try again.';
      }
    }
  }

  return 'An error occurred. Please try again.';
}

// Component usage
try {
  await invoke('create_conversation', { title });
} catch (error) {
  const userMessage = handleApiError(error);
  showToast(userMessage, 'error');
}
```

---

## Frontend Integration

### Example 12: Complete Chat Component

```svelte
<!-- ChatInterface.svelte -->
<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount, onDestroy } from 'svelte';

  export let conversationId: number | null;

  let messages = [];
  let inputText = '';
  let loading = false;
  let sending = false;
  let error: string | null = null;

  $: if (conversationId) {
    loadMessages();
  }

  async function loadMessages() {
    if (!conversationId || loading) return;

    loading = true;
    error = null;

    try {
      messages = await invoke('get_messages', { conversationId });
    } catch (e) {
      error = `Failed to load messages: ${e}`;
      console.error(error);
    } finally {
      loading = false;
    }
  }

  async function sendMessage() {
    if (!inputText.trim() || sending || !conversationId) return;

    const userMessage = inputText.trim();
    const originalInput = inputText; // Store for restore on error
    inputText = ''; // Clear optimistically
    sending = true;
    error = null;

    try {
      // Add user message
      const userMsg = await invoke('add_message', {
        conversationId,
        role: 'user',
        content: userMessage,
        tokensUsed: null,
        modelUsed: null
      });

      messages = [...messages, userMsg];

      // Scroll to bottom
      setTimeout(scrollToBottom, 100);

    } catch (e) {
      error = `Failed to send message: ${e}`;
      inputText = originalInput; // Restore on error
      console.error(error);
    } finally {
      sending = false;
    }
  }

  function scrollToBottom() {
    const container = document.querySelector('.messages-container');
    if (container) {
      container.scrollTop = container.scrollHeight;
    }
  }

  onMount(() => {
    if (conversationId) {
      loadMessages();
    }
  });
</script>

<div class="chat-interface">
  {#if error}
    <div class="error-banner">
      {error}
      <button on:click={() => error = null}>Dismiss</button>
    </div>
  {/if}

  {#if loading}
    <div class="loading">Loading messages...</div>
  {:else}
    <div class="messages-container">
      {#each messages as message}
        <div class="message {message.role}">
          <div class="message-header">
            <span class="role">{message.role}</span>
            <span class="time">
              {new Date(message.created_at).toLocaleTimeString()}
            </span>
          </div>
          <div class="message-content">{message.content}</div>
          {#if message.tokens_used}
            <div class="message-meta">
              {message.tokens_used} tokens
              {#if message.model_used}
                • {message.model_used}
              {/if}
            </div>
          {/if}
        </div>
      {/each}
    </div>
  {/if}

  <form class="input-form" on:submit|preventDefault={sendMessage}>
    <input
      type="text"
      bind:value={inputText}
      placeholder="Type your message..."
      disabled={sending || loading}
      class="message-input"
    />
    <button
      type="submit"
      disabled={sending || !inputText.trim() || !conversationId}
      class="send-button"
    >
      {sending ? 'Sending...' : 'Send'}
    </button>
  </form>
</div>

<style>
  .chat-interface {
    display: flex;
    flex-direction: column;
    height: 100%;
  }

  .messages-container {
    flex: 1;
    overflow-y: auto;
    padding: 1rem;
  }

  .message {
    margin-bottom: 1rem;
    padding: 1rem;
    border-radius: 8px;
  }

  .message.user {
    background: #e3f2fd;
    margin-left: 20%;
  }

  .message.assistant {
    background: #f5f5f5;
    margin-right: 20%;
  }

  .message.system {
    background: #fff3e0;
    font-style: italic;
  }

  .input-form {
    display: flex;
    gap: 0.5rem;
    padding: 1rem;
    border-top: 1px solid #ddd;
  }

  .message-input {
    flex: 1;
    padding: 0.75rem;
    border: 1px solid #ddd;
    border-radius: 4px;
    font-size: 1rem;
  }

  .send-button {
    padding: 0.75rem 1.5rem;
    background: #1976d2;
    color: white;
    border: none;
    border-radius: 4px;
    cursor: pointer;
  }

  .send-button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
```

---

## Testing Patterns

### Example 13: Backend Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use forbidden_library_native::database::DatabaseManager;

    #[test]
    fn test_complete_conversation_flow() {
        // Setup
        let db = DatabaseManager::new_in_memory().unwrap();

        // Create conversation
        let conv_id = db.with_transaction(|tx| {
            tx.execute(
                "INSERT INTO conversations (title, created_at, updated_at)
                 VALUES (?1, ?2, ?3)",
                ("Test", "2024-01-01T00:00:00Z", "2024-01-01T00:00:00Z"),
            )?;
            Ok(tx.last_insert_rowid())
        }).unwrap();

        assert!(conv_id > 0);

        // Add messages
        db.with_transaction(|tx| {
            tx.execute(
                "INSERT INTO messages (conversation_id, role, content)
                 VALUES (?1, ?2, ?3)",
                (conv_id, "user", "Hello"),
            )?;
            Ok(())
        }).unwrap();

        // Verify
        let conn = db.get_connection().unwrap();
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM messages WHERE conversation_id = ?1",
            [conv_id],
            |row| row.get(0),
        ).unwrap();

        assert_eq!(count, 1);
    }
}
```

### Example 14: Frontend Component Tests

```typescript
// ChatInterface.test.ts
import { render, fireEvent, waitFor } from '@testing-library/svelte';
import { vi } from 'vitest';
import ChatInterface from './ChatInterface.svelte';

// Mock Tauri invoke
vi.mock('@tauri-apps/api/tauri', () => ({
  invoke: vi.fn()
}));

describe('ChatInterface', () => {
  it('loads messages on mount', async () => {
    const mockMessages = [
      { id: 1, role: 'user', content: 'Hello', created_at: new Date().toISOString() }
    ];

    invoke.mockResolvedValue(mockMessages);

    const { getByText } = render(ChatInterface, {
      props: { conversationId: 123 }
    });

    await waitFor(() => {
      expect(getByText('Hello')).toBeInTheDocument();
    });
  });

  it('sends message when form submitted', async () => {
    const { getByPlaceholderText, getByText } = render(ChatInterface, {
      props: { conversationId: 123 }
    });

    const input = getByPlaceholderText('Type your message...');
    const sendButton = getByText('Send');

    await fireEvent.input(input, { target: { value: 'Test message' } });
    await fireEvent.click(sendButton);

    expect(invoke).toHaveBeenCalledWith('add_message', expect.objectContaining({
      content: 'Test message'
    }));
  });
});
```

---

## Performance Optimization

### Example 15: Efficient Message Loading

```typescript
// Implement virtual scrolling for large conversations
import { invoke } from '@tauri-apps/api/tauri';

class MessagePaginator {
  private conversationId: number;
  private pageSize = 50;
  private currentPage = 0;
  private allLoaded = false;

  constructor(conversationId: number) {
    this.conversationId = conversationId;
  }

  async loadNextPage(): Promise<Message[]> {
    if (this.allLoaded) return [];

    const messages = await invoke('get_messages_paginated', {
      conversationId: this.conversationId,
      limit: this.pageSize,
      offset: this.currentPage * this.pageSize
    });

    if (messages.length < this.pageSize) {
      this.allLoaded = true;
    }

    this.currentPage++;
    return messages;
  }

  reset() {
    this.currentPage = 0;
    this.allLoaded = false;
  }
}
```

### Example 16: Debounced Search

```typescript
// utils/debounce.ts
export function debounce<T extends (...args: any[]) => any>(
  func: T,
  wait: number
): (...args: Parameters<T>) => void {
  let timeout: ReturnType<typeof setTimeout> | null = null;

  return function executedFunction(...args: Parameters<T>) {
    const later = () => {
      timeout = null;
      func(...args);
    };

    if (timeout) clearTimeout(timeout);
    timeout = setTimeout(later, wait);
  };
}

// Usage in component
let searchQuery = '';
let searchResults = [];

const debouncedSearch = debounce(async (query: string) => {
  if (!query.trim()) {
    searchResults = [];
    return;
  }

  searchResults = await invoke('search_conversations', {
    query,
    limit: 20
  });
}, 300);

$: debouncedSearch(searchQuery);
```

---

## See Also

- [API Documentation](./API.md)
- [Architecture Guide](./ARCHITECTURE.md)
- [Error Message Style Guide](../src-tauri/ERROR_MESSAGE_GUIDE.md)
- [Troubleshooting Guide](./TROUBLESHOOTING.md)
