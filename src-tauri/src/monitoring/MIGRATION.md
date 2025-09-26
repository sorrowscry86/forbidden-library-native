# Monitoring Module Migration Guide

This guide will help you migrate from the old monitoring module to the new, improved version.

## Key Changes

1. **Startup Time Tracking**: Split into two functions for accurate measurement
2. **Error Handling**: Standardized error types with `MonitoringError<E>`
3. **Performance Thresholds**: Configurable thresholds with `PerformanceConfig`
4. **Panic Handling**: All monitoring functions now catch panics
5. **AI Request Tracking**: Now accepts a `Result` to track errors

## Migration Steps

### 1. Startup Time Tracking

**Before:**

```rust
PerformanceMonitor::track_startup_time();
```

**After:**

```rust
// At the very beginning of main()
let startup_start_time = PerformanceMonitor::start_startup_tracking();

// After initialization is complete
PerformanceMonitor::finish_startup_tracking(startup_start_time, None);
```

### 2. Database Operation Monitoring

**Before:**

```rust
let result = PerformanceMonitor::track_database_operation(
    "query_users",
    || db.query_users()
);
```

**After:**

```rust
let result = PerformanceMonitor::track_database_operation(
    "query_users",
    || db.query_users(),
    None // Or Some(&perf_config)
);

// Error handling has changed
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

### 3. IPC Command Monitoring

**Before:**

```rust
let result = PerformanceMonitor::track_ipc_command(
    "get_user_data",
    || get_user_data(user_id)
);
```

**After:**

```rust
let result = PerformanceMonitor::track_ipc_command(
    "get_user_data",
    || get_user_data(user_id),
    None // Or Some(&perf_config)
);

// Error handling has changed
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

### 4. AI Request Tracking

**Before:**

```rust
PerformanceMonitor::track_ai_request("gpt-4", Some(1024));
```

**After:**

```rust
// Now requires a Result parameter
PerformanceMonitor::track_ai_request(
    "gpt-4",
    Some(1024),
    Ok::<(), String>(()) // Or Err("error message")
);
```

### 5. Performance Configuration

**New Feature:**

```rust
// Create a performance configuration
let perf_config = PerformanceConfig::default();

// Or use environment-specific configuration
let perf_config = if env == "production" {
    PerformanceConfig::production()
} else {
    PerformanceConfig::development()
};

// Pass to monitoring functions
PerformanceMonitor::track_database_operation(
    "query_users",
    || db.query_users(),
    Some(&perf_config)
);
```

## Automatic Transaction Management

**New Feature:**

```rust
// Create a scoped transaction that automatically finishes when dropped
let transaction = scoped_transaction("custom_operation", "custom");

// Do some work...

// Transaction will be automatically finished when it goes out of scope
// Or you can manually finish it
transaction.finish();
```
