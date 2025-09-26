# GitHub Coding Agent Implementation Prompt
# Systematic Implementation of CR_ActPlan.md for Forbidden Library

## CRITICAL IMPLEMENTATION PROTOCOL

**MANDATORY WORKFLOW:** You MUST follow this exact sequence without deviation. Each phase requires completion verification before proceeding to the next.

### PHASE COMPLETION REQUIREMENTS
- ✅ All code changes implemented and compiling
- ✅ All tests passing (existing + new)
- ✅ Lint checks passing (cargo clippy, ESLint)
- ✅ Documentation updated for changes
- ✅ Performance benchmarks stable or improved
- ✅ Integration tests confirming functionality

**DO NOT PROCEED** to the next phase until current phase meets ALL completion criteria.

---

## PHASE 1: FOUNDATION - ERROR HANDLING & DATABASE ARCHITECTURE
**Priority: CRITICAL - Must complete before any other work**

### 1.1 Implement Consistent Error Handling System

**Step 1.1.1: Create AppError Infrastructure**
```rust
// Create: src-tauri/src/errors.rs
// Implement comprehensive AppError enum using thiserror crate
// Required variants: Database, Io, Validation, NotFound, Api, Encryption, Unexpected
// Include From implementations for String, &str, common error types
// Create AppResult<T> type alias
```

**Step 1.1.2: Update Cargo Dependencies**
```toml
// Add to src-tauri/Cargo.toml:
// thiserror = "1.0"
// Ensure compatibility with existing dependencies
```

**Step 1.1.3: Refactor All Backend Methods**
- Replace all `SqliteResult<T>` returns with `AppResult<T>`
- Replace all `Result<T, String>` returns with `AppResult<T>`
- Update service methods in conversations, personas, messages modules
- Ensure proper error conversion and propagation

**Step 1.1.4: Update Tauri Command Handlers**
- Modify all `#[tauri::command]` functions to handle AppError properly
- Implement error categorization (Validation -> user-friendly, Database -> technical)
- Add proper logging for different error types
- Maintain Result<T, String> return for Tauri compatibility

**PHASE 1.1 TESTING CHECKPOINT:**
```bash
# Must pass ALL before proceeding:
cargo check                    # ✅ No compilation errors
cargo clippy -- -D warnings   # ✅ No clippy warnings
cargo test                     # ✅ All existing tests pass
# Manual verification: Error messages are user-friendly in UI
```

### 1.2 Refactor Database Layer for Concurrency

**Step 1.2.1: Add Connection Pooling Dependencies**
```toml
// Add to src-tauri/Cargo.toml:
// r2d2 = "0.8"
// r2d2_sqlite = "0.21"
```

**Step 1.2.2: Create DatabaseConfig Structure**
```rust
// Update: src-tauri/src/database/mod.rs
// Create DatabaseConfig struct with encryption_enabled, pragma_settings, in_memory fields
// Implement Default trait with current pragma settings
// Add validation for configuration parameters
```

**Step 1.2.3: Implement Connection Pool**
```rust
// Replace Mutex<Connection> with r2d2::Pool<SqliteConnectionManager>
// Create new_with_config method that accepts DatabaseConfig
// Implement get_connection() method returning pooled connections
// Update initialize_schema to use pooled connections
// Maintain backward compatibility with existing new() and new_in_memory() methods
```

**Step 1.2.4: Update All Database Access Points**
```rust
// Replace all `self.db.connection().lock().unwrap()` calls
// Use `self.db.get_connection()?` instead
// Update all service methods: conversations, personas, messages
// Ensure proper error handling for connection failures
```

**PHASE 1.2 TESTING CHECKPOINT:**
```bash
cargo check                    # ✅ No compilation errors
cargo clippy -- -D warnings   # ✅ No clippy warnings
cargo test                     # ✅ All tests pass
cargo tauri dev               # ✅ App launches and functions normally
# Manual verification: Database operations work with concurrent access
# Performance test: Multiple simultaneous database operations complete successfully
```

### 1.3 Implement Repository Pattern and Dependency Injection

**Step 1.3.1: Create Repository Structure**
```rust
// Create: src-tauri/src/repositories/mod.rs
// Create: src-tauri/src/repositories/conversation_repository.rs
// Create: src-tauri/src/repositories/message_repository.rs
// Create: src-tauri/src/repositories/persona_repository.rs
```

**Step 1.3.2: Implement ConversationRepository**
```rust
// Move database access logic from ConversationService to ConversationRepository
// Methods: create, find_by_id, find_all_paginated, update, delete
// Use AppResult<T> for all return types
// Use connection pool for all database access
```

**Step 1.3.3: Implement MessageRepository**
```rust
// Move database access logic from MessageService to MessageRepository
// Methods: create, find_by_conversation_id, find_by_id, update, delete
// Implement batch operations for bulk inserts
```

**Step 1.3.4: Implement PersonaRepository**
```rust
// Move database access logic from PersonaService to PersonaRepository
// Methods: create, find_by_id, find_all, update, delete
```

**Step 1.3.5: Create Service Provider**
```rust
// Create: src-tauri/src/services/provider.rs
// Implement lazy initialization for all services
// Use Arc<T> for shared services
// Implement proper lifetime management
```

**Step 1.3.6: Update Services to Use Repositories**
```rust
// Refactor ConversationService to use ConversationRepository
// Refactor MessageService to use MessageRepository
// Refactor PersonaService to use PersonaRepository
// Focus services on business logic only
// Remove direct database access from services
```

**Step 1.3.7: Update Main Application State**
```rust
// Update AppState to use individual services instead of services container
// Update main.rs initialization to use ServiceProvider
// Update all command handlers to use new service structure
```

**PHASE 1.3 TESTING CHECKPOINT:**
```bash
cargo check                    # ✅ No compilation errors
cargo clippy -- -D warnings   # ✅ No clippy warnings
cargo test                     # ✅ All tests pass + new repository tests
cargo tauri dev               # ✅ App launches and all features work
# Manual verification: All CRUD operations work through new architecture
# Integration test: Create conversation, add messages, verify data integrity
```

**PHASE 1 COMPLETION VERIFICATION:**
- ✅ All error handling uses AppError enum consistently
- ✅ Database uses connection pooling (no more Mutex<Connection>)
- ✅ Repository pattern separates data access from business logic
- ✅ Services use dependency injection
- ✅ All existing functionality preserved
- ✅ Performance equal or better than before
- ✅ No memory leaks or connection leaks

---

## PHASE 2: PERFORMANCE & SCALABILITY OPTIMIZATIONS
**Prerequisites: Phase 1 must be 100% complete**

### 2.1 Backend Query and Transaction Optimization

**Step 2.1.1: Implement Prepared Statement Caching**
```rust
// Add StatementCache struct to DatabaseManager
// Cache frequently used statements: get_conversation, get_messages, etc.
// Implement statement lifecycle management
// Add cache hit/miss metrics
```

**Step 2.1.2: Implement Batch Operations**
```rust
// Update MessageRepository with batch insert methods
// Use transactions for bulk operations
// Implement batch update and delete operations
// Add performance benchmarks for batch vs individual operations
```

**Step 2.1.3: Add RwLock for Read-Heavy Operations**
```rust
// Identify read-heavy operations (get_conversations, get_messages)
// Implement RwLock where connection pooling isn't sufficient
// Benchmark performance improvements
```

**Step 2.1.4: Optimize VACUUM Operations**
```rust
// Create fragmentation detection methods
// Implement conditional VACUUM based on metrics
// Add scheduling for maintenance operations
// Add configuration for VACUUM thresholds
```

**PHASE 2.1 TESTING CHECKPOINT:**
```bash
cargo bench                    # ✅ Performance benchmarks pass
cargo test                     # ✅ All tests pass
# Performance verification: Batch operations >50% faster than individual
# Memory verification: No memory leaks during bulk operations
```

### 2.2 IPC and API Efficiency

**Step 2.2.1: Consolidate IPC Commands**
```rust
// Create get_conversation_with_messages command
// Create get_dashboard_data command (conversations + personas)
// Implement batch operations for related data
// Remove or deprecate fine-grained commands
```

**Step 2.2.2: Implement Streaming for Large Responses**
```rust
// Add streaming support for large message lists
// Implement pagination for all list endpoints
// Add streaming for file operations if applicable
```

**Step 2.2.3: Optimize Serialization**
```rust
// Review struct designs for serialization efficiency
// Implement custom serialization where beneficial
// Add compression for large responses
// Benchmark serialization performance
```

**PHASE 2.2 TESTING CHECKPOINT:**
```bash
cargo test                     # ✅ All tests pass
# IPC performance: Consolidated commands >30% faster
# Memory usage: Large responses don't cause UI freezing
# Streaming: Large datasets load progressively
```

### 2.3 Frontend API and State Management

**Step 2.3.1: Create Typed API Client**
```typescript
// Create: src/lib/services/api-client.ts
// Implement ApiClient class with singleton pattern
// Add typed responses, error handling, timeout management
// Implement retry logic with exponential backoff
```

**Step 2.3.2: Implement Request Batching and Caching**
```typescript
// Add request batching for related operations
// Implement TTL-based response caching
// Add cache invalidation strategies
// Create cache management utilities
```

**Step 2.3.3: Implement Client-Side Pagination**
```typescript
// Update conversation list with pagination
// Update message list with pagination
// Add virtualized scrolling for large lists
// Implement progressive loading indicators
```

**PHASE 2.3 TESTING CHECKPOINT:**
```bash
pnpm run test                  # ✅ All frontend tests pass
pnpm run build                 # ✅ Build succeeds
# Manual verification: API client handles errors gracefully
# Performance: Large lists render smoothly with virtualization
# Caching: Repeated requests served from cache
```

**PHASE 2 COMPLETION VERIFICATION:**
- ✅ Database queries use prepared statement cache
- ✅ Batch operations perform significantly better
- ✅ IPC commands consolidated for related operations
- ✅ Frontend uses typed API client with caching
- ✅ Large datasets handled efficiently
- ✅ No performance regressions in existing features

---

## PHASE 3: DATA INTEGRITY & VALIDATION
**Prerequisites: Phases 1-2 must be 100% complete**

### 3.1 Input Validation System

**Step 3.1.1: Create Validation Module**
```rust
// Create: src-tauri/src/validation/mod.rs
// Implement validation functions for all input types
// validate_conversation_title, validate_message_content, validate_persona_name
// Add comprehensive validation rules and error messages
```

**Step 3.1.2: Integrate Validation in Command Handlers**
```rust
// Update all Tauri command handlers to use validation
// Ensure validation happens before business logic
// Return user-friendly validation error messages
// Add validation for edge cases and security concerns
```

**Step 3.1.3: Add Frontend Validation**
```typescript
// Mirror backend validation rules in frontend
// Add real-time validation feedback
// Implement form validation utilities
// Ensure consistent validation messages
```

**PHASE 3.1 TESTING CHECKPOINT:**
```bash
cargo test                     # ✅ All tests pass + new validation tests
pnpm run test                  # ✅ Frontend tests pass
# Security test: Invalid inputs rejected consistently
# UX test: Validation errors are user-friendly
```

### 3.2 Pagination and List Operations

**Step 3.2.1: Standardize Pagination**
```rust
// Create PaginationParams and PaginatedResponse structs
// Update all list operations to use pagination
// Implement consistent pagination across all endpoints
// Add total count and page information
```

**Step 3.2.2: Update Frontend Pagination**
```typescript
// Update all list components to use pagination
// Add pagination controls with total counts
// Implement efficient page navigation
// Add loading states for pagination
```

**PHASE 3.2 TESTING CHECKPOINT:**
```bash
cargo test                     # ✅ All tests pass
pnpm run test                  # ✅ Frontend tests pass
# Manual verification: All lists support pagination
# Performance: Large datasets load quickly with pagination
```

**PHASE 3 COMPLETION VERIFICATION:**
- ✅ All user inputs validated consistently
- ✅ Validation errors are user-friendly
- ✅ Pagination implemented across all list operations
- ✅ Data integrity maintained throughout the application
- ✅ No security vulnerabilities in input handling

---

## PHASE 4: STARTUP & RESOURCE OPTIMIZATION
**Prerequisites: Phases 1-3 must be 100% complete**

### 4.1 Startup Performance

**Step 4.1.1: Implement Lazy Initialization**
```rust
// Create LazyService wrapper for non-critical services
// Defer initialization of heavy services until first use
// Implement service loading prioritization
// Add startup performance metrics
```

**Step 4.1.2: Optimize Asset Loading**
```typescript
// Defer non-critical asset loading
// Implement progressive asset loading
// Optimize bundle size and loading order
// Add loading progress indicators
```

**PHASE 4.1 TESTING CHECKPOINT:**
```bash
# Startup time: Must be <1 second on target hardware
# Memory usage: Minimal memory footprint at startup
# UX: Application feels responsive immediately
```

### 4.2 Memory and Asset Management

**Step 4.2.1: Implement Memory Monitoring**
```rust
// Add memory usage tracking in development
// Implement leak detection utilities
// Add memory usage alerts for development
// Create memory profiling tools
```

**Step 4.2.2: Optimize Resource Usage**
```typescript
// Implement efficient asset cleanup
// Add memory-aware caching strategies
// Optimize component lifecycle management
// Add resource usage monitoring
```

**PHASE 4.2 TESTING CHECKPOINT:**
```bash
# Memory: No memory leaks detected in long-running tests
# Resources: Efficient cleanup of unused resources
# Performance: Stable memory usage during extended use
```

**PHASE 4 COMPLETION VERIFICATION:**
- ✅ Application startup time <1 second
- ✅ Lazy initialization working for non-critical services
- ✅ Memory usage optimized and stable
- ✅ No memory leaks detected
- ✅ Resource cleanup functioning properly

---

## PHASE 5: MONITORING & QUALITY GATES
**Prerequisites: Phases 1-4 must be 100% complete**

### 5.1 Benchmark Coverage and Monitoring

**Step 5.1.1: Expand Benchmark Coverage**
```rust
// Add benchmarks for all critical user paths
// Implement performance regression detection
// Add benchmark CI integration
// Create performance baseline documentation
```

**Step 5.1.2: Implement Real User Monitoring**
```rust
// Add opt-in telemetry for performance insights
// Implement performance metrics collection
// Add performance dashboard for development
// Create performance alerting system
```

**PHASE 5.1 TESTING CHECKPOINT:**
```bash
cargo bench                    # ✅ All benchmarks pass baseline
# Performance: All critical paths meet performance targets
# Monitoring: Telemetry data collection working
```

### 5.2 Logging and Tracing Optimization

**Step 5.2.1: Optimize Production Logging**
```rust
// Implement conditional compilation for tracing
// Reduce logging overhead in release builds
// Streamline error logging and event tracking
// Add structured logging for better analysis
```

**PHASE 5.2 TESTING CHECKPOINT:**
```bash
cargo build --release         # ✅ Release build optimized
# Performance: Logging overhead minimized in production
# Debugging: Development logging still comprehensive
```

**PHASE 5 COMPLETION VERIFICATION:**
- ✅ Comprehensive benchmark coverage implemented
- ✅ Performance monitoring and alerting active
- ✅ Production logging optimized
- ✅ All performance targets met
- ✅ Quality gates passing consistently

---

## PHASE 6: UI/UX RESPONSIVENESS & FINAL OPTIMIZATIONS
**Prerequisites: Phases 1-5 must be 100% complete**

### 6.1 Frontend Rendering Optimization

**Step 6.1.1: Implement Virtualized Lists**
```typescript
// Add virtualization for conversation lists
// Add virtualization for message lists
// Implement efficient scrolling and rendering
// Add placeholder loading states
```

**Step 6.1.2: Component Lazy Loading**
```typescript
// Implement lazy loading for non-critical components
// Add code splitting for route-based components
// Optimize component loading strategies
// Add loading progress indicators
```

**Step 6.1.3: Optimize Reactivity**
```typescript
// Implement derived stores for computed values
// Add memoization for expensive calculations
// Optimize store subscription patterns
// Reduce unnecessary re-renders
```

**PHASE 6.1 TESTING CHECKPOINT:**
```bash
pnpm run test                  # ✅ All tests pass
pnpm run build                 # ✅ Build optimization successful
# Performance: 60 FPS maintained with large datasets
# UX: Smooth scrolling and responsive interactions
```

### 6.2 Final Performance Optimization

**Step 6.2.1: Parallel Operations**
```rust
// Implement tokio::join! for independent operations
// Add parallel processing where beneficial
// Optimize async operation scheduling
// Add concurrency performance metrics
```

**Step 6.2.2: Error Handling Path Optimization**
```rust
// Streamline error handling overhead
// Reduce allocations in error paths
// Optimize error propagation performance
// Add error handling performance tests
```

**PHASE 6.2 TESTING CHECKPOINT:**
```bash
cargo test                     # ✅ All tests pass
cargo bench                    # ✅ Performance targets exceeded
# Parallel operations: Significant performance improvement
# Error handling: No performance impact on happy path
```

**PHASE 6 COMPLETION VERIFICATION:**
- ✅ UI maintains 60 FPS with large datasets
- ✅ Virtualized lists working efficiently
- ✅ Component lazy loading implemented
- ✅ Parallel operations optimized
- ✅ Error handling paths optimized
- ✅ All performance goals achieved

---

## FINAL VALIDATION & TESTING PROTOCOL

### Comprehensive Integration Testing
```bash
# Backend Testing
cargo check                    # ✅ No compilation errors
cargo clippy -- -D warnings   # ✅ No warnings
cargo test                     # ✅ All tests pass
cargo bench                    # ✅ Performance benchmarks pass

# Frontend Testing  
pnpm run check                 # ✅ TypeScript checks pass
pnpm run lint                  # ✅ ESLint passes
pnpm run test                  # ✅ All tests pass
pnpm run build                 # ✅ Production build succeeds

# Integration Testing
cargo tauri dev               # ✅ Development build works
cargo tauri build             # ✅ Production build works
```

### Performance Validation
- ✅ Startup time: <1 second
- ✅ UI responsiveness: 60 FPS maintained
- ✅ Memory usage: Stable and optimized
- ✅ Database operations: Optimized with pooling
- ✅ Large datasets: Handled efficiently

### Security & Quality Validation
- ✅ All inputs properly validated
- ✅ Database encrypted and secure
- ✅ Error handling doesn't leak information
- ✅ Code coverage >80%
- ✅ Documentation updated

### User Experience Validation
- ✅ All existing features working
- ✅ Error messages user-friendly
- ✅ Loading states and progress indicators
- ✅ Smooth interactions and transitions
- ✅ Responsive design maintained

## DELIVERY REQUIREMENTS

Upon completion, provide:
1. **Summary Report**: What was implemented, performance improvements achieved
2. **Migration Guide**: Any breaking changes and how to handle them
3. **Performance Metrics**: Before/after benchmarks
4. **Test Coverage Report**: Coverage statistics and test results
5. **Documentation Updates**: All documentation updated to reflect changes

**CRITICAL**: Do not mark any phase as complete until ALL verification criteria are met. Each phase builds on the previous one, so incomplete implementation will cause cascading failures.

---

*This implementation plan ensures systematic, tested delivery of all CR_ActPlan optimizations with minimal rework required.*
