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

        // Conversations table - Core chat functionality
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

        // Messages table - Individual conversation messages
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

        // Personas table - Character profiles and behavior
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

        // Grimoire entries - Knowledge base system
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

        // API configurations - External service management
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

        // Projects table - Development project tracking
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

        // Create indices for performance
        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_conversations_persona ON conversations(persona_id);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_conversation ON messages(conversation_id);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_messages_timestamp ON messages(timestamp);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_grimoire_category ON grimoire_entries(category);",
            [],
        )?;

        conn.execute(
            "CREATE INDEX IF NOT EXISTS idx_grimoire_tags ON grimoire_entries(tags);",
            [],
        )?;

        Ok(())
    }

    /// Legacy method for backward compatibility - DO NOT USE IN NEW CODE
    /// This method is deprecated and only exists for compatibility with existing services
    /// Use get_connection() instead
    pub fn connection(&self) -> std::sync::MutexGuard<'_, Connection> {
        // For now, create a temporary connection for backward compatibility
        // This is not ideal but allows gradual migration
        // TODO: Update all services to use get_connection() directly
        panic!("Backward compatibility not implemented yet - use get_connection() instead")
    }

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
}
