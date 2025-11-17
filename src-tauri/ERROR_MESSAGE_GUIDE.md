# Error Message Style Guide

This document defines the standard format for error messages in the Forbidden Library backend.

## General Rules

1. **Capitalization**: Start with a capital letter
2. **Punctuation**: No period at the end (unless multiple sentences)
3. **Clarity**: Be specific about what went wrong
4. **Context**: Include relevant resource names or identifiers
5. **Actionability**: Suggest what the user should do when appropriate

## Message Patterns

### Pattern 1: Resource Not Found
```rust
// ✅ Good
AppError::not_found(format!("Conversation with ID {} not found", id))
AppError::not_found(format!("Persona '{}' does not exist", name))

// ❌ Avoid
AppError::not_found("Not found")
AppError::not_found("The conversation doesn't exist")
```

### Pattern 2: Validation Errors
```rust
// ✅ Good
AppError::validation("Email address is required")
AppError::validation(format!("Title exceeds maximum length of {} characters", MAX_LENGTH))

// ❌ Avoid
AppError::validation("Invalid input")
AppError::validation("The title is too long.")
```

### Pattern 3: Operation Failures
```rust
// ✅ Good - Use "Failed to" for operations
AppError::database("Failed to create conversation")
AppError::io("Failed to write file")
AppError::keychain(format!("Failed to store API key: {}", details))

// ❌ Avoid - Vague or inconsistent
AppError::database("Cannot create")
AppError::io("Error writing")
```

### Pattern 4: Missing Requirements
```rust
// ✅ Good - Use "X is required" or "X cannot be empty"
AppError::validation("Provider name cannot be empty")
AppError::validation("API key is required")

// ❌ Avoid
AppError::validation("No provider name")
AppError::validation("You must provide an API key")
```

### Pattern 5: Permission/Access Issues
```rust
// ✅ Good
AppError::io("Access denied to system directory")
AppError::validation("Path traversal is not allowed")

// ❌ Avoid
AppError::io("Can't access")
AppError::validation("No permission")
```

## Category-Specific Guidelines

### Database Errors
- Format: "Failed to [action] [resource]"
- Include table/resource name when possible
- Examples:
  - "Failed to create conversation"
  - "Failed to update message with ID 123"
  - "Failed to begin transaction"

### Validation Errors
- Be specific about what's invalid
- Include limits when applicable
- Examples:
  - "Conversation title cannot be empty"
  - "Title exceeds maximum length of 200 characters"
  - "Email format is invalid"
  - "API key contains invalid characters"

### I/O Errors
- Specify what operation failed
- Include file path when safe to do so
- Examples:
  - "Failed to read file at /path/to/file"
  - "Failed to create directory"
  - "Access denied to system directory"

### API Errors
- Include service/provider name
- Mention specific operation
- Examples:
  - "Failed to connect to OpenAI API"
  - "Request to AI service timed out"
  - "Invalid API key for provider 'anthropic'"

### Keychain Errors
- Specify the operation (store, retrieve, delete)
- Include provider name when available
- Examples:
  - "Failed to store API key for provider 'openai'"
  - "No API key found for provider 'anthropic'"
  - "Failed to access system keychain"

## Error Message Components

### Required Components
1. **What went wrong**: The core issue
2. **Where**: Which resource or component

### Optional Components (when applicable)
3. **Why**: Root cause details
4. **How to fix**: Suggested action

### Examples

```rust
// Minimal (what + where)
"Failed to create conversation"

// With details (what + where + why)
format!("Failed to create conversation: {}", db_error)

// With context (what + where + which)
format!("Failed to update message with ID {}", message_id)

// Complete (what + where + why + suggestion)
"Failed to write file: Permission denied. Try running as administrator"
```

## Common Phrases

### Preferred Phrases
- "Failed to [action]"
- "[Resource] not found"
- "[Field] cannot be empty"
- "[Field] is required"
- "[Field] exceeds maximum length"
- "[Field] contains invalid characters"
- "Access denied to [resource]"
- "Invalid [format/type]"

### Avoid These Phrases
- "Error occurred" (too vague)
- "Something went wrong" (not helpful)
- "Cannot do X" (use "Failed to do X")
- "You must..." (too prescriptive, use "X is required")
- "Please..." (error messages, not instructions)

## Checklist

Before committing error message changes, verify:

- [ ] Message starts with a capital letter
- [ ] No period at the end (unless multiple sentences)
- [ ] Specific about what failed
- [ ] Includes resource/identifier when available
- [ ] Uses consistent phrasing from this guide
- [ ] Appropriate error category (Database, Validation, etc.)
- [ ] User-actionable when possible

## Examples by Module

### Validation Messages
```rust
"Conversation title cannot be empty"
"Title exceeds maximum length of 200 characters"
"Title contains invalid characters"
"Email format is invalid"
"URL must use HTTP or HTTPS protocol"
"File extension '.exe' is not allowed"
"Path traversal is not allowed"
```

### Database Messages
```rust
"Failed to create conversation"
"Failed to insert message"
"Failed to begin transaction"
"Failed to commit transaction"
format!("Conversation with ID {} not found", id)
format!("Failed to update persona with ID {}", id)
```

### Keychain Messages
```rust
"Provider name cannot be empty"
"API key cannot be empty"
"Failed to store API key"
"Failed to create keychain entry"
format!("No API key found for provider: {}", provider)
format!("Failed to retrieve API key: {}", error)
```
