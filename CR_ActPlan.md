# CR_ActPlan.md

## Forbidden Library Unified Action Plan  
*(Derived from CodyRep.md, September 2025)*

---

## 1. Backend: Robustness, Maintainability, and Performance

### 1.1. Implement Consistent Error Handling System

**Action:**  
- Create a comprehensive `AppError` enum using the `thiserror` crate.
- Refactor all backend methods to use `AppResult<T>` and propagate errors consistently.
- Update Tauri command handlers to map errors to user-friendly messages.

**Reasoning:**  
A unified error system ensures all errors are categorized, traceable, and handled in a type-safe manner. This reduces debugging time, improves user feedback, and prevents silent failures or inconsistent error propagation.

---

### 1.2. Refactor Database Layer for Concurrency and Flexibility

**Action:**  
- Replace `Mutex<Connection>` with a connection pool (`r2d2` + `r2d2_sqlite`).
- Centralize database configuration (encryption, pragmas) via a `DatabaseConfig` struct.
- Eliminate code duplication in database initialization.

**Reasoning:**  
A connection pool allows multiple threads to access the database concurrently, eliminating mutex contention and improving scalability. Centralized configuration makes the system more maintainable and testable, while removing duplication reduces bugs and technical debt.

---

### 1.3. Adopt Repository Pattern and Dependency Injection

**Action:**  
- Create repository modules for each data domain (conversations, messages, personas).
- Refactor services to use repositories, separating business logic from data access.
- Implement a service provider for dependency injection and lazy initialization.

**Reasoning:**  
The repository pattern separates concerns, making business logic independent of data storage. Dependency injection enables easier testing and future refactoring, while lazy initialization improves startup performance and resource usage.

---

## 2. Performance and Scalability

### 2.1. Backend Query and Transaction Optimization

**Action:**  
- Implement prepared statement caching for frequently used queries.
- Batch inserts and updates within transactions for bulk operations.
- Use `RwLock` for read-heavy operations if pooling is not sufficient.
- Run VACUUM conditionally based on fragmentation metrics or on a schedule.

**Reasoning:**  
Statement caching and batching reduce database overhead and improve throughput. Conditional VACUUM avoids unnecessary heavy operations, and `RwLock` (if needed) allows concurrent reads, further boosting performance.

---

### 2.2. IPC and API Efficiency

**Action:**  
- Consolidate fine-grained IPC commands into batch operations (e.g., `get_conversation_with_messages`).
- Implement streaming for large responses and pagination for list endpoints.
- Optimize serialization/deserialization for IPC.

**Reasoning:**  
Reducing the number of IPC calls and optimizing data transfer minimizes latency and memory usage, especially for large datasets. Streaming and pagination prevent UI freezes and excessive memory consumption.

---

### 2.3. Frontend API and State Management

**Action:**  
- Centralize API calls and error handling in a typed API client (TypeScript).
- Add request batching and response caching with TTL for read operations.
- Implement client-side pagination and virtualized lists for large datasets.

**Reasoning:**  
A typed API client ensures consistent error handling and reduces boilerplate. Batching and caching minimize redundant requests, while virtualized lists and pagination keep the UI responsive even with large data.

---

## 3. Data Integrity and Validation

### 3.1. Input Validation

**Action:**  
- Create a validation module in the backend for all user inputs.
- Enforce validation in Tauri command handlers before processing.

**Reasoning:**  
Centralized validation prevents invalid data from entering the system, improving data integrity and user experience. It also ensures consistent validation rules across the application.

---

### 3.2. Pagination and List Operations

**Action:**  
- Standardize pagination parameters and responses across all list endpoints.
- Return total counts and page info for better UX.

**Reasoning:**  
Consistent pagination enables efficient data loading and better user navigation, while providing total counts allows for accurate pagination controls in the UI.

---

## 4. Startup and Resource Optimization

### 4.1. Startup Performance

**Action:**  
- Implement lazy initialization for non-critical services.
- Defer non-essential asset loading until after main UI is interactive.

**Reasoning:**  
Lazy initialization and deferred loading reduce startup time, ensuring the application launches quickly and feels responsive from the start.

---

### 4.2. Memory and Asset Management

**Action:**  
- Use streaming and pagination for large data sets to minimize memory usage.
- Monitor for memory leaks and add development-time leak detection.

**Reasoning:**  
Efficient memory management prevents slowdowns and crashes, especially in long-running desktop applications. Leak detection tools help catch issues early in development.

---

## 5. Monitoring, Benchmarking, and Quality Gates

### 5.1. Benchmark Coverage

**Action:**  
- Expand Criterion benchmarks to cover all critical user paths.
- Add real user monitoring (opt-in telemetry) for performance insights.

**Reasoning:**  
Comprehensive benchmarks catch regressions before release, and real user monitoring provides actionable data for further optimization.

---

### 5.2. Logging and Tracing

**Action:**  
- Use conditional compilation to reduce tracing overhead in production.
- Streamline error and event logging for clarity and minimal overhead.

**Reasoning:**  
Reducing logging in production improves performance, while clear logs aid in debugging and monitoring without excessive resource use.

---

## 6. Frontend UI/UX Responsiveness

### 6.1. Rendering Optimization

**Action:**  
- Implement virtualized lists for conversations and messages.
- Lazy load non-critical components.
- Use derived stores and memoization for computed values to minimize reactivity overhead.

**Reasoning:**  
Virtualization and lazy loading keep the UI fast and responsive, even with large data sets. Memoization and derived stores prevent unnecessary re-renders, improving perceived performance.

---

## 7. Additional Backend and Frontend Improvements

### 7.1. Batch Operations and Parallelism

**Action:**  
- Use `tokio::join!` or `futures::join_all` for parallel execution of independent backend operations.
- Batch frontend API requests where possible.

**Reasoning:**  
Parallelism and batching maximize hardware utilization and minimize user wait times for complex or multi-step operations.

---

### 7.2. Optimize Error Handling Paths

**Action:**  
- Streamline error handling and reduce allocations in error paths (both backend and frontend).

**Reasoning:**  
Efficient error handling reduces overhead, especially in failure scenarios, and improves overall system reliability.

---

### 7.3. Optimize Asset Loading

**Action:**  
- Defer non-critical asset loading in both backend and frontend.

**Reasoning:**  
Deferring asset loading ensures that only essential resources are loaded at startup, reducing initial load times and improving user experience.

---

## 8. Memory Leak Prevention

**Action:**  
- Implement memory monitoring and leak detection in development for both backend and frontend.

**Reasoning:**  
Proactive leak detection ensures long-term stability and prevents gradual performance degradation in production.

---

## 9. Optimize Vacuum Operations

**Action:**  
- Run VACUUM only when fragmentation metrics or a schedule indicate it is necessary.

**Reasoning:**  
Unnecessary VACUUM operations are expensive and can degrade performance; running them conditionally maintains database health without impacting responsiveness.

---

## 10. Documentation and Testing

**Action:**  
- Document all new modules, patterns, and error types.
- Add or update automated tests for all refactored and new code paths.

**Reasoning:**  
Documentation and tests ensure maintainability, knowledge transfer, and long-term code quality.

---

**This plan covers all recommendations and opportunities identified in CodyRep.md, ensuring no aspect is omitted. Each action is justified to maximize maintainability, performance, and user experience for the Forbidden Library project.**
