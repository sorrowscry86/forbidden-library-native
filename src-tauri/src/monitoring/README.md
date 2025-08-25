# Performance Monitoring Module

This module provides comprehensive performance monitoring and error tracking for the Forbidden Library application.

## Features

- **Startup Time Tracking**: Accurately measure application startup time
- **Database Operation Monitoring**: Track database operations with performance thresholds
- **IPC Command Monitoring**: Monitor IPC commands with performance thresholds
- **AI Request Tracking**: Track AI requests with token usage
- **Error Handling**: Standardized error types for all monitoring operations
- **Transaction Management**: Automatic transaction management with scoped transactions
- **Configurable Thresholds**: Environment-specific performance thresholds

## Usage

### Startup Time Tracking

```rust
// At the very beginning of main()
let startup_start_time = PerformanceMonitor::start_startup_tracking();

// After initialization is complete
PerformanceMonitor::finish_startup_tracking(startup_start_time, Some(&perf_config));
```

### Database Operation Monitoring

```rust
let result = PerformanceMonitor::track_database_operation(
    "query_users",
    || db.query_users(),
    Some(&perf_config)
);
```

### IPC Command Monitoring

```rust
let result = PerformanceMonitor::track_ipc_command(
    "get_user_data",
    || get_user_data(user_id),
    Some(&perf_config)
);
```

### AI Request Tracking

```rust
PerformanceMonitor::track_ai_request(
    "gpt-4",
    Some(1024),
    result
);
```

### Custom Performance Configuration

```rust
// Use default configuration
let config = PerformanceConfig::default();

// Use environment-specific configuration
let config = PerformanceConfig::production();
let config = PerformanceConfig::development();

// Create custom configuration
let config = PerformanceConfig::builder()
    .startup_threshold_ms(500)
    .database_threshold_ms(25)
    .ipc_threshold_ms(75)
    .ai_request_threshold_ms(1000)
    .build();
```

## Error Handling

All monitoring functions return a `Result<T, MonitoringError<E>>` where `E` is the error type of the operation being monitored. This allows for consistent error handling across all monitoring functions.

```rust
match result {
    Ok(value) => {
        // Operation succeeded
    },
    Err(MonitoringError::Operation(e)) => {
        // Operation failed with error e
    },
    Err(MonitoringError::Panic(msg)) => {
        // Operation panicked with message msg
    },
    Err(MonitoringError::Timeout(msg)) => {
        // Operation timed out with message msg
    },
}
```

## Testing

The monitoring module includes comprehensive tests for all functionality. Run the tests with:

```bash
cargo test --package forbidden-library --lib -- monitoring::tests
```
