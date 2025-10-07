use forbidden_library_native::{DatabaseManager, Services};
use std::sync::Arc;

#[tokio::test]
async fn test_database_and_services_integration() {
    // Create in-memory database
    let db_manager = DatabaseManager::new_in_memory().expect("Failed to create test database");

    let services = Arc::new(Services::new(Arc::new(db_manager)));

    // Test conversation creation
    let conversation = services
        .conversations
        .create_conversation("Integration Test Conversation".to_string(), None)
        .expect("Failed to create conversation");

    assert_eq!(conversation.title, "Integration Test Conversation");
    assert!(conversation.id.is_some());

    // Test conversation retrieval
    let conversations = services
        .conversations
        .get_conversations(None, None)
        .expect("Failed to get conversations");

    assert_eq!(conversations.len(), 1);
    assert_eq!(conversations[0].title, "Integration Test Conversation");

    println!("✅ Integration test passed: Database and Services working correctly");
}
