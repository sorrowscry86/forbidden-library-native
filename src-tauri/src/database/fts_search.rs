/// Full-Text Search Module for Forbidden Library
///
/// Provides fast, powerful search capabilities using SQLite FTS5
/// (Full-Text Search version 5) for conversations and messages.

use crate::database::DatabaseManager;
use crate::errors::AppResult;
use crate::models::{Conversation, Message};
use rusqlite::Connection;

/// Search results with relevance scores
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchResult {
    pub conversation_id: i64,
    pub message_id: Option<i64>,
    pub title: String,
    pub content: String,
    pub relevance_score: f64,
    pub created_at: String,
    pub snippet: String, // Highlighted snippet showing match context
}

/// Search filter options
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SearchFilters {
    pub persona_id: Option<i64>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub archived: Option<bool>,
    pub min_tokens: Option<i32>,
    pub max_tokens: Option<i32>,
}

/// Initialize FTS5 virtual tables for full-text search
pub fn initialize_fts_tables(conn: &Connection) -> AppResult<()> {
    // Create FTS5 table for conversations
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS conversations_fts USING fts5(
            conversation_id UNINDEXED,
            title,
            metadata,
            tokenize = 'porter unicode61'
        )",
        [],
    )?;

    // Create FTS5 table for messages
    conn.execute(
        "CREATE VIRTUAL TABLE IF NOT EXISTS messages_fts USING fts5(
            message_id UNINDEXED,
            conversation_id UNINDEXED,
            content,
            role UNINDEXED,
            tokenize = 'porter unicode61'
        )",
        [],
    )?;

    // Create triggers to keep FTS tables in sync with main tables

    // Trigger for inserting conversations
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS conversations_ai AFTER INSERT ON conversations BEGIN
            INSERT INTO conversations_fts(conversation_id, title, metadata)
            VALUES (new.id, new.title, new.metadata);
        END",
        [],
    )?;

    // Trigger for updating conversations
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS conversations_au AFTER UPDATE ON conversations BEGIN
            UPDATE conversations_fts
            SET title = new.title, metadata = new.metadata
            WHERE conversation_id = new.id;
        END",
        [],
    )?;

    // Trigger for deleting conversations
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS conversations_ad AFTER DELETE ON conversations BEGIN
            DELETE FROM conversations_fts WHERE conversation_id = old.id;
        END",
        [],
    )?;

    // Trigger for inserting messages
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS messages_ai AFTER INSERT ON messages BEGIN
            INSERT INTO messages_fts(message_id, conversation_id, content, role)
            VALUES (new.id, new.conversation_id, new.content, new.role);
        END",
        [],
    )?;

    // Trigger for updating messages
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS messages_au AFTER UPDATE ON messages BEGIN
            UPDATE messages_fts
            SET content = new.content, role = new.role
            WHERE message_id = new.id;
        END",
        [],
    )?;

    // Trigger for deleting messages
    conn.execute(
        "CREATE TRIGGER IF NOT EXISTS messages_ad AFTER DELETE ON messages BEGIN
            DELETE FROM messages_fts WHERE message_id = old.id;
        END",
        [],
    )?;

    Ok(())
}

/// Rebuild FTS indices from existing data
pub fn rebuild_fts_indices(conn: &Connection) -> AppResult<()> {
    // Clear existing FTS data
    conn.execute("DELETE FROM conversations_fts", [])?;
    conn.execute("DELETE FROM messages_fts", [])?;

    // Repopulate conversations FTS
    conn.execute(
        "INSERT INTO conversations_fts(conversation_id, title, metadata)
         SELECT id, title, metadata FROM conversations",
        [],
    )?;

    // Repopulate messages FTS
    conn.execute(
        "INSERT INTO messages_fts(message_id, conversation_id, content, role)
         SELECT id, conversation_id, content, role FROM messages",
        [],
    )?;

    // Optimize FTS indices
    conn.execute("INSERT INTO conversations_fts(conversations_fts) VALUES('optimize')", [])?;
    conn.execute("INSERT INTO messages_fts(messages_fts) VALUES('optimize')", [])?;

    Ok(())
}

/// Search conversations and messages with full-text search
pub fn search_full_text(
    conn: &Connection,
    query: &str,
    filters: Option<SearchFilters>,
    limit: Option<i32>,
) -> AppResult<Vec<SearchResult>> {
    let limit = limit.unwrap_or(50);

    // Build the WHERE clause for filters
    let mut where_clauses = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    // Add search query
    where_clauses.push("messages_fts MATCH ?");
    params.push(Box::new(query.to_string()));

    if let Some(f) = filters {
        if let Some(persona_id) = f.persona_id {
            where_clauses.push("c.persona_id = ?");
            params.push(Box::new(persona_id));
        }

        if let Some(date_from) = f.date_from {
            where_clauses.push("c.created_at >= ?");
            params.push(Box::new(date_from));
        }

        if let Some(date_to) = f.date_to {
            where_clauses.push("c.created_at <= ?");
            params.push(Box::new(date_to));
        }

        if let Some(archived) = f.archived {
            where_clauses.push("c.archived = ?");
            params.push(Box::new(archived));
        }

        if let Some(min_tokens) = f.min_tokens {
            where_clauses.push("m.tokens_used >= ?");
            params.push(Box::new(min_tokens));
        }

        if let Some(max_tokens) = f.max_tokens {
            where_clauses.push("m.tokens_used <= ?");
            params.push(Box::new(max_tokens));
        }
    }

    let where_clause = if where_clauses.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", where_clauses.join(" AND "))
    };

    // Search query with relevance ranking
    let sql = format!(
        "SELECT
            c.id as conversation_id,
            m.id as message_id,
            c.title,
            m.content,
            bm25(messages_fts) as score,
            m.timestamp,
            snippet(messages_fts, 2, '<mark>', '</mark>', '...', 64) as snippet
        FROM messages_fts
        INNER JOIN messages m ON messages_fts.message_id = m.id
        INNER JOIN conversations c ON m.conversation_id = c.id
        {}
        ORDER BY score
        LIMIT ?",
        where_clause
    );

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let mut stmt = conn.prepare(&sql)?;

    let mut all_params = param_refs;
    all_params.push(&limit);

    let results = stmt.query_map(all_params.as_slice(), |row| {
        Ok(SearchResult {
            conversation_id: row.get(0)?,
            message_id: Some(row.get(1)?),
            title: row.get(2)?,
            content: row.get(3)?,
            relevance_score: row.get(4)?,
            created_at: row.get(5)?,
            snippet: row.get(6)?,
        })
    })?;

    let mut search_results = Vec::new();
    for result in results {
        search_results.push(result?);
    }

    Ok(search_results)
}

/// Advanced search with phrase matching
pub fn search_phrases(
    conn: &Connection,
    phrase: &str,
    limit: Option<i32>,
) -> AppResult<Vec<SearchResult>> {
    let limit = limit.unwrap_or(50);

    // Phrase query with quotes
    let phrase_query = format!("\"{}\"", phrase);

    let sql = "SELECT
            c.id as conversation_id,
            m.id as message_id,
            c.title,
            m.content,
            bm25(messages_fts) as score,
            m.timestamp,
            snippet(messages_fts, 2, '<mark>', '</mark>', '...', 64) as snippet
        FROM messages_fts
        INNER JOIN messages m ON messages_fts.message_id = m.id
        INNER JOIN conversations c ON m.conversation_id = c.id
        WHERE messages_fts MATCH ?
        ORDER BY score
        LIMIT ?";

    let mut stmt = conn.prepare(sql)?;

    let results = stmt.query_map([&phrase_query, &limit.to_string()], |row| {
        Ok(SearchResult {
            conversation_id: row.get(0)?,
            message_id: Some(row.get(1)?),
            title: row.get(2)?,
            content: row.get(3)?,
            relevance_score: row.get(4)?,
            created_at: row.get(5)?,
            snippet: row.get(6)?,
        })
    })?;

    let mut search_results = Vec::new();
    for result in results {
        search_results.push(result?);
    }

    Ok(search_results)
}

/// Search only in conversation titles
pub fn search_titles(
    conn: &Connection,
    query: &str,
    limit: Option<i32>,
) -> AppResult<Vec<SearchResult>> {
    let limit = limit.unwrap_or(50);

    let sql = "SELECT
            c.id as conversation_id,
            NULL as message_id,
            c.title,
            '' as content,
            bm25(conversations_fts) as score,
            c.created_at as timestamp,
            snippet(conversations_fts, 1, '<mark>', '</mark>', '...', 64) as snippet
        FROM conversations_fts
        INNER JOIN conversations c ON conversations_fts.conversation_id = c.id
        WHERE conversations_fts MATCH ?
        ORDER BY score
        LIMIT ?";

    let mut stmt = conn.prepare(sql)?;

    let results = stmt.query_map([query, &limit.to_string()], |row| {
        Ok(SearchResult {
            conversation_id: row.get(0)?,
            message_id: row.get::<_, Option<i64>>(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            relevance_score: row.get(4)?,
            created_at: row.get(5)?,
            snippet: row.get(6)?,
        })
    })?;

    let mut search_results = Vec::new();
    for result in results {
        search_results.push(result?);
    }

    Ok(search_results)
}

/// Get search suggestions based on partial query
pub fn get_search_suggestions(
    conn: &Connection,
    partial_query: &str,
    limit: Option<i32>,
) -> AppResult<Vec<String>> {
    let limit = limit.unwrap_or(10);

    // Use prefix matching with FTS5
    let query = format!("{}*", partial_query);

    let sql = "SELECT DISTINCT
            snippet(messages_fts, 2, '', '', '', 5) as suggestion
        FROM messages_fts
        WHERE messages_fts MATCH ?
        LIMIT ?";

    let mut stmt = conn.prepare(sql)?;

    let results = stmt.query_map([&query, &limit.to_string()], |row| {
        row.get::<_, String>(0)
    })?;

    let mut suggestions = Vec::new();
    for result in results {
        suggestions.push(result?);
    }

    Ok(suggestions)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::database::DatabaseManager;

    #[test]
    fn test_fts_initialization() {
        let db = DatabaseManager::new_in_memory().unwrap();
        let conn = db.get_connection().unwrap();

        let result = initialize_fts_tables(&conn);
        assert!(result.is_ok());

        // Verify tables were created
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name LIKE '%_fts'")
            .unwrap()
            .query_map([], |row| row.get(0))
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        assert!(tables.contains(&"conversations_fts".to_string()));
        assert!(tables.contains(&"messages_fts".to_string()));
    }

    #[test]
    fn test_fts_search() {
        let db = DatabaseManager::new_in_memory().unwrap();
        let conn = db.get_connection().unwrap();

        initialize_fts_tables(&conn).unwrap();

        // Insert test data
        conn.execute(
            "INSERT INTO conversations (uuid, title) VALUES ('test-uuid', 'Test Conversation')",
            [],
        )
        .unwrap();

        conn.execute(
            "INSERT INTO messages (id, conversation_id, role, content) VALUES ('msg1', 1, 'user', 'Hello world this is a test')",
            [],
        )
        .unwrap();

        // Search should find the message
        let results = search_full_text(&conn, "test", None, Some(10)).unwrap();
        assert!(!results.is_empty());
        assert_eq!(results[0].conversation_id, 1);
    }
}
