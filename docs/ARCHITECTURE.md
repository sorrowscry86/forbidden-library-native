# Forbidden Library Architecture Overview

This document provides an overview of the codebase architecture, explaining key design decisions, code organization, and when to use different versions of modules.

## Table of Contents
- [Project Structure](#project-structure)
- [Backend Architecture (Rust/Tauri)](#backend-architecture-rusttauri)
- [Frontend Architecture (SvelteKit)](#frontend-architecture-sveltekit)
- [Module Versioning Strategy](#module-versioning-strategy)
- [Security Architecture](#security-architecture)
- [Testing Strategy](#testing-strategy)

---

## Project Structure

```
forbidden-library-native/
├── src/                    # Frontend (SvelteKit)
│   ├── lib/
│   │   ├── components/    # Svelte components
│   │   ├── services/      # API services (basic & enhanced)
│   │   ├── stores/        # State management (basic & enhanced)
│   │   ├── types/         # TypeScript type definitions
│   │   └── utils/         # Utility functions
│   ├── routes/            # SvelteKit routes
│   └── test/              # Test configuration
├── src-tauri/             # Backend (Rust)
│   ├── src/
│   │   ├── commands.rs    # Tauri command handlers
│   │   ├── database/      # Database layer (SQLite + SQLCipher)
│   │   ├── keychain.rs    # OS keychain integration
│   │   ├── services/      # Business logic
│   │   ├── validation.rs  # Input validation
│   │   └── errors.rs      # Error handling
│   └── tests/             # Rust tests
└── docs/                  # Documentation

```

---

## Backend Architecture (Rust/Tauri)

### Core Modules

#### Database Layer (`database/mod.rs`)
- **Connection Pooling**: r2d2 connection pool for concurrent access
- **Encryption**: SQLCipher support for encrypted at-rest storage
- **Transactions**: Comprehensive transaction support with automatic rollback
- **Schema Management**: Automated schema creation and migrations

**Key Methods:**
- `with_transaction<T, F>(f: F) -> AppResult<T>` - Execute operations atomically
- `with_savepoint<T, F>` - Nested transactions for complex workflows
- `get_connection()` - Get a pooled database connection

#### Keychain Integration (`keychain.rs`)
- **Cross-platform**: macOS Keychain, Windows Credential Manager, Linux Secret Service
- **Secure Storage**: API keys never written to disk in plain text
- **Simple API**: Store, retrieve, update, delete operations

**Key Methods:**
- `store_api_key(provider_name, api_key)` - Securely store API key
- `get_api_key(provider_name)` - Retrieve API key from keychain
- `delete_api_key(provider_name)` - Remove API key

#### Error Handling (`errors.rs`)
- **Unified Error Type**: `AppError` enum with all error categories
- **User-Friendly Messages**: Automatic conversion to user-readable text
- **Platform-Specific Suggestions**: Context-aware error guidance

**Error Categories:**
- Database, I/O, Validation, NotFound, API, Encryption, Keychain, Unexpected

#### Command Handlers (`commands.rs`)
- **Input Validation**: All commands validate inputs using `validation.rs`
- **Error Propagation**: Consistent error handling across all commands
- **Type Safety**: Strong typing for all parameters and return values

---

## Frontend Architecture (SvelteKit)

### Module Versioning Strategy

The frontend uses a **dual-version approach** with "basic" and "enhanced" versions of key modules:

#### Why Two Versions?

1. **Progressive Enhancement**: Basic versions work everywhere, enhanced versions add features when available
2. **Performance**: Basic versions have minimal overhead for simple use cases
3. **Compatibility**: Basic versions provide fallbacks for web-only environments
4. **Testing**: Easier to test core functionality in isolation

### Error Stores

#### Basic Error Store (`error-store.ts`)
**Use When:**
- Simple error tracking is sufficient
- Minimal memory footprint required
- Basic error dismissal and filtering needed

**Features:**
- Error storage and retrieval
- Auto-cleanup of old errors
- Category and severity filtering
- Convenience functions (addTimeoutError, addNetworkError, etc.)

#### Enhanced Error Store (`enhanced-error-store.ts`)
**Use When:**
- Advanced error analytics needed
- Recovery strategies required
- System health monitoring desired
- Production environments

**Additional Features:**
- Error rate tracking (errors per minute)
- Recovery rate calculation
- System health status (healthy/degraded/unhealthy)
- Retry tracking and metrics
- Circuit breaker patterns
- Error pattern detection
- Comprehensive analytics export

**Test Coverage:** 34 tests, 91.17% coverage

### API Services

#### Basic API Service (`api.ts`)
**Use When:**
- Simple Tauri command invocation needed
- Timeout handling is primary concern
- Retry logic for basic operations

**Features:**
- `invokeWithTimeout()` - Command with timeout
- `invokeWithRetry()` - Command with retry logic
- `invokeWithValidation()` - Command with input validation

#### Enhanced API Service (`enhanced-api.ts`)
**Use When:**
- Circuit breaker patterns needed
- Request queuing required
- Advanced caching desired
- Production environments with high reliability needs

**Additional Features:**
- Circuit breaker with configurable thresholds
- Request deduplication
- Response caching with TTL
- Command prioritization
- Comprehensive metrics
- Health check endpoints

### Error Notification Components

#### ErrorNotification.svelte
- **Simple UI**: Basic error display with auto-dismiss
- **Auto-Cleanup**: Proper timeout management
- **Severity Indicators**: Color-coded by severity
- **186 lines**: Lightweight and fast

#### EnhancedErrorNotification.svelte
- **Advanced UI**: System health indicators, analytics
- **Recovery Suggestions**: Context-aware help
- **Pattern Detection**: Alerts for cascading failures
- **380+ lines**: Feature-rich with advanced diagnostics

### Page Components

#### +page.svelte (Basic)
- **Use**: Simple conversation management
- **229 lines**: Minimal, focused interface
- **Dependencies**: Basic error store, basic API service

#### enhanced-page.svelte (Enhanced)
- **Use**: Production deployment with monitoring
- **524 lines**: Comprehensive error tracking, health checks
- **Dependencies**: Enhanced error store, enhanced API service

---

## Security Architecture

### Defense in Depth

1. **Input Validation** (`validation.rs`)
   - SQL injection prevention via parameterized queries
   - Path traversal prevention via path normalization
   - XSS prevention via content sanitization
   - Command injection prevention via validation rules

2. **Secure Storage**
   - **Database Encryption**: SQLCipher for at-rest encryption
   - **Keychain Integration**: OS-level credential protection
   - **No Plain-text Secrets**: API keys never stored in database

3. **Type Safety**
   - **Rust**: Memory safety, no null pointers, no data races
   - **TypeScript**: Static typing with minimal `any` usage
   - **Strong Interfaces**: Type definitions for all API boundaries

4. **Error Handling**
   - **No Information Leakage**: User-friendly messages hide technical details
   - **Logging**: Technical details logged for debugging
   - **Recovery**: Automatic retry with backoff for transient failures

---

## Testing Strategy

### Backend Tests (Rust)
- **Unit Tests**: All modules have comprehensive unit tests
- **Transaction Tests**: 9 tests covering commit, rollback, savepoints
- **Keychain Tests**: 8 tests covering all operations
- **Integration Tests**: Cross-module interaction tests

**Run tests:**
```bash
cd src-tauri && cargo test
```

### Frontend Tests (TypeScript/Svelte)
- **Component Tests**: Svelte Testing Library
- **Store Tests**: Vitest with state management
- **Type Tests**: Error type definitions

**Coverage:**
- Overall: 90.34% statement coverage, 93.65% line coverage
- Components: 76.74%
- Stores: 91.17%
- Types: 100%

**Run tests:**
```bash
pnpm test                  # Run all tests
pnpm test -- --coverage    # With coverage report
```

---

## Migration Guide

### From Basic to Enhanced

When migrating from basic to enhanced versions:

1. **Update Imports**
   ```typescript
   // Before
   import { errorStore } from '$lib/stores/error-store';

   // After
   import { errorStore } from '$lib/stores/enhanced-error-store';
   ```

2. **Initialize Enhanced Features**
   ```typescript
   onMount(() => {
     // Initialize enhanced error store with cleanup
     const cleanup = errorStore.init(200, 7200000); // max errors, max age ms

     // Cleanup on unmount
     onDestroy(cleanup);
   });
   ```

3. **Use Enhanced Features**
   ```typescript
   // Access analytics
   const analytics = errorStore.getAnalytics();
   console.log(`Error rate: ${analytics.errorRate}/min`);
   console.log(`Recovery rate: ${analytics.recoveryRate}%`);

   // Check system health
   const health = errorStore.getSystemHealth();
   if (health !== 'healthy') {
     // Show warning to user
   }

   // Detect patterns
   const patterns = detectErrorPatterns();
   if (patterns.cascadingFailures) {
     // Alert operations team
   }
   ```

---

## Best Practices

### When to Use Basic vs Enhanced

**Use Basic Versions When:**
- Building prototypes or demos
- Simple applications with minimal error handling needs
- Memory constraints are critical
- Learning the codebase

**Use Enhanced Versions When:**
- Building production applications
- Advanced monitoring and analytics needed
- High-reliability requirements
- User experience is critical

### Error Handling

1. **Always handle errors gracefully**
   ```typescript
   try {
     await invokeWithTimeout('command', args, ms(10));
   } catch (error) {
     errorStore.addError({
       message: 'Operation failed',
       details: error instanceof Error ? error.message : String(error),
       category: ErrorCategory.API,
       severity: ErrorSeverity.ERROR
     });
   }
   ```

2. **Use appropriate error categories**
   - API: External API failures
   - NETWORK: Connection issues
   - TIMEOUT: Operation timeouts
   - VALIDATION: Invalid user input
   - DATA: Data integrity issues
   - PERMISSION: Access denied
   - ENVIRONMENT: Feature not available

3. **Provide recovery guidance**
   - Include actionable error messages
   - Suggest next steps when possible
   - Log technical details for debugging

---

## Performance Considerations

### Database
- Use transactions for multi-step operations
- Use connection pooling (configured for 10 max connections)
- Add indexes for frequently queried fields (9 indexes already added)
- Enable WAL mode for better concurrency

### Frontend
- Lazy load components when possible
- Use reactive statements judiciously (avoid double-loading)
- Clean up subscriptions in `onDestroy`
- Debounce expensive operations

### Memory Management
- Error stores auto-cleanup old errors
- Timeout cleanup in all components
- Connection pooling prevents connection leaks
- Proper resource disposal in `Drop` implementations

---

## Additional Resources

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [SvelteKit Documentation](https://kit.svelte.dev/docs)
- [SQLite Best Practices](https://www.sqlite.org/bestpractice.html)
- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

---

## Contributing

When adding new features:

1. **Add tests first** (TDD approach)
2. **Document public APIs** with JSDoc/rustdoc
3. **Update this architecture guide** for significant changes
4. **Run all tests** before committing
5. **Follow existing patterns** (basic vs enhanced separation)

---

**Last Updated:** 2025-11-17
**Version:** 2.0.0
