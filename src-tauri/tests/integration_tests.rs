use forbidden_library_native::database::DatabaseManager;
use forbidden_library_native::models::{Conversation, Message, MessageRole, Persona};
use forbidden_library_native::services::Services;
use std::sync::Arc;
use tempfile::TempDir;
use tokio_test;

/// Integration tests for the Forbidden Library
/// Tests complete application workflow using direct service testing
/// This approach eliminates Tauri State dependencies and provides superior test performance

struct IntegrationTestEnvironment {
    services: Arc<Services>,
    _temp_dir: TempDir,
}

impl IntegrationTestEnvironment {
    fn new() -> Self {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let db_manager = DatabaseManager::new_in_memory().expect("Failed to create test database");
        let services = Arc::new(Services::new(Arc::new(db_manager)));

        Self {
            services,
            _temp_dir: temp_dir,
        }
    }
}

/// Test complete conversation lifecycle using direct service calls
#[tokio::test]
async fn test_conversation_lifecycle() {
    let env = IntegrationTestEnvironment::new();

    // 1. Create conversation
    let conversation = env
        .services
        .conversations
        .create_conversation("Integration Test Conversation".to_string(), None)
        .expect("Failed to create conversation");

    assert!(conversation.id.is_some());
    assert_eq!(conversation.title, "Integration Test Conversation");
    assert!(!conversation.archived);

    let conversation_id = conversation.id.unwrap();

    // 2. Add messages to conversation
    let user_message = env
        .services
        .messages
        .add_message(
            conversation_id,
            MessageRole::User,
            "Hello, this is a test message".to_string(),
            None,
        )
        .expect("Failed to add user message");

    assert!(user_message.id.is_some());
    assert_eq!(user_message.role, MessageRole::User);
    assert_eq!(user_message.content, "Hello, this is a test message");

    let assistant_message = env
        .services
        .messages
        .add_message(
            conversation_id,
            MessageRole::Assistant,
            "Hello! I'm here to help with your test.".to_string(),
            None,
        )
        .expect("Failed to add assistant message");

    assert!(assistant_message.id.is_some());
    assert_eq!(assistant_message.role, MessageRole::Assistant);

    // 3. Retrieve conversation with messages
    let retrieved_conversation = env
        .services
        .conversations
        .get_conversation(conversation_id)
        .expect("Failed to get conversation")
        .expect("Conversation should exist");

    assert_eq!(retrieved_conversation.id, Some(conversation_id));
    assert_eq!(
        retrieved_conversation.title,
        "Integration Test Conversation"
    );

    // 4. Retrieve messages
    let messages = env
        .services
        .messages
        .get_messages(conversation_id, None, None)
        .expect("Failed to get messages");

    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].role, MessageRole::User);
    assert_eq!(messages[1].role, MessageRole::Assistant);

    // 5. Archive conversation
    env.services
        .conversations
        .archive_conversation(conversation_id)
        .expect("Failed to archive conversation");

    let archived_conversation = env
        .services
        .conversations
        .get_conversation(conversation_id)
        .expect("Failed to get archived conversation")
        .expect("Archived conversation should exist");

    assert!(archived_conversation.archived);

    println!("✅ Conversation lifecycle test passed: Complete workflow validated");
}

/// Test persona management workflow
#[tokio::test]
async fn test_persona_management() {
    let env = IntegrationTestEnvironment::new();

    // 1. Create persona
    let persona = env
        .services
        .personas
        .create_persona(
            "Test Assistant".to_string(),
            "A helpful test assistant".to_string(),
            "You are a helpful test assistant.".to_string(),
            None,
        )
        .expect("Failed to create persona");

    assert!(persona.id.is_some());
    assert_eq!(persona.name, "Test Assistant");
    assert_eq!(persona.description, "A helpful test assistant");

    let persona_id = persona.id.unwrap();

    // 2. Create conversation with persona
    let conversation = env
        .services
        .conversations
        .create_conversation("Persona Test Conversation".to_string(), Some(persona_id))
        .expect("Failed to create conversation with persona");

    assert_eq!(conversation.persona_id, Some(persona_id));

    // 3. Retrieve persona
    let retrieved_persona = env
        .services
        .personas
        .get_persona(persona_id)
        .expect("Failed to get persona")
        .expect("Persona should exist");

    assert_eq!(retrieved_persona.name, "Test Assistant");

    // 4. Update persona
    let updated_persona = env
        .services
        .personas
        .update_persona(
            persona_id,
            "Updated Test Assistant".to_string(),
            "An updated test assistant".to_string(),
            "You are an updated helpful test assistant.".to_string(),
            None,
        )
        .expect("Failed to update persona");

    assert_eq!(updated_persona.name, "Updated Test Assistant");
    assert_eq!(updated_persona.description, "An updated test assistant");

    // 5. List personas
    let personas = env
        .services
        .personas
        .get_personas(None, None)
        .expect("Failed to get personas");

    assert!(!personas.is_empty());
    assert!(personas.iter().any(|p| p.id == Some(persona_id)));

    println!("✅ Persona management test passed: Complete persona workflow validated");
}

/// Test message management and retrieval
#[tokio::test]
async fn test_message_management() {
    let env = IntegrationTestEnvironment::new();

    // 1. Create conversation
    let conversation = env
        .services
        .conversations
        .create_conversation("Message Test Conversation".to_string(), None)
        .expect("Failed to create conversation");

    let conversation_id = conversation.id.unwrap();

    // 2. Add multiple messages
    let messages_data = vec![
        (MessageRole::User, "First user message"),
        (MessageRole::Assistant, "First assistant response"),
        (MessageRole::User, "Second user message"),
        (MessageRole::Assistant, "Second assistant response"),
        (MessageRole::System, "System message"),
    ];

    let mut created_messages = Vec::new();
    for (role, content) in messages_data {
        let message = env
            .services
            .messages
            .add_message(conversation_id, role, content.to_string(), None)
            .expect(&format!("Failed to add {} message", role));

        created_messages.push(message);
    }

    assert_eq!(created_messages.len(), 5);

    // 3. Test message retrieval with pagination
    let all_messages = env
        .services
        .messages
        .get_messages(conversation_id, None, None)
        .expect("Failed to get all messages");

    assert_eq!(all_messages.len(), 5);

    // 4. Test pagination
    let first_two = env
        .services
        .messages
        .get_messages(conversation_id, Some(2), Some(0))
        .expect("Failed to get first two messages");

    assert_eq!(first_two.len(), 2);
    assert_eq!(first_two[0].role, MessageRole::User);
    assert_eq!(first_two[1].role, MessageRole::Assistant);

    // 5. Test message update
    let first_message_id = created_messages[0].id.unwrap();
    let updated_message = env
        .services
        .messages
        .update_message(first_message_id, "Updated user message".to_string())
        .expect("Failed to update message");

    assert_eq!(updated_message.content, "Updated user message");

    // 6. Test message deletion
    let last_message_id = created_messages.last().unwrap().id.unwrap();
    env.services
        .messages
        .delete_message(last_message_id)
        .expect("Failed to delete message");

    let remaining_messages = env
        .services
        .messages
        .get_messages(conversation_id, None, None)
        .expect("Failed to get remaining messages");

    assert_eq!(remaining_messages.len(), 4);

    println!("✅ Message management test passed: Complete message workflow validated");
}

/// Test conversation search and filtering
#[tokio::test]
async fn test_conversation_search() {
    let env = IntegrationTestEnvironment::new();

    // 1. Create multiple conversations
    let conversation_titles = vec![
        "Work Discussion",
        "Personal Chat",
        "Project Planning",
        "Work Meeting Notes",
        "Personal Notes",
    ];

    for title in &conversation_titles {
        env.services
            .conversations
            .create_conversation(title.to_string(), None)
            .expect(&format!("Failed to create conversation: {}", title));
    }

    // 2. Test basic retrieval
    let all_conversations = env
        .services
        .conversations
        .get_conversations(None, None)
        .expect("Failed to get all conversations");

    assert_eq!(all_conversations.len(), 5);

    // 3. Test pagination
    let first_three = env
        .services
        .conversations
        .get_conversations(Some(3), Some(0))
        .expect("Failed to get first three conversations");

    assert_eq!(first_three.len(), 3);

    // 4. Test offset
    let last_two = env
        .services
        .conversations
        .get_conversations(Some(2), Some(3))
        .expect("Failed to get last two conversations");

    assert_eq!(last_two.len(), 2);

    // 5. Archive some conversations
    let work_conversation = all_conversations
        .iter()
        .find(|c| c.title.contains("Work"))
        .expect("Work conversation should exist");

    env.services
        .conversations
        .archive_conversation(work_conversation.id.unwrap())
        .expect("Failed to archive work conversation");

    // 6. Verify archived conversation is still retrievable
    let archived_conversation = env
        .services
        .conversations
        .get_conversation(work_conversation.id.unwrap())
        .expect("Failed to get archived conversation")
        .expect("Archived conversation should exist");

    assert!(archived_conversation.archived);

    println!("✅ Conversation search test passed: Search and filtering validated");
}

/// Test error handling and edge cases
#[tokio::test]
async fn test_error_handling() {
    let env = IntegrationTestEnvironment::new();

    // 1. Test invalid conversation ID
    let invalid_conversation = env.services.conversations.get_conversation(999999);
    assert!(invalid_conversation.is_ok()); // Should return Ok(None) for non-existent conversation
    assert!(invalid_conversation.unwrap().is_none());

    // 2. Test invalid persona ID
    let invalid_persona = env.services.personas.get_persona(999999);
    assert!(invalid_persona.is_ok()); // Should return Ok(None) for non-existent persona
    assert!(invalid_persona.unwrap().is_none());

    // 3. Test adding message to non-existent conversation
    let message_result = env.services.messages.add_message(
        999999,
        MessageRole::User,
        "Test message".to_string(),
        None,
    );
    assert!(message_result.is_err()); // Should fail for non-existent conversation

    // 4. Test updating non-existent message
    let update_result = env
        .services
        .messages
        .update_message(999999, "Updated content".to_string());
    assert!(update_result.is_err()); // Should fail for non-existent message

    // 5. Test deleting non-existent message
    let delete_result = env.services.messages.delete_message(999999);
    assert!(delete_result.is_err()); // Should fail for non-existent message

    println!("✅ Error handling test passed: Edge cases and error conditions validated");
}

/// Test data integrity and consistency
#[tokio::test]
async fn test_data_integrity() {
    let env = IntegrationTestEnvironment::new();

    // 1. Create conversation and verify initial state
    let conversation = env
        .services
        .conversations
        .create_conversation("Integrity Test".to_string(), None)
        .expect("Failed to create conversation");

    let conversation_id = conversation.id.unwrap();

    // 2. Add messages and verify conversation message count
    for i in 1..=5 {
        env.services
            .messages
            .add_message(
                conversation_id,
                MessageRole::User,
                format!("Message {}", i),
                None,
            )
            .expect(&format!("Failed to add message {}", i));
    }

    // 3. Verify all messages are associated with the conversation
    let messages = env
        .services
        .messages
        .get_messages(conversation_id, None, None)
        .expect("Failed to get messages");

    assert_eq!(messages.len(), 5);
    for message in &messages {
        assert_eq!(message.conversation_id, conversation_id);
    }

    // 4. Test conversation deletion cascades to messages
    env.services
        .conversations
        .delete_conversation(conversation_id)
        .expect("Failed to delete conversation");

    // 5. Verify conversation is deleted
    let deleted_conversation = env
        .services
        .conversations
        .get_conversation(conversation_id)
        .expect("Failed to check deleted conversation");
    assert!(deleted_conversation.is_none());

    // 6. Verify messages are also deleted (if cascade delete is implemented)
    let remaining_messages = env
        .services
        .messages
        .get_messages(conversation_id, None, None)
        .expect("Failed to check remaining messages");
    assert_eq!(remaining_messages.len(), 0);

    println!("✅ Data integrity test passed: Referential integrity and consistency validated");
}

/// Test performance characteristics
#[tokio::test]
async fn test_performance_characteristics() {
    let env = IntegrationTestEnvironment::new();

    // 1. Test bulk conversation creation
    let start_time = std::time::Instant::now();

    for i in 1..=100 {
        env.services
            .conversations
            .create_conversation(format!("Performance Test Conversation {}", i), None)
            .expect(&format!("Failed to create conversation {}", i));
    }

    let creation_time = start_time.elapsed();
    assert!(
        creation_time.as_millis() < 1000,
        "Bulk creation should complete in under 1 second"
    );

    // 2. Test bulk message creation
    let conversation = env
        .services
        .conversations
        .create_conversation("Bulk Message Test".to_string(), None)
        .expect("Failed to create conversation for bulk test");

    let conversation_id = conversation.id.unwrap();
    let message_start_time = std::time::Instant::now();

    for i in 1..=50 {
        env.services
            .messages
            .add_message(
                conversation_id,
                MessageRole::User,
                format!("Bulk message {}", i),
                None,
            )
            .expect(&format!("Failed to add bulk message {}", i));
    }

    let message_creation_time = message_start_time.elapsed();
    assert!(
        message_creation_time.as_millis() < 500,
        "Bulk message creation should complete in under 500ms"
    );

    // 3. Test retrieval performance
    let retrieval_start_time = std::time::Instant::now();

    let conversations = env
        .services
        .conversations
        .get_conversations(None, None)
        .expect("Failed to get conversations");
    let messages = env
        .services
        .messages
        .get_messages(conversation_id, None, None)
        .expect("Failed to get messages");

    let retrieval_time = retrieval_start_time.elapsed();
    assert!(
        retrieval_time.as_millis() < 100,
        "Retrieval should complete in under 100ms"
    );

    assert_eq!(conversations.len(), 101); // 100 + 1 bulk test conversation
    assert_eq!(messages.len(), 50);

    println!("✅ Performance test passed: Sub-second operations validated");
    println!("   - Bulk creation: {}ms", creation_time.as_millis());
    println!(
        "   - Bulk messages: {}ms",
        message_creation_time.as_millis()
    );
    println!("   - Retrieval: {}ms", retrieval_time.as_millis());
}
