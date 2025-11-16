use crate::database::DatabaseManager;
use crate::models::{Conversation, Message, Persona};
use chrono::{DateTime, Utc};
use rusqlite::Result as SqliteResult;
use uuid::Uuid;

/// Conversation service - Manages chat sessions and message history
pub struct ConversationService {
    db: std::sync::Arc<DatabaseManager>,
}

impl ConversationService {
    pub fn new(db: std::sync::Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    /// Create new conversation
    pub fn create_conversation(
        &self,
        title: String,
        persona_id: Option<i64>,
    ) -> SqliteResult<Conversation> {
        let conversation = Conversation::new(title, persona_id);
        let uuid_str = conversation.uuid.to_string();

        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "INSERT INTO conversations (uuid, title, persona_id, created_at, updated_at, archived)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        )?;

        stmt.execute([
            &uuid_str,
            &conversation.title,
            &conversation
                .persona_id
                .map(|id| id.to_string())
                .unwrap_or_default(),
            &conversation.created_at.to_rfc3339(),
            &conversation.updated_at.to_rfc3339(),
            &conversation.archived.to_string(),
        ])?;

        let id = conn.last_insert_rowid();
        let mut result = conversation;
        result.id = Some(id);
        Ok(result)
    }

    /// Get all conversations with pagination
    pub fn get_conversations(
        &self,
        limit: Option<i32>,
        offset: Option<i32>,
    ) -> SqliteResult<Vec<Conversation>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);

        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "SELECT id, uuid, title, persona_id, created_at, updated_at, archived
             FROM conversations
             ORDER BY updated_at DESC
             LIMIT ?1 OFFSET ?2",
        )?;

        let rows = stmt.query_map([limit, offset], |row| {
            Ok(Conversation {
                id: Some(row.get::<_, i64>(0)?),
                uuid: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                title: row.get(2)?,
                persona_id: row.get::<_, Option<i64>>(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                archived: row.get::<_, String>(6)? == "true",
                metadata: None, // Load separately if needed
            })
        })?;

        let mut conversations = Vec::new();
        for row in rows {
            conversations.push(row?);
        }
        Ok(conversations)
    }

    /// Get conversation by ID
    pub fn get_conversation(&self, id: i64) -> SqliteResult<Option<Conversation>> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "SELECT id, uuid, title, persona_id, created_at, updated_at, archived
             FROM conversations
             WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map([id], |row| {
            Ok(Conversation {
                id: Some(row.get::<_, i64>(0)?),
                uuid: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                title: row.get(2)?,
                persona_id: row.get::<_, Option<i64>>(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                archived: row.get::<_, String>(6)? == "true",
                metadata: None,
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Search conversations by title or content
    pub fn search_conversations(
        &self,
        query: &str,
        limit: Option<i32>,
    ) -> SqliteResult<Vec<Conversation>> {
        let limit = limit.unwrap_or(50);
        let search_pattern = format!("%{}%", query);

        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;

        // Search by title or messages content
        let mut stmt = conn.prepare(
            "SELECT DISTINCT c.id, c.uuid, c.title, c.persona_id, c.created_at, c.updated_at, c.archived
             FROM conversations c
             LEFT JOIN messages m ON c.id = m.conversation_id
             WHERE c.title LIKE ?1 OR m.content LIKE ?1
             ORDER BY c.updated_at DESC
             LIMIT ?2",
        )?;

        let rows = stmt.query_map(params![&search_pattern, limit], |row| {
            Ok(Conversation {
                id: Some(row.get::<_, i64>(0)?),
                uuid: Uuid::parse_str(&row.get::<_, String>(1)?).unwrap_or_default(),
                title: row.get(2)?,
                persona_id: row.get::<_, Option<i64>>(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                archived: row.get::<_, String>(6)? == "true",
                metadata: None,
            })
        })?;

        let mut conversations = Vec::new();
        for row in rows {
            conversations.push(row?);
        }
        Ok(conversations)
    }

    /// Add message to conversation
    pub fn add_message(
        &self,
        conversation_id: i64,
        role: crate::models::MessageRole,
        content: String,
        tokens_used: Option<i32>,
        model_used: Option<String>,
    ) -> SqliteResult<Message> {
        let message = Message::new(conversation_id, role, content);

        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "INSERT INTO messages (conversation_id, role, content, created_at, tokens_used, model_used)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )?;

        let role_str = match message.role {
            crate::models::MessageRole::User => "user",
            crate::models::MessageRole::Assistant => "assistant",
            crate::models::MessageRole::System => "system",
        };

        let model_used_str = model_used.as_deref().unwrap_or("");
        stmt.execute([
            &conversation_id.to_string(),
            role_str,
            &message.content,
            &message.created_at.to_rfc3339(),
            &tokens_used.map(|t| t.to_string()).unwrap_or_default(),
            model_used_str,
        ])?;

        // Update conversation's updated_at timestamp
        conn.execute(
            "UPDATE conversations SET updated_at = ?1 WHERE id = ?2",
            [&Utc::now().to_rfc3339(), &conversation_id.to_string()],
        )?;

        let id = conn.last_insert_rowid();
        let mut result = message;
        result.id = Some(id);
        result.tokens_used = tokens_used;
        result.model_used = model_used;
        Ok(result)
    }

    /// Get messages for a conversation
    pub fn get_messages(&self, conversation_id: i64) -> SqliteResult<Vec<Message>> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "SELECT id, conversation_id, role, content, created_at, tokens_used, model_used
             FROM messages
             WHERE conversation_id = ?1
             ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map([conversation_id], |row| {
            let role_str: String = row.get(2)?;
            let role = match role_str.as_str() {
                "user" => crate::models::MessageRole::User,
                "assistant" => crate::models::MessageRole::Assistant,
                "system" => crate::models::MessageRole::System,
                _ => crate::models::MessageRole::User, // Default fallback
            };

            Ok(Message {
                id: Some(row.get::<_, i64>(0)?),
                conversation_id: row.get(1)?,
                role,
                content: row.get(3)?,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                tokens_used: row
                    .get::<_, Option<String>>(5)?
                    .and_then(|s| s.parse().ok()),
                model_used: row.get::<_, Option<String>>(6)?.filter(|s| !s.is_empty()),
                metadata: None,
            })
        })?;

        let mut messages = Vec::new();
        for row in rows {
            messages.push(row?);
        }
        Ok(messages)
    }

    /// Delete conversation and all its messages
    pub fn delete_conversation(&self, id: i64) -> SqliteResult<()> {
        // Messages will be deleted automatically due to CASCADE
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        conn.execute("DELETE FROM conversations WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Archive/unarchive conversation
    pub fn set_conversation_archived(&self, id: i64, archived: bool) -> SqliteResult<()> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        conn.execute(
            "UPDATE conversations SET archived = ?1, updated_at = ?2 WHERE id = ?3",
            [
                &archived.to_string(),
                &Utc::now().to_rfc3339(),
                &id.to_string(),
            ],
        )?;
        Ok(())
    }
}

/// Persona service - Manages AI character profiles
pub struct PersonaService {
    db: std::sync::Arc<DatabaseManager>,
}

impl PersonaService {
    pub fn new(db: std::sync::Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    /// Create new persona
    pub fn create_persona(
        &self,
        name: String,
        description: Option<String>,
        system_prompt: String,
    ) -> SqliteResult<Persona> {
        let persona = Persona::new(name, description, system_prompt);

        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "INSERT INTO personas (name, description, system_prompt, created_at, updated_at, active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)"
        )?;

        let description_str = persona.description.as_deref().unwrap_or("");
        stmt.execute([
            &persona.name,
            description_str,
            &persona.system_prompt,
            &persona.created_at.to_rfc3339(),
            &persona.updated_at.to_rfc3339(),
            &persona.active.to_string(),
        ])?;

        let id = conn.last_insert_rowid();
        let mut result = persona;
        result.id = Some(id);
        Ok(result)
    }

    /// Get all active personas
    pub fn get_personas(&self) -> SqliteResult<Vec<Persona>> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, system_prompt, created_at, updated_at, active
             FROM personas
             WHERE active = 'true'
             ORDER BY name ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            Ok(Persona {
                id: Some(row.get::<_, i64>(0)?),
                name: row.get(1)?,
                description: {
                    let desc: String = row.get(2)?;
                    if desc.is_empty() {
                        None
                    } else {
                        Some(desc)
                    }
                },
                system_prompt: row.get(3)?,
                avatar_path: None,
                memory_context: None,
                settings: None,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                active: row.get::<_, String>(6)? == "true",
            })
        })?;

        let mut personas = Vec::new();
        for row in rows {
            personas.push(row?);
        }
        Ok(personas)
    }

    /// Get persona by ID
    pub fn get_persona(&self, id: i64) -> SqliteResult<Option<Persona>> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "SELECT id, name, description, system_prompt, created_at, updated_at, active
             FROM personas
             WHERE id = ?1",
        )?;

        let mut rows = stmt.query_map([id], |row| {
            Ok(Persona {
                id: Some(row.get::<_, i64>(0)?),
                name: row.get(1)?,
                description: {
                    let desc: String = row.get(2)?;
                    if desc.is_empty() {
                        None
                    } else {
                        Some(desc)
                    }
                },
                system_prompt: row.get(3)?,
                avatar_path: None,
                memory_context: None,
                settings: None,
                created_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(4)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                updated_at: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap_or_default()
                    .with_timezone(&Utc),
                active: row.get::<_, String>(6)? == "true",
            })
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Update persona
    pub fn update_persona(
        &self,
        id: i64,
        name: Option<String>,
        description: Option<String>,
        system_prompt: Option<String>,
    ) -> SqliteResult<()> {
        let mut query_parts = Vec::new();
        let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(name) = name {
            query_parts.push("name = ?");
            params.push(Box::new(name));
        }
        if let Some(description) = description {
            query_parts.push("description = ?");
            params.push(Box::new(description));
        }
        if let Some(system_prompt) = system_prompt {
            query_parts.push("system_prompt = ?");
            params.push(Box::new(system_prompt));
        }

        if query_parts.is_empty() {
            return Ok(());
        }

        query_parts.push("updated_at = ?");
        params.push(Box::new(Utc::now().to_rfc3339()));
        params.push(Box::new(id));

        let query = format!(
            "UPDATE personas SET {} WHERE id = ?",
            query_parts.join(", ")
        );

        let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        conn.execute(&query, param_refs.as_slice())?;
        Ok(())
    }

    /// Delete persona
    pub fn delete_persona(&self, id: i64) -> SqliteResult<()> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        conn.execute("DELETE FROM personas WHERE id = ?1", [id])?;
        Ok(())
    }
}

/// API service - Manages external AI service configurations
pub struct ApiService {
    db: std::sync::Arc<DatabaseManager>,
}

impl ApiService {
    pub fn new(db: std::sync::Arc<DatabaseManager>) -> Self {
        Self { db }
    }

    /// Store API configuration (encrypt sensitive data)
    pub fn store_api_config(
        &self,
        provider: String,
        api_key: String,
        base_url: Option<String>,
    ) -> SqliteResult<()> {
        // TODO: Implement proper encryption for API keys
        let encrypted_key = api_key; // Placeholder - implement actual encryption

        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        conn.execute(
            "INSERT OR REPLACE INTO api_configs
             (id, provider, api_key, base_url, created_at, updated_at, active)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            [
                &provider,
                &provider,
                &encrypted_key,
                &base_url.unwrap_or_default(),
                &Utc::now().to_rfc3339(),
                &Utc::now().to_rfc3339(),
                "true",
            ],
        )?;
        Ok(())
    }

    /// Retrieve API configuration (decrypt sensitive data)
    pub fn get_api_config(&self, provider: &str) -> SqliteResult<Option<(String, Option<String>)>> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        let mut stmt = conn.prepare(
            "SELECT api_key, base_url FROM api_configs WHERE provider = ?1 AND active = 'true'",
        )?;

        let mut rows = stmt.query_map([provider], |row| {
            let encrypted_key: String = row.get(0)?;
            let base_url: Option<String> = {
                let url: String = row.get(1)?;
                if url.is_empty() {
                    None
                } else {
                    Some(url)
                }
            };

            // TODO: Implement proper decryption for API keys
            let decrypted_key = encrypted_key; // Placeholder

            Ok((decrypted_key, base_url))
        })?;

        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }

    /// Delete API configuration
    pub fn delete_api_config(&self, provider: &str) -> SqliteResult<()> {
        let conn = self.db.get_connection().map_err(|e| rusqlite::Error::ToSqlConversionFailure(Box::new(e)))?;
        conn.execute(
            "UPDATE api_configs SET active = 'false', updated_at = ?1 WHERE provider = ?2",
            [&Utc::now().to_rfc3339(), provider],
        )?;
        Ok(())
    }
}

/// Service container for dependency injection
pub struct Services {
    pub conversations: ConversationService,
    pub personas: PersonaService,
    pub apis: ApiService,
}

impl Services {
    pub fn new(db: std::sync::Arc<DatabaseManager>) -> Self {
        Self {
            conversations: ConversationService::new(db.clone()),
            personas: PersonaService::new(db.clone()),
            apis: ApiService::new(db),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseManager;
    use std::sync::Arc;
    use tempfile::TempDir;

    /// Test setup helper for creating isolated test environment
    fn setup_test_environment() -> (ConversationService, TempDir) {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let db_path = temp_dir.path().join("test.db");

        // Initialize test database
        let db_manager = DatabaseManager::new_in_memory().expect("Failed to create test database");

        let service = ConversationService::new(Arc::new(db_manager));

        (service, temp_dir)
    }

    #[test]
    fn test_create_conversation() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.create_conversation("Test Conversation".to_string(), None);
        assert!(result.is_ok());

        let conversation = result.unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert!(conversation.id.is_some());
        assert!(!conversation.archived);
    }

    #[test]
    fn test_create_conversation_with_persona() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.create_conversation("Test Conversation".to_string(), Some(1));
        assert!(result.is_ok());

        let conversation = result.unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.persona_id, Some(1));
    }

    #[test]
    fn test_create_conversation_empty_title() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.create_conversation("".to_string(), None);
        assert!(result.is_ok());

        let conversation = result.unwrap();
        assert_eq!(conversation.title, "");
    }

    #[test]
    fn test_get_conversations_empty() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.get_conversations(None, None);
        assert!(result.is_ok());
        let conversations = result.unwrap();
        assert!(conversations.is_empty());
    }

    #[test]
    fn test_get_conversations_with_data() {
        let (service, _temp_dir) = setup_test_environment();

        // Create multiple conversations
        for i in 0..5 {
            service
                .create_conversation(format!("Conversation {}", i), None)
                .unwrap();
        }

        let result = service.get_conversations(None, None);
        assert!(result.is_ok());
        let conversations = result.unwrap();
        assert_eq!(conversations.len(), 5);
    }

    #[test]
    fn test_get_conversations_with_pagination() {
        let (service, _temp_dir) = setup_test_environment();

        // Create multiple conversations
        for i in 0..10 {
            service
                .create_conversation(format!("Conversation {}", i), None)
                .unwrap();
        }

        // Test limit
        let result = service.get_conversations(Some(3), None);
        assert!(result.is_ok());
        let conversations = result.unwrap();
        assert_eq!(conversations.len(), 3);

        // Test offset
        let result = service.get_conversations(Some(3), Some(3));
        assert!(result.is_ok());
        let conversations = result.unwrap();
        assert_eq!(conversations.len(), 3);
    }

    #[test]
    fn test_get_conversation_not_found() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.get_conversation(999);
        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }

    #[test]
    fn test_get_conversation_found() {
        let (service, _temp_dir) = setup_test_environment();

        let created = service
            .create_conversation("Test Conversation".to_string(), None)
            .unwrap();
        let conversation_id = created.id.unwrap();

        let result = service.get_conversation(conversation_id);
        assert!(result.is_ok());

        let conversation = result.unwrap().unwrap();
        assert_eq!(conversation.title, "Test Conversation");
        assert_eq!(conversation.id, Some(conversation_id));
    }

    #[test]
    fn test_delete_conversation() {
        let (service, _temp_dir) = setup_test_environment();

        let created = service
            .create_conversation("Test Conversation".to_string(), None)
            .unwrap();
        let conversation_id = created.id.unwrap();

        let result = service.delete_conversation(conversation_id);
        assert!(result.is_ok());

        // Verify it's deleted
        let get_result = service.get_conversation(conversation_id);
        assert!(get_result.is_ok());
        assert!(get_result.unwrap().is_none());
    }

    #[test]
    fn test_delete_conversation_not_found() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.delete_conversation(999);
        assert!(result.is_err());
    }

    #[test]
    fn test_archive_conversation() {
        let (service, _temp_dir) = setup_test_environment();

        let created = service
            .create_conversation("Test Conversation".to_string(), None)
            .unwrap();
        let conversation_id = created.id.unwrap();

        let result = service.set_conversation_archived(conversation_id, true);
        assert!(result.is_ok());

        // Verify it's archived
        let conversation = service.get_conversation(conversation_id).unwrap().unwrap();
        assert!(conversation.archived);
    }

    #[test]
    fn test_archive_conversation_not_found() {
        let (service, _temp_dir) = setup_test_environment();

        let result = service.set_conversation_archived(999, true);
        assert!(result.is_err());
    }

    /// Performance test for conversation operations
    #[test]
    fn test_conversation_operations_performance() {
        let (service, _temp_dir) = setup_test_environment();
        let start = std::time::Instant::now();

        // Create 100 conversations rapidly
        for i in 0..100 {
            service
                .create_conversation(format!("Performance Test {}", i), None)
                .unwrap();
        }

        let create_duration = start.elapsed();
        assert!(
            create_duration.as_millis() < 500,
            "Conversation creation took too long: {:?}",
            create_duration
        );

        // Test retrieval performance
        let retrieve_start = std::time::Instant::now();
        let conversations = service.get_conversations(None, None).unwrap();
        let retrieve_duration = retrieve_start.elapsed();

        assert_eq!(conversations.len(), 100);
        assert!(
            retrieve_duration.as_millis() < 100,
            "Conversation retrieval took too long: {:?}",
            retrieve_duration
        );
    }

    /// Security test for SQL injection prevention
    #[test]
    fn test_sql_injection_prevention() {
        let (service, _temp_dir) = setup_test_environment();

        // Test with potentially malicious input
        let malicious_title = "'; DROP TABLE conversations; --";

        let result = service.create_conversation(malicious_title.to_string(), None);
        assert!(result.is_ok());

        let conversation = result.unwrap();
        assert_eq!(conversation.title, malicious_title);

        // Verify the table still exists and works
        let conversations = service.get_conversations(None, None).unwrap();
        assert_eq!(conversations.len(), 1);
    }

    /// Test conversation ordering by updated_at
    #[test]
    fn test_conversation_ordering() {
        let (service, _temp_dir) = setup_test_environment();

        // Create conversations with delays
        service
            .create_conversation("First".to_string(), None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        service
            .create_conversation("Second".to_string(), None)
            .unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));
        service
            .create_conversation("Third".to_string(), None)
            .unwrap();

        let conversations = service.get_conversations(None, None).unwrap();
        assert_eq!(conversations.len(), 3);

        // Should be ordered by updated_at DESC (newest first)
        assert_eq!(conversations[0].title, "Third");
        assert_eq!(conversations[1].title, "Second");
        assert_eq!(conversations[2].title, "First");
    }
}
