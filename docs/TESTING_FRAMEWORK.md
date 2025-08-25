# Testing Framework Documentation

## Overview

The Forbidden Library implements a comprehensive testing framework designed to ensure the highest quality standards for a production-ready native desktop application. This framework validates performance, security, and functionality across all aspects of the application.

## Testing Philosophy

Our testing approach is guided by the **VoidCat RDC Excellence Protocol**:

- **Performance as Law**: Every test validates the sub-second startup and 60 FPS UI mandates
- **Security by Default**: All functionality is tested against common attack vectors
- **Comprehensive Coverage**: Minimum 80% code coverage with real-world scenarios
- **Continuous Validation**: Automated testing on every commit and pull request

## Test Categories

### 1. Unit Tests (`src-tauri/src/**/tests/`)

**Purpose**: Test individual functions and methods in isolation

**Coverage**:
- Command handlers (IPC layer)
- Service layer operations
- Database operations
- Model validation
- Error handling

**Example**:
```rust
#[tokio::test]
async fn test_create_conversation() {
    let (app_state, _temp_dir) = setup_test_environment().await;
    
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
```

### 2. Integration Tests (`src-tauri/tests/integration_tests.rs`)

**Purpose**: Test complete workflows and cross-module interactions

**Coverage**:
- Complete conversation lifecycle
- Persona management workflow
- API configuration workflow
- Data persistence and recovery
- Concurrent operations
- Export and backup functionality

**Example**:
```rust
#[tokio::test]
async fn test_conversation_lifecycle() {
    let env = IntegrationTestEnvironment::new();
    
    // 1. Create conversation
    let conversation = create_conversation(
        "Integration Test Conversation".to_string(),
        None,
        State::new(&env.app_state)
    ).await.unwrap();
    
    // 2. Add messages
    let user_message = add_message(
        conversation.id.unwrap(),
        "user".to_string(),
        "Hello, this is a test message".to_string(),
        None,
        State::new(&env.app_state)
    ).await.unwrap();
    
    // 3. Verify complete workflow
    assert_eq!(user_message.role, MessageRole::User);
    assert_eq!(user_message.content, "Hello, this is a test message");
}
```

### 3. Security Tests (`src-tauri/tests/security_audit.rs`)

**Purpose**: Validate security measures and prevent common vulnerabilities

**Coverage**:
- SQL injection prevention
- Path traversal prevention
- Command injection prevention
- XSS prevention
- Input validation
- Error handling security
- Concurrent access security
- Data integrity under malicious input

**Example**:
```rust
#[tokio::test]
async fn test_sql_injection_prevention_conversations() {
    let env = SecurityTestEnvironment::new();
    
    let malicious_inputs = vec![
        "'; DROP TABLE conversations; --",
        "' OR '1'='1",
        "'; INSERT INTO conversations VALUES (999, 'hacked', 1, '2023-01-01', '2023-01-01', 'false'); --",
    ];
    
    for malicious_input in malicious_inputs {
        let result = create_conversation(
            malicious_input.to_string(),
            None,
            State::new(&env.app_state)
        ).await;
        
        // Should succeed but not cause SQL injection
        assert!(result.is_ok());
        
        let conversation = result.unwrap();
        assert_eq!(conversation.title, malicious_input);
        
        // Verify table structure is intact
        let conversations = get_conversations(None, None, State::new(&env.app_state)).await.unwrap();
        assert!(!conversations.is_empty());
    }
}
```

### 4. Performance Benchmarks (`src-tauri/benches/performance_benchmarks.rs`)

**Purpose**: Validate performance requirements and identify bottlenecks

**Coverage**:
- Sub-second startup validation
- 60 FPS UI responsiveness
- Memory efficiency
- Database operation performance
- Concurrent operation performance
- Large dataset handling

**Example**:
```rust
fn benchmark_conversation_creation(c: &mut Criterion) {
    let services = setup_benchmark_environment();
    
    c.bench_function("conversation_creation", |b| {
        b.iter(|| {
            let service = &services.conversations;
            black_box(service.create_conversation(
                "Benchmark Conversation".to_string(),
                None
            ).unwrap());
        });
    });
}

fn validate_performance_requirements() {
    // Test 1: Sub-second startup simulation
    let startup_start = Instant::now();
    let db_manager = DatabaseManager::new_in_memory().unwrap();
    let services = Services::new(Arc::new(db_manager));
    let startup_duration = startup_start.elapsed();
    
    assert!(
        startup_duration.as_millis() < 1000,
        "Startup time exceeds 1 second: {:?}",
        startup_duration
    );
    
    // Test 2: 60 FPS equivalent operations (16.67ms per operation)
    let fps_start = Instant::now();
    for _ in 0..60 {
        services.conversations.create_conversation(
            "FPS Test".to_string(),
            None
        ).unwrap();
    }
    let fps_duration = fps_start.elapsed();
    let avg_operation_time = fps_duration.as_millis() / 60;
    
    assert!(
        avg_operation_time < 16,
        "Operation time exceeds 60 FPS target: {}ms",
        avg_operation_time
    );
}
```

## Running Tests

### Prerequisites

```bash
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install testing dependencies
cargo install cargo-audit
cargo install cargo-criterion
cargo install cargo-llvm-cov
```

### Running All Tests

```bash
# Run all tests with coverage
cargo test --all-features

# Run with verbose output
cargo test --all-features -- --nocapture

# Run specific test categories
cargo test --test integration_tests --all-features
cargo test --test security_audit --all-features
```

### Running Performance Benchmarks

```bash
# Run all benchmarks
cargo criterion

# Run specific benchmarks
cargo criterion --bench performance_benchmarks

# Generate benchmark reports
cargo criterion --message-format=json > benchmark-results.json
```

### Running Security Audits

```bash
# Run security tests
cargo test --test security_audit --all-features -- --nocapture

# Run cargo audit
cargo audit --deny warnings

# Run with additional security checks
cargo clippy --all-targets --all-features -- -D warnings
```

### Code Coverage

```bash
# Generate coverage report
cargo llvm-cov --all-features --workspace --lcov --output-path lcov.info

# View coverage in browser
cargo llvm-cov --all-features --workspace --html
```

## Continuous Integration

The testing framework is integrated into GitHub Actions with the following workflow:

### Test Matrix
- **Operating Systems**: Ubuntu, Windows, macOS
- **Rust Versions**: Stable, 1.70
- **Test Types**: Unit, Integration, Security, Performance, Coverage

### Automated Checks
1. **Code Quality**: `cargo fmt`, `cargo clippy`
2. **Security**: `cargo audit`, security tests
3. **Functionality**: Unit and integration tests
4. **Performance**: Benchmarks and validation
5. **Coverage**: Code coverage analysis
6. **Build**: Tauri application build test

### Quality Gates
- All tests must pass
- Code coverage must be ≥80%
- No security vulnerabilities
- Performance benchmarks must meet targets
- Build must succeed on all platforms

## Test Environment Setup

### Test Database

Tests use in-memory SQLite databases for isolation:

```rust
fn setup_test_environment() -> (ConversationService, TempDir) {
    let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
    let db_manager = DatabaseManager::new_in_memory()
        .expect("Failed to create test database");
    let service = ConversationService::new(Arc::new(db_manager));
    
    (service, temp_dir)
}
```

### Mock Services

For testing external dependencies:

```rust
#[cfg(test)]
mod tests {
    use mockall::predicate::*;
    use mockall::*;
    
    mock! {
        ApiClient {}
        
        impl ApiClient {
            pub async fn send_request(&self, message: String) -> Result<String, String>;
        }
    }
}
```

## Performance Validation

### Startup Time Validation

```rust
fn validate_startup_performance() {
    let start = Instant::now();
    
    // Simulate application startup
    let db_manager = DatabaseManager::new_in_memory().unwrap();
    let services = Services::new(Arc::new(db_manager));
    
    let duration = start.elapsed();
    assert!(
        duration.as_millis() < 1000,
        "Startup time exceeds 1 second: {:?}",
        duration
    );
}
```

### UI Responsiveness Validation

```rust
fn validate_ui_responsiveness() {
    let services = setup_benchmark_environment();
    
    // Simulate 60 FPS operations
    let start = Instant::now();
    for _ in 0..60 {
        services.conversations.create_conversation(
            "UI Test".to_string(),
            None
        ).unwrap();
    }
    let duration = start.elapsed();
    
    let avg_time = duration.as_millis() / 60;
    assert!(
        avg_time < 16,
        "Average operation time {}ms exceeds 60 FPS target (16.67ms)",
        avg_time
    );
}
```

## Security Testing Strategy

### Input Validation Testing

```rust
#[tokio::test]
async fn test_input_validation_conversation_titles() {
    let env = SecurityTestEnvironment::new();
    
    // Test extremely long titles
    let long_title = "a".repeat(10000);
    let result = create_conversation(
        long_title.clone(),
        None,
        State::new(&env.app_state)
    ).await;
    
    assert!(result.is_ok());
    
    // Test titles with null bytes
    let null_title = "test\0title";
    let result = create_conversation(
        null_title.to_string(),
        None,
        State::new(&env.app_state)
    ).await;
    
    assert!(result.is_ok());
}
```

### SQL Injection Prevention

```rust
#[tokio::test]
async fn test_sql_injection_prevention() {
    let env = SecurityTestEnvironment::new();
    
    let malicious_inputs = vec![
        "'; DROP TABLE conversations; --",
        "' OR '1'='1",
        "'; INSERT INTO conversations VALUES (999, 'hacked', 1, '2023-01-01', '2023-01-01', 'false'); --",
    ];
    
    for malicious_input in malicious_inputs {
        let result = create_conversation(
            malicious_input.to_string(),
            None,
            State::new(&env.app_state)
        ).await;
        
        // Should succeed but not cause SQL injection
        assert!(result.is_ok());
        
        // Verify table structure is intact
        let conversations = get_conversations(None, None, State::new(&env.app_state)).await.unwrap();
        assert!(!conversations.is_empty());
    }
}
```

## Best Practices

### Test Organization

1. **Arrange**: Set up test data and environment
2. **Act**: Execute the function being tested
3. **Assert**: Verify the expected outcome

### Test Isolation

- Each test should be independent
- Use unique test data
- Clean up after tests
- Use in-memory databases for isolation

### Performance Testing

- Test realistic scenarios
- Measure both average and worst-case performance
- Validate against specific targets (sub-second startup, 60 FPS)
- Use statistical analysis for benchmarks

### Security Testing

- Test all input validation
- Verify SQL injection prevention
- Test path traversal prevention
- Validate error handling security
- Test concurrent access scenarios

## Troubleshooting

### Common Issues

1. **Test Database Connection**: Ensure in-memory databases are used
2. **Async Test Issues**: Use `#[tokio::test]` for async tests
3. **Performance Test Failures**: Check system resources and background processes
4. **Security Test Failures**: Verify input validation and sanitization

### Debugging Tests

```bash
# Run specific test with output
cargo test test_name -- --nocapture

# Run tests with backtrace
RUST_BACKTRACE=1 cargo test

# Run tests with specific features
cargo test --features test-feature
```

## Contributing to Tests

### Adding New Tests

1. **Unit Tests**: Add to the appropriate module's `tests` module
2. **Integration Tests**: Add to `src-tauri/tests/integration_tests.rs`
3. **Security Tests**: Add to `src-tauri/tests/security_audit.rs`
4. **Performance Tests**: Add to `src-tauri/benches/performance_benchmarks.rs`

### Test Naming Convention

- Unit tests: `test_function_name`
- Integration tests: `test_workflow_name`
- Security tests: `test_security_vulnerability_prevention`
- Performance tests: `benchmark_operation_name`

### Test Documentation

All tests should include:
- Clear description of what is being tested
- Expected behavior
- Any special setup requirements
- Performance expectations (if applicable)

---

**Forbidden Library Testing Framework v2.0.0**  
*Maintained by Pandora - The Crucible of Code Perfection*  
*VoidCat RDC Excellence Protocol*


