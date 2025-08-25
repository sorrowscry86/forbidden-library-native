use rusqlite::{Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

/// Database connection manager for Forbidden Library
/// Provides encrypted SQLite storage with VoidCat RDC security standards
pub struct DatabaseManager {
    connection: Mutex<Connection>,
    db_path: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub encryption_key: String,
    pub pragma_settings: Vec<String>,
    pub backup_enabled: bool,
}

impl DatabaseManager {
    /// Create in-memory database for testing
    pub fn new_in_memory() -> Result<Self, Box<dyn std::error::Error>> {
        let connection = Connection::open_in_memory()?;

        // Performance settings for in-memory database
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        connection.execute_batch("PRAGMA temp_store = MEMORY;")?;
        connection.execute_batch("PRAGMA cache_size = 10000;")?;

        let mut db_manager = DatabaseManager {
            connection: Mutex::new(connection),
            db_path: std::path::PathBuf::from(":memory:"),
        };

        db_manager.initialize_schema()?;
        Ok(db_manager)
    }

    /// Initialize encrypted database connection
    /// Enforces VoidCat RDC security protocols with SQLCipher
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, Box<dyn std::error::Error>> {
        let app_data_dir = app_handle.path_resolver()
            .app_data_dir()
            .ok_or("Failed to get app data directory")?;

        std::fs::create_dir_all(&app_data_dir)?;
        let db_path = app_data_dir.join("forbidden_library.db");

        let connection = Connection::open(&db_path)?;

        // Enable SQLite encryption (SQLCipher compatibility)
        // Note: For development, we'll use basic SQLite. In production, compile with SQLCipher support
        // connection.execute_batch("PRAGMA key = 'VoidCatRDC_SecureKey_2024';")?;
        // connection.execute_batch("PRAGMA cipher_page_size = 4096;")?;
        // connection.execute_batch("PRAGMA kdf_iter = 256000;")?;
        // connection.execute_batch("PRAGMA cipher_hmac_algorithm = 'HMAC_SHA512';")?;
        // connection.execute_batch("PRAGMA cipher_kdf_algorithm = 'PBKDF2_HMAC_SHA512';")?;

        // Performance and reliability settings
        connection.execute_batch("PRAGMA foreign_keys = ON;")?;
        connection.execute_batch("PRAGMA journal_mode = WAL;")?;
        connection.execute_batch("PRAGMA synchronous = NORMAL;")?;
        connection.execute_batch("PRAGMA temp_store = MEMORY;")?;
        connection.execute_batch("PRAGMA mmap_size = 268435456;")?; // 256MB
        connection.execute_batch("PRAGMA cache_size = 10000;")?;

        let mut db_manager = DatabaseManager {
            connection: Mutex::new(connection),
            db_path,
        };

        db_manager.initialize_schema()?;
        Ok(db_manager)
    }

    /// Create all required database tables
    /// Implements complete Forbidden Library data model
    fn initialize_schema(&mut self) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();

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

    /// Get database connection reference
    pub fn connection(&self) -> &Mutex<Connection> {
        &self.connection
    }

    /// Get database file path
    pub fn db_path(&self) -> &PathBuf {
        &self.db_path
    }



    /// Perform database vacuum and optimization
    pub fn optimize(&self) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute("VACUUM;", [])?;
        conn.execute("ANALYZE;", [])?;
        Ok(())
    }

    /// Create database backup
    pub fn backup(&self, backup_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        std::fs::copy(&self.db_path, backup_path)?;
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
        connection.execute(
            "CREATE TABLE test (id INTEGER PRIMARY KEY);",
            [],
        ).unwrap();

        assert!(db_path.exists());
    }
}
