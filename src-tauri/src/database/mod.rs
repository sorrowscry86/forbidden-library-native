use crate::errors::{AppError, AppResult};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Connection pool type alias for cleaner code
type SqlitePool = Pool<SqliteConnectionManager>;
pub type PooledSqliteConnection = PooledConnection<SqliteConnectionManager>;

/// Database connection manager for Forbidden Library
/// Provides encrypted SQLite storage with VoidCat RDC security standards
/// Uses connection pooling for improved concurrency and performance
pub struct DatabaseManager {
    pool: SqlitePool,
    db_path: PathBuf,
    config: DatabaseConfig,
}

/// Database configuration structure with validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    /// SQLCipher encryption key (empty for development/testing)
    pub encryption_key: String,
    /// SQLite pragma settings for performance optimization
    pub pragma_settings: Vec<String>,
    /// Enable automatic database backups
    pub backup_enabled: bool,
    /// Connection pool configuration
    pub pool_config: PoolConfig,
}

/// Connection pool configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoolConfig {
    /// Maximum number of connections in the pool
    pub max_size: u32,
    /// Minimum number of idle connections to maintain
    pub min_idle: Option<u32>,
    /// Connection timeout in seconds
    pub timeout_seconds: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            encryption_key: String::new(), // No encryption for development
            pragma_settings: vec![
                "PRAGMA foreign_keys = ON".to_string(),
                "PRAGMA journal_mode = WAL".to_string(),
                "PRAGMA synchronous = NORMAL".to_string(),
                "PRAGMA cache_size = 10000".to_string(),
                "PRAGMA temp_store = MEMORY".to_string(),
            ],
            backup_enabled: false,
            pool_config: PoolConfig::default(),
        }
    }
}

impl Default for PoolConfig {
    fn default() -> Self {
        Self {
            max_size: 10,
            min_idle: Some(2),
            timeout_seconds: 30,
        }
    }
}

impl DatabaseConfig {
    /// Validate the database configuration
    pub fn validate(&self) -> AppResult<()> {
        if self.pool_config.max_size == 0 {
            return Err(AppError::validation("Pool max_size must be greater than 0"));
        }

        if self.pool_config.timeout_seconds == 0 {
            return Err(AppError::validation("Pool timeout must be greater than 0"));
        }

        if let Some(min_idle) = self.pool_config.min_idle {
            if min_idle > self.pool_config.max_size {
                return Err(AppError::validation(
                    "min_idle cannot be greater than max_size",
                ));
            }
        }

        Ok(())
    }

    /// Create configuration for production use with encryption
    pub fn production(encryption_key: String) -> Self {
        Self {
            encryption_key,
            pragma_settings: vec![
                "PRAGMA foreign_keys = ON".to_string(),
                "PRAGMA journal_mode = WAL".to_string(),
                "PRAGMA synchronous = FULL".to_string(),
                "PRAGMA cache_size = 20000".to_string(),
                "PRAGMA temp_store = MEMORY".to_string(),
                "PRAGMA secure_delete = ON".to_string(),
            ],
            backup_enabled: true,
            pool_config: PoolConfig {
                max_size: 20,
                min_idle: Some(5),
                timeout_seconds: 60,
            },
        }
    }

    /// Create configuration for in-memory testing
    pub fn in_memory() -> Self {
        Self {
            encryption_key: String::new(),
            pragma_settings: vec![
                "PRAGMA foreign_keys = ON".to_string(),
                "PRAGMA temp_store = MEMORY".to_string(),
                "PRAGMA cache_size = 10000".to_string(),
            ],
            backup_enabled: false,
            pool_config: PoolConfig {
                max_size: 5,
                min_idle: Some(1),
                timeout_seconds: 10,
            },
        }
    }
}

impl DatabaseManager {
    /// Create database manager with connection pooling using custom configuration
    pub fn new_with_config(db_path: PathBuf, config: DatabaseConfig) -> AppResult<Self> {
        config.validate()?;

        let manager = SqliteConnectionManager::file(&db_path);

        let pool = Pool::builder()
            .max_size(config.pool_config.max_size)
            .min_idle(config.pool_config.min_idle)
            .connection_timeout(Duration::from_secs(config.pool_config.timeout_seconds))
            .build(manager)
            .map_err(|e| AppError::database(format!("Failed to create connection pool: {}", e)))?;

        let db_manager = DatabaseManager {
            pool,
            db_path,
            config: config.clone(),
        };

        // Initialize the database schema and apply pragma settings
        db_manager.initialize_schema()?;
        db_manager.apply_pragma_settings()?;

        Ok(db_manager)
    }

    /// Create database manager with default configuration (for production use)
    pub fn new(app_handle: &tauri::AppHandle) -> AppResult<Self> {
        let app_data_dir = app_handle
            .path_resolver()
            .app_data_dir()
            .ok_or_else(|| AppError::io("Failed to get app data directory"))?;

        std::fs::create_dir_all(&app_data_dir)
            .map_err(|e| AppError::io(format!("Failed to create app data directory: {}", e)))?;

        let db_path = app_data_dir.join("forbidden_library.db");
        let config = DatabaseConfig::default();

        Self::new_with_config(db_path, config)
    }

    /// Create in-memory database for testing
    pub fn new_in_memory() -> AppResult<Self> {
        let db_path = PathBuf::from(":memory:");
        let config = DatabaseConfig::in_memory();

        Self::new_with_config(db_path, config)
    }

    /// Get a connection from the pool
    pub fn get_connection(&self) -> AppResult<PooledSqliteConnection> {
        self.pool
            .get()
            .map_err(|e| AppError::database(format!("Failed to get connection from pool: {}", e)))
    }

    /// Apply pragma settings to a connection
    fn apply_pragma_settings(&self) -> AppResult<()> {
        let conn = self.get_connection()?;

        for pragma in &self.config.pragma_settings {
            conn.execute_batch(pragma).map_err(|e| {
                AppError::database(format!("Failed to apply pragma '{}': {}", pragma, e))
            })?;
        }

        // Apply encryption if configured
        if !self.config.encryption_key.is_empty() {
            // Validate encryption key to prevent SQL injection
            // Keys should only contain alphanumeric characters, hyphens, and underscores
            if !self.config.encryption_key.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
                return Err(AppError::validation(
                    "Encryption key contains invalid characters. Only alphanumeric, hyphens, and underscores allowed."
                ));
            }

            // Safe to use in SQL now that we've validated the key format
            let encryption_cmd = format!("PRAGMA key = '{}';", self.config.encryption_key);
            conn.execute_batch(&encryption_cmd).map_err(|e| {
                AppError::encryption(format!("Failed to set encryption key: {}", e))
            })?;
        }

        Ok(())
    }

    /// Get database path
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }

    /// Get database configuration
    pub fn config(&self) -> &DatabaseConfig {
        &self.config
    }

    /// Create all required database tables
    /// Implements complete Forbidden Library data model
    fn initialize_schema(&self) -> AppResult<()> {
        let conn = self.get_connection()?;

        // Create all tables
        Self::create_conversations_table(&conn)?;
        Self::create_messages_table(&conn)?;
        Self::create_personas_table(&conn)?;
        Self::create_grimoire_table(&conn)?;
        Self::create_api_configs_table(&conn)?;
        Self::create_projects_table(&conn)?;

        // Create all indices
        Self::create_performance_indices(&conn)?;

        Ok(())
    }

    /// Create conversations table
    fn create_conversations_table(conn: &Connection) -> AppResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS conversations (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                uuid TEXT NOT NULL UNIQUE,
                title TEXT NOT NULL,
                persona_id TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                archived BOOLEAN DEFAULT FALSE,
                metadata TEXT,
                FOREIGN KEY (persona_id) REFERENCES personas (id)
            );",
            [],
        )?;
        Ok(())
    }

    /// Create messages table
    fn create_messages_table(conn: &Connection) -> AppResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS messages (
                id TEXT PRIMARY KEY,
                conversation_id TEXT NOT NULL,
                role TEXT NOT NULL CHECK (role IN ('user', 'assistant', 'system')),
                content TEXT NOT NULL,
                timestamp DATETIME DEFAULT CURRENT_TIMESTAMP,
                tokens_used INTEGER DEFAULT 0,
                model_used TEXT,
                metadata TEXT,
                FOREIGN KEY (conversation_id) REFERENCES conversations (id) ON DELETE CASCADE
            );",
            [],
        )?;
        Ok(())
    }

    /// Create personas table
    fn create_personas_table(conn: &Connection) -> AppResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS personas (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                system_prompt TEXT NOT NULL,
                avatar_path TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                active BOOLEAN DEFAULT TRUE,
                preferences TEXT
            );",
            [],
        )?;
        Ok(())
    }

    /// Create grimoire entries table
    fn create_grimoire_table(conn: &Connection) -> AppResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS grimoire_entries (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                content TEXT NOT NULL,
                category TEXT,
                tags TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                accessed_count INTEGER DEFAULT 0,
                last_accessed DATETIME,
                encrypted BOOLEAN DEFAULT FALSE
            );",
            [],
        )?;
        Ok(())
    }

    /// Create API configurations table
    fn create_api_configs_table(conn: &Connection) -> AppResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS api_configs (
                id TEXT PRIMARY KEY,
                provider TEXT NOT NULL,
                api_key TEXT NOT NULL,
                base_url TEXT,
                model_preferences TEXT,
                rate_limits TEXT,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                active BOOLEAN DEFAULT TRUE
            );",
            [],
        )?;
        Ok(())
    }

    /// Create projects table
    fn create_projects_table(conn: &Connection) -> AppResult<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                description TEXT,
                repository_url TEXT,
                status TEXT DEFAULT 'active',
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                metadata TEXT
            );",
            [],
        )?;
        Ok(())
    }

    /// Create performance indices for all tables
    fn create_performance_indices(conn: &Connection) -> AppResult<()> {
        let indices = [
            "CREATE INDEX IF NOT EXISTS idx_conversations_persona ON conversations(persona_id);",
            "CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);",
            "CREATE INDEX IF NOT EXISTS idx_messages_timestamp ON messages(timestamp);",
            "CREATE INDEX IF NOT EXISTS idx_grimoire_category ON grimoire_entries(category);",
            "CREATE INDEX IF NOT EXISTS idx_grimoire_tags ON grimoire_entries(tags);",
        ];

        for index_sql in &indices {
            conn.execute(index_sql, [])?;
        }

        Ok(())
    }

    // REMOVED: Legacy connection() method that was causing panics
    // All services have been migrated to use get_connection() instead
    // If you need a connection, use: let conn = db_manager.get_connection()?;

    /// Optimize database (VACUUM, ANALYZE)
    pub fn optimize(&self) -> AppResult<()> {
        let conn = self.get_connection()?;
        conn.execute_batch("VACUUM; ANALYZE;")?;
        Ok(())
    }

    /// Backup database to specified path
    pub fn backup(&self, backup_path: &PathBuf) -> AppResult<()> {
        if self.db_path.to_str() == Some(":memory:") {
            return Err(AppError::validation("Cannot backup in-memory database"));
        }

        std::fs::copy(&self.db_path, backup_path)
            .map_err(|e| AppError::io(format!("Failed to backup database: {}", e)))?;

        Ok(())
    }

    /// Execute a function within a database transaction
    ///
    /// This method automatically handles BEGIN, COMMIT, and ROLLBACK:
    /// - Begins a transaction
    /// - Executes the provided function
    /// - Commits if the function succeeds
    /// - Rolls back if the function returns an error or panics
    ///
    /// # Arguments
    /// * `f` - A closure that performs database operations
    ///
    /// # Returns
    /// * `Ok(T)` if the transaction succeeds
    /// * `Err(AppError)` if the transaction fails or is rolled back
    ///
    /// # Example
    /// ```rust,ignore
    /// db_manager.with_transaction(|conn| {
    ///     conn.execute("INSERT INTO users (name) VALUES (?)", params![name])?;
    ///     conn.execute("INSERT INTO profiles (user_id) VALUES (?)", params![user_id])?;
    ///     Ok(())
    /// })?;
    /// ```
    pub fn with_transaction<T, F>(&self, f: F) -> AppResult<T>
    where
        F: FnOnce(&rusqlite::Transaction) -> AppResult<T>,
    {
        let mut conn = self.get_connection()?;

        let tx = conn.transaction()
            .map_err(|e| AppError::database(format!("Failed to begin transaction: {}", e)))?;

        match f(&tx) {
            Ok(result) => {
                tx.commit()
                    .map_err(|e| AppError::database(format!("Failed to commit transaction: {}", e)))?;
                Ok(result)
            }
            Err(e) => {
                // Rollback is automatic when transaction is dropped, but we can be explicit
                let _ = tx.rollback();
                Err(e)
            }
        }
    }

    /// Execute a function with a savepoint (nested transaction)
    ///
    /// Savepoints allow for partial rollbacks within a transaction.
    /// This is useful for complex operations where you want to retry
    /// specific parts without rolling back the entire transaction.
    ///
    /// # Arguments
    /// * `conn` - An existing transaction or connection
    /// * `savepoint_name` - Name for the savepoint
    /// * `f` - A closure that performs database operations
    ///
    /// # Returns
    /// * `Ok(T)` if the savepoint succeeds
    /// * `Err(AppError)` if the savepoint fails or is rolled back
    pub fn with_savepoint<T, F>(
        conn: &rusqlite::Connection,
        savepoint_name: &str,
        f: F,
    ) -> AppResult<T>
    where
        F: FnOnce(&rusqlite::Savepoint) -> AppResult<T>,
    {
        let sp = conn.savepoint_with_name(savepoint_name)
            .map_err(|e| AppError::database(format!("Failed to create savepoint: {}", e)))?;

        match f(&sp) {
            Ok(result) => {
                sp.commit()
                    .map_err(|e| AppError::database(format!("Failed to commit savepoint: {}", e)))?;
                Ok(result)
            }
            Err(e) => {
                let _ = sp.rollback();
                Err(e)
            }
        }
    }

    /// Check if a transaction is currently active on a connection
    ///
    /// This is useful for debugging and ensuring transactions are properly managed.
    pub fn is_in_transaction(conn: &rusqlite::Connection) -> bool {
        // SQLite returns true if in a transaction, false otherwise
        conn.is_autocommit() == false
    }
}

/// Database error types for enhanced error handling
#[derive(Debug, thiserror::Error)]
pub enum DatabaseError {
    #[error("Database connection failed: {0}")]
    ConnectionFailed(String),

    #[error("Schema initialization failed: {0}")]
    SchemaFailed(String),

    #[error("Query execution failed: {0}")]
    QueryFailed(String),

    #[error("Encryption setup failed: {0}")]
    EncryptionFailed(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_database_initialization() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let connection = Connection::open(&db_path).unwrap();
        // Test schema creation without encryption for unit tests
        connection
            .execute("CREATE TABLE test (id INTEGER PRIMARY KEY);", [])
            .unwrap();

        assert!(db_path.exists());
    }

    #[test]
    fn test_transaction_commit() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute("CREATE TABLE test_users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])
                .unwrap();
        }

        // Execute operations in a transaction
        let result = db_manager.with_transaction(|tx| {
            tx.execute("INSERT INTO test_users (name) VALUES (?)", ["Alice"])?;
            tx.execute("INSERT INTO test_users (name) VALUES (?)", ["Bob"])?;
            Ok(())
        });

        assert!(result.is_ok());

        // Verify both inserts were committed
        let conn = db_manager.get_connection().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_users", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_transaction_rollback_on_error() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute("CREATE TABLE test_users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])
                .unwrap();
        }

        // Execute operations in a transaction that will fail
        let result = db_manager.with_transaction(|tx| {
            tx.execute("INSERT INTO test_users (name) VALUES (?)", ["Alice"])?;
            // This will fail because we're returning an error
            Err(AppError::validation("Test error - should rollback"))
        });

        assert!(result.is_err());

        // Verify no inserts were committed (rollback worked)
        let conn = db_manager.get_connection().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_users", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 0);
    }

    #[test]
    fn test_transaction_rollback_on_constraint_violation() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table with a unique constraint
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute(
                "CREATE TABLE test_users (id INTEGER PRIMARY KEY, email TEXT NOT NULL UNIQUE)",
                []
            ).unwrap();
        }

        // First transaction succeeds
        let result1 = db_manager.with_transaction(|tx| {
            tx.execute("INSERT INTO test_users (email) VALUES (?)", ["test@example.com"])?;
            Ok(())
        });
        assert!(result1.is_ok());

        // Second transaction fails due to unique constraint
        let result2 = db_manager.with_transaction(|tx| {
            tx.execute("INSERT INTO test_users (email) VALUES (?)", ["test@example.com"])?;
            Ok(())
        });
        assert!(result2.is_err());

        // Verify only one row exists
        let conn = db_manager.get_connection().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_users", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_transaction_with_return_value() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute("CREATE TABLE test_users (id INTEGER PRIMARY KEY, name TEXT NOT NULL)", [])
                .unwrap();
        }

        // Execute transaction and return a value
        let user_id = db_manager.with_transaction(|tx| {
            tx.execute("INSERT INTO test_users (name) VALUES (?)", ["Alice"])?;
            let id: i64 = tx.last_insert_rowid();
            Ok(id)
        });

        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), 1);
    }

    #[test]
    fn test_savepoint_commit() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute("CREATE TABLE test_data (id INTEGER PRIMARY KEY, value TEXT)", [])
                .unwrap();
        }

        let mut conn = db_manager.get_connection().unwrap();

        // Start a transaction
        let tx = conn.transaction().unwrap();

        // Insert first record
        tx.execute("INSERT INTO test_data (value) VALUES (?)", ["before_savepoint"])
            .unwrap();

        // Use savepoint for nested operation
        let result = DatabaseManager::with_savepoint(&tx, "sp1", |sp| {
            sp.execute("INSERT INTO test_data (value) VALUES (?)", ["in_savepoint"])?;
            Ok(())
        });

        assert!(result.is_ok());
        tx.commit().unwrap();

        // Verify both records exist
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_data", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn test_savepoint_rollback() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute("CREATE TABLE test_data (id INTEGER PRIMARY KEY, value TEXT)", [])
                .unwrap();
        }

        let mut conn = db_manager.get_connection().unwrap();

        // Start a transaction
        let tx = conn.transaction().unwrap();

        // Insert first record
        tx.execute("INSERT INTO test_data (value) VALUES (?)", ["before_savepoint"])
            .unwrap();

        // Use savepoint that will fail
        let result = DatabaseManager::with_savepoint(&tx, "sp1", |sp| {
            sp.execute("INSERT INTO test_data (value) VALUES (?)", ["in_savepoint"])?;
            Err(AppError::validation("Savepoint test error"))
        });

        assert!(result.is_err());
        tx.commit().unwrap();

        // Verify only the first record exists (savepoint was rolled back)
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_data", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);

        let value: String = conn
            .query_row("SELECT value FROM test_data", [], |row| row.get(0))
            .unwrap();
        assert_eq!(value, "before_savepoint");
    }

    #[test]
    fn test_is_in_transaction() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();
        let mut conn = db_manager.get_connection().unwrap();

        // Initially not in a transaction
        assert!(!DatabaseManager::is_in_transaction(&conn));

        // Start a transaction
        let tx = conn.transaction().unwrap();
        assert!(DatabaseManager::is_in_transaction(&tx));

        // Commit the transaction
        tx.commit().unwrap();

        // No longer in a transaction
        assert!(!DatabaseManager::is_in_transaction(&conn));
    }

    #[test]
    fn test_nested_transactions_via_savepoints() {
        let db_manager = DatabaseManager::new_in_memory().unwrap();

        // Create a test table
        {
            let conn = db_manager.get_connection().unwrap();
            conn.execute("CREATE TABLE test_levels (id INTEGER PRIMARY KEY, level INTEGER)", [])
                .unwrap();
        }

        let result = db_manager.with_transaction(|tx| {
            tx.execute("INSERT INTO test_levels (level) VALUES (?)", [1])?;

            // First savepoint
            DatabaseManager::with_savepoint(tx, "level2", |sp1| {
                sp1.execute("INSERT INTO test_levels (level) VALUES (?)", [2])?;

                // Nested savepoint
                DatabaseManager::with_savepoint(sp1, "level3", |sp2| {
                    sp2.execute("INSERT INTO test_levels (level) VALUES (?)", [3])?;
                    Ok(())
                })?;

                Ok(())
            })?;

            Ok(())
        });

        assert!(result.is_ok());

        // Verify all three levels were committed
        let conn = db_manager.get_connection().unwrap();
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM test_levels", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 3);
    }
}
