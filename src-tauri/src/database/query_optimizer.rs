/// Database Query Optimizer
///
/// Provides optimization utilities for database queries including:
/// - Query result caching
/// - Batch operations
/// - Performance monitoring
/// - Index recommendations

use crate::database::DatabaseManager;
use crate::errors::AppResult;
use rusqlite::Connection;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Query cache entry with TTL
#[derive(Clone)]
struct CacheEntry {
    data: String,
    inserted_at: Instant,
    ttl: Duration,
}

/// Query performance metrics
#[derive(Debug, Clone)]
pub struct QueryMetrics {
    pub query: String,
    pub execution_time_ms: u64,
    pub rows_affected: usize,
    pub timestamp: Instant,
}

/// Query cache for frequently accessed data
pub struct QueryCache {
    cache: Arc<Mutex<HashMap<String, CacheEntry>>>,
    default_ttl: Duration,
}

impl QueryCache {
    /// Create a new query cache with default TTL
    pub fn new(default_ttl_seconds: u64) -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
            default_ttl: Duration::from_secs(default_ttl_seconds),
        }
    }

    /// Get cached result if available and not expired
    pub fn get(&self, key: &str) -> Option<String> {
        let mut cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get(key) {
            if entry.inserted_at.elapsed() < entry.ttl {
                return Some(entry.data.clone());
            } else {
                // Entry expired, remove it
                cache.remove(key);
            }
        }

        None
    }

    /// Store result in cache with optional custom TTL
    pub fn set(&self, key: String, data: String, ttl: Option<Duration>) {
        let mut cache = self.cache.lock().unwrap();

        cache.insert(key, CacheEntry {
            data,
            inserted_at: Instant::now(),
            ttl: ttl.unwrap_or(self.default_ttl),
        });
    }

    /// Invalidate specific cache entry
    pub fn invalidate(&self, key: &str) {
        let mut cache = self.cache.lock().unwrap();
        cache.remove(key);
    }

    /// Clear all cache entries
    pub fn clear(&self) {
        let mut cache = self.cache.lock().unwrap();
        cache.clear();
    }

    /// Get cache statistics
    pub fn stats(&self) -> CacheStats {
        let cache = self.cache.lock().unwrap();
        let total_entries = cache.len();
        let expired_entries = cache.values()
            .filter(|entry| entry.inserted_at.elapsed() >= entry.ttl)
            .count();

        CacheStats {
            total_entries,
            active_entries: total_entries - expired_entries,
            expired_entries,
        }
    }
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: usize,
    pub active_entries: usize,
    pub expired_entries: usize,
}

/// Performance monitor for tracking query execution times
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<Vec<QueryMetrics>>>,
    max_metrics: usize,
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new(max_metrics: usize) -> Self {
        Self {
            metrics: Arc::new(Mutex::new(Vec::with_capacity(max_metrics))),
            max_metrics,
        }
    }

    /// Record a query execution
    pub fn record(&self, metric: QueryMetrics) {
        let mut metrics = self.metrics.lock().unwrap();

        if metrics.len() >= self.max_metrics {
            // Remove oldest metric
            metrics.remove(0);
        }

        metrics.push(metric);
    }

    /// Get average execution time for a specific query pattern
    pub fn get_average_time(&self, query_pattern: &str) -> Option<u64> {
        let metrics = self.metrics.lock().unwrap();

        let matching: Vec<_> = metrics.iter()
            .filter(|m| m.query.contains(query_pattern))
            .collect();

        if matching.is_empty() {
            return None;
        }

        let sum: u64 = matching.iter().map(|m| m.execution_time_ms).sum();
        Some(sum / matching.len() as u64)
    }

    /// Get slowest queries
    pub fn get_slowest_queries(&self, limit: usize) -> Vec<QueryMetrics> {
        let mut metrics = self.metrics.lock().unwrap().clone();
        metrics.sort_by(|a, b| b.execution_time_ms.cmp(&a.execution_time_ms));
        metrics.truncate(limit);
        metrics
    }

    /// Get all metrics
    pub fn get_all_metrics(&self) -> Vec<QueryMetrics> {
        self.metrics.lock().unwrap().clone()
    }

    /// Clear all metrics
    pub fn clear(&self) {
        self.metrics.lock().unwrap().clear();
    }
}

/// Batch operation helper for bulk inserts
pub struct BatchInserter {
    batch_size: usize,
}

impl BatchInserter {
    /// Create a new batch inserter
    pub fn new(batch_size: usize) -> Self {
        Self { batch_size }
    }

    /// Insert multiple rows in batches using transactions
    pub fn batch_insert<T, F>(
        &self,
        db: &DatabaseManager,
        items: Vec<T>,
        insert_fn: F,
    ) -> AppResult<usize>
    where
        F: Fn(&Connection, &[T]) -> AppResult<usize>,
    {
        let mut total_inserted = 0;

        for chunk in items.chunks(self.batch_size) {
            let inserted = db.with_transaction(|tx| {
                insert_fn(tx, chunk)
            })?;

            total_inserted += inserted;
        }

        Ok(total_inserted)
    }
}

/// Additional index recommendations for optimal query performance
pub fn create_additional_indices(conn: &Connection) -> AppResult<()> {
    let indices = [
        // Conversation indices
        "CREATE INDEX IF NOT EXISTS idx_conversations_archived ON conversations(archived) WHERE archived = FALSE;",
        "CREATE INDEX IF NOT EXISTS idx_conversations_created_at ON conversations(created_at);",
        "CREATE INDEX IF NOT EXISTS idx_conversations_updated_at ON conversations(updated_at);",

        // Message indices
        "CREATE INDEX IF NOT EXISTS idx_messages_role ON messages(role);",
        "CREATE INDEX IF NOT EXISTS idx_messages_tokens_used ON messages(tokens_used);",
        "CREATE INDEX IF NOT EXISTS idx_messages_model_used ON messages(model_used);",

        // Persona indices
        "CREATE INDEX IF NOT EXISTS idx_personas_name ON personas(name);",
        "CREATE INDEX IF NOT EXISTS idx_personas_active ON personas(active) WHERE active = TRUE;",

        // Grimoire indices
        "CREATE INDEX IF NOT EXISTS idx_grimoire_last_accessed ON grimoire_entries(last_accessed);",
        "CREATE INDEX IF NOT EXISTS idx_grimoire_accessed_count ON grimoire_entries(accessed_count);",

        // API configs indices
        "CREATE INDEX IF NOT EXISTS idx_api_configs_provider ON api_configs(provider);",
        "CREATE INDEX IF NOT EXISTS idx_api_configs_active ON api_configs(active) WHERE active = TRUE;",

        // Composite indices for common queries
        "CREATE INDEX IF NOT EXISTS idx_messages_conversation_timestamp
         ON messages(conversation_id, timestamp);",
        "CREATE INDEX IF NOT EXISTS idx_conversations_persona_updated
         ON conversations(persona_id, updated_at);",
    ];

    for index_sql in &indices {
        conn.execute(index_sql, [])?;
    }

    Ok(())
}

/// Analyze query plan to identify potential optimizations
pub fn analyze_query_plan(conn: &Connection, query: &str) -> AppResult<String> {
    let explain_query = format!("EXPLAIN QUERY PLAN {}", query);

    let mut stmt = conn.prepare(&explain_query)?;
    let mut plan = String::new();

    let rows = stmt.query_map([], |row| {
        let detail: String = row.get(3)?;
        Ok(detail)
    })?;

    for row in rows {
        plan.push_str(&row?);
        plan.push('\n');
    }

    Ok(plan)
}

/// Vacuum and analyze database for optimal performance
pub fn optimize_database(conn: &Connection) -> AppResult<()> {
    // ANALYZE updates statistics about table contents
    conn.execute_batch("ANALYZE;")?;

    // Get database size before vacuum
    let size_before: i64 = conn.query_row(
        "SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()",
        [],
        |row| row.get(0),
    )?;

    // VACUUM rebuilds the database file, repacking it into a minimal amount of disk space
    conn.execute_batch("VACUUM;")?;

    // Get database size after vacuum
    let size_after: i64 = conn.query_row(
        "SELECT page_count * page_size as size FROM pragma_page_count(), pragma_page_size()",
        [],
        |row| row.get(0),
    )?;

    tracing::info!(
        "Database optimized: {} bytes -> {} bytes (saved {} bytes)",
        size_before,
        size_after,
        size_before - size_after
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_query_cache_basic() {
        let cache = QueryCache::new(60);

        // Cache miss
        assert!(cache.get("test_key").is_none());

        // Cache set
        cache.set("test_key".to_string(), "test_value".to_string(), None);

        // Cache hit
        assert_eq!(cache.get("test_key"), Some("test_value".to_string()));
    }

    #[test]
    fn test_query_cache_expiration() {
        let cache = QueryCache::new(1); // 1 second TTL

        cache.set("test_key".to_string(), "test_value".to_string(), None);
        assert_eq!(cache.get("test_key"), Some("test_value".to_string()));

        // Wait for expiration
        thread::sleep(Duration::from_secs(2));

        // Should be expired
        assert!(cache.get("test_key").is_none());
    }

    #[test]
    fn test_query_cache_invalidation() {
        let cache = QueryCache::new(60);

        cache.set("test_key".to_string(), "test_value".to_string(), None);
        assert_eq!(cache.get("test_key"), Some("test_value".to_string()));

        cache.invalidate("test_key");
        assert!(cache.get("test_key").is_none());
    }

    #[test]
    fn test_query_cache_stats() {
        let cache = QueryCache::new(60);

        cache.set("key1".to_string(), "value1".to_string(), None);
        cache.set("key2".to_string(), "value2".to_string(), None);

        let stats = cache.stats();
        assert_eq!(stats.total_entries, 2);
        assert_eq!(stats.active_entries, 2);
        assert_eq!(stats.expired_entries, 0);
    }

    #[test]
    fn test_performance_monitor() {
        let monitor = PerformanceMonitor::new(100);

        monitor.record(QueryMetrics {
            query: "SELECT * FROM users".to_string(),
            execution_time_ms: 10,
            rows_affected: 5,
            timestamp: Instant::now(),
        });

        monitor.record(QueryMetrics {
            query: "SELECT * FROM users WHERE id = ?".to_string(),
            execution_time_ms: 5,
            rows_affected: 1,
            timestamp: Instant::now(),
        });

        let avg = monitor.get_average_time("SELECT * FROM users");
        assert!(avg.is_some());
        assert_eq!(avg.unwrap(), 7); // (10 + 5) / 2
    }

    #[test]
    fn test_performance_monitor_slowest() {
        let monitor = PerformanceMonitor::new(100);

        monitor.record(QueryMetrics {
            query: "SLOW QUERY".to_string(),
            execution_time_ms: 100,
            rows_affected: 1000,
            timestamp: Instant::now(),
        });

        monitor.record(QueryMetrics {
            query: "FAST QUERY".to_string(),
            execution_time_ms: 5,
            rows_affected: 1,
            timestamp: Instant::now(),
        });

        let slowest = monitor.get_slowest_queries(1);
        assert_eq!(slowest.len(), 1);
        assert_eq!(slowest[0].query, "SLOW QUERY");
        assert_eq!(slowest[0].execution_time_ms, 100);
    }

    #[test]
    fn test_batch_inserter() {
        use crate::database::DatabaseManager;

        let db = DatabaseManager::new_in_memory().unwrap();

        // Create test table
        {
            let conn = db.get_connection().unwrap();
            conn.execute(
                "CREATE TABLE test_items (id INTEGER PRIMARY KEY, value TEXT)",
                [],
            ).unwrap();
        }

        let inserter = BatchInserter::new(10);
        let items: Vec<String> = (0..25).map(|i| format!("item_{}", i)).collect();

        let result = inserter.batch_insert(&db, items.clone(), |conn, batch| {
            for item in batch {
                conn.execute("INSERT INTO test_items (value) VALUES (?)", [item])?;
            }
            Ok(batch.len())
        });

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 25);

        // Verify all items were inserted
        let conn = db.get_connection().unwrap();
        let count: i64 = conn.query_row("SELECT COUNT(*) FROM test_items", [], |row| row.get(0)).unwrap();
        assert_eq!(count, 25);
    }
}
