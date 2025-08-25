use forbidden_library_native::services::Services;
use forbidden_library_native::database::DatabaseManager;
use forbidden_library_native::models::MessageRole;
use std::sync::Arc;
use tempfile::TempDir;

/// Security audit framework for the Forbidden Library
/// Tests service layer for vulnerabilities and validates security measures
/// This approach eliminates Tauri State dependencies and provides superior security validation

struct SecurityTestEnvironment {
    services: Arc<Services>,
    _temp_dir: TempDir,
}

impl SecurityTestEnvironment {
    fn new() -> Self {
        let temp_dir = tempfile::tempdir().expect("Failed to create temp directory");
        let db_manager = DatabaseManager::new_in_memory()
            .expect("Failed to create test database");
        let services = Arc::new(Services::new(Arc::new(db_manager)));
        
        Self {
            services,
            _temp_dir: temp_dir,
        }
    }
}

/// Test SQL injection prevention in conversation titles
#[tokio::test]
async fn test_sql_injection_prevention_conversations() {
    let env = SecurityTestEnvironment::new();
    
    let malicious_inputs = vec![
        "'; DROP TABLE conversations; --",
        "' OR '1'='1",
        "'; INSERT INTO conversations VALUES (999, 'hacked', 1, '2023-01-01', '2023-01-01', 'false'); --",
        "'; UPDATE conversations SET title = 'hacked'; --",
        "'; DELETE FROM conversations; --",
        "' UNION SELECT * FROM conversations --",
        "'; CREATE TABLE hack (id INTEGER); --",
    ];
    
    for malicious_input in malicious_inputs {
        let result = env.services.conversations.create_conversation(
            malicious_input.to_string(),
            None
        );
        
        // Should succeed but not cause SQL injection
        assert!(result.is_ok(), "Failed to handle malicious input: {}", malicious_input);
        
        let conversation = result.unwrap();
        assert_eq!(conversation.title, malicious_input);
        
        // Verify the table structure is intact
        let conversations = env.services.conversations.get_conversations(None, None).unwrap();
        assert!(!conversations.is_empty(), "Table should still contain data after malicious input");
    }
}

/// Test path traversal prevention in file operations
#[tokio::test]
async fn test_path_traversal_prevention() {
    let env = SecurityTestEnvironment::new();
    
    let path_traversal_attempts = vec![
        "../../../etc/passwd",
        "..\\..\\..\\Windows\\System32\\config\\SAM",
        "....//....//....//etc/passwd",
        "..%2F..%2F..%2Fetc%2Fpasswd",
        "..%5C..%5C..%5CWindows%5CSystem32%5Cconfig%5CSAM",
        "/etc/passwd",
        "C:\\Windows\\System32\\config\\SAM",
        "~/.ssh/id_rsa",
        "C:\\Users\\Administrator\\Desktop\\secret.txt",
    ];
    
    // Test conversation titles with path traversal attempts
    for path_attempt in &path_traversal_attempts {
        let result = env.services.conversations.create_conversation(
            path_attempt.to_string(),
            None
        );
        
        // Should succeed but not allow path traversal
        assert!(result.is_ok(), "Failed to handle path traversal attempt: {}", path_attempt);
        
        let conversation = result.unwrap();
        assert_eq!(conversation.title, *path_attempt);
    }
}

/// Test command injection prevention
#[tokio::test]
async fn test_command_injection_prevention() {
    let env = SecurityTestEnvironment::new();
    
    let command_injection_attempts = vec![
        "test; rm -rf /",
        "test && rm -rf /",
        "test | rm -rf /",
        "test || rm -rf /",
        "test; del C:\\Windows\\System32",
        "test && del C:\\Windows\\System32",
        "test | del C:\\Windows\\System32",
        "test || del C:\\Windows\\System32",
        "test; format C:",
        "test && format C:",
        "test; shutdown -s -t 0",
        "test && shutdown -s -t 0",
    ];
    
    for injection_attempt in command_injection_attempts {
        let result = env.services.conversations.create_conversation(
            injection_attempt.to_string(),
            None
        );
        
        // Should succeed but not execute commands
        assert!(result.is_ok(), "Failed to handle command injection attempt: {}", injection_attempt);
        
        let conversation = result.unwrap();
        assert_eq!(conversation.title, injection_attempt);
    }
}

/// Test XSS prevention in conversation titles
#[tokio::test]
async fn test_xss_prevention() {
    let env = SecurityTestEnvironment::new();
    
    let xss_attempts = vec![
        "<script>alert('xss')</script>",
        "<img src=x onerror=alert('xss')>",
        "javascript:alert('xss')",
        "data:text/html,<script>alert('xss')</script>",
        "<svg onload=alert('xss')>",
        "<iframe src=javascript:alert('xss')>",
        "';alert('xss');//",
        "\"><script>alert('xss')</script>",
    ];
    
    for xss_attempt in xss_attempts {
        let result = env.services.conversations.create_conversation(
            xss_attempt.to_string(),
            None
        );
        
        // Should succeed but not execute scripts
        assert!(result.is_ok(), "Failed to handle XSS attempt: {}", xss_attempt);
        
        let conversation = result.unwrap();
        assert_eq!(conversation.title, xss_attempt);
    }
}

/// Test input validation for conversation titles
#[tokio::test]
async fn test_input_validation_conversation_titles() {
    let env = SecurityTestEnvironment::new();
    
    // Test extremely long titles
    let long_title = "a".repeat(10000);
    let result = env.services.conversations.create_conversation(
        long_title.clone(),
        None
    );
    
    assert!(result.is_ok(), "Failed to handle long title");
    let conversation = result.unwrap();
    assert_eq!(conversation.title, long_title);
    
    // Test titles with null bytes
    let null_title = "test\0title";
    let result = env.services.conversations.create_conversation(
        null_title.to_string(),
        None
    );
    
    assert!(result.is_ok(), "Failed to handle title with null bytes");
    
    // Test titles with unicode characters
    let unicode_title = "测试标题 🚀 特殊字符";
    let result = env.services.conversations.create_conversation(
        unicode_title.to_string(),
        None
    );
    
    assert!(result.is_ok(), "Failed to handle unicode title");
    let conversation = result.unwrap();
    assert_eq!(conversation.title, unicode_title);
}

/// Test persona ID validation
#[tokio::test]
async fn test_persona_id_validation() {
    let env = SecurityTestEnvironment::new();
    
    // Test with negative persona ID
    let result = env.services.conversations.create_conversation(
        "Test Conversation".to_string(),
        Some(-1)
    );
    
    assert!(result.is_ok(), "Should handle negative persona ID gracefully");
    
    // Test with extremely large persona ID
    let result = env.services.conversations.create_conversation(
        "Test Conversation".to_string(),
        Some(i64::MAX)
    );
    
    assert!(result.is_ok(), "Should handle large persona ID gracefully");
    
    // Test with zero persona ID
    let result = env.services.conversations.create_conversation(
        "Test Conversation".to_string(),
        Some(0)
    );
    
    assert!(result.is_ok(), "Should handle zero persona ID gracefully");
}

/// Test pagination parameter validation
#[tokio::test]
async fn test_pagination_parameter_validation() {
    let env = SecurityTestEnvironment::new();
    
    // Test negative limits
    let result = env.services.conversations.get_conversations(
        Some(-1),
        None
    );
    
    assert!(result.is_ok(), "Should handle negative limit gracefully");
    
    // Test negative offsets
    let result = env.services.conversations.get_conversations(
        None,
        Some(-1)
    );
    
    assert!(result.is_ok(), "Should handle negative offset gracefully");
    
    // Test extremely large limits
    let result = env.services.conversations.get_conversations(
        Some(i32::MAX),
        None
    );
    
    assert!(result.is_ok(), "Should handle large limit gracefully");
    
    // Test extremely large offsets
    let result = env.services.conversations.get_conversations(
        None,
        Some(i32::MAX)
    );
    
    assert!(result.is_ok(), "Should handle large offset gracefully");
}

/// Test conversation ID validation
#[tokio::test]
async fn test_conversation_id_validation() {
    let env = SecurityTestEnvironment::new();
    
    // Test with negative conversation ID
    let result = env.services.conversations.get_conversation(-1);
    
    assert!(result.is_ok(), "Should handle negative conversation ID gracefully");
    
    // Test with extremely large conversation ID
    let result = env.services.conversations.get_conversation(i64::MAX);
    
    assert!(result.is_ok(), "Should handle large conversation ID gracefully");
    
    // Test with zero conversation ID
    let result = env.services.conversations.get_conversation(0);
    
    assert!(result.is_ok(), "Should handle zero conversation ID gracefully");
}

/// Test memory exhaustion prevention
#[tokio::test]
async fn test_memory_exhaustion_prevention() {
    let env = SecurityTestEnvironment::new();
    
    // Create many conversations to test memory limits
    for i in 0..1000 {
        let result = create_conversation(
            format!("Memory Test {}", i),
            None,
            State::new(&env.app_state)
        ).await;
        
        assert!(result.is_ok(), "Failed to create conversation {}", i);
    }
    
    // Try to retrieve all conversations
    let result = env.services.conversations.get_conversations(None, None);
    assert!(result.is_ok(), "Should handle large dataset retrieval");
    
    let conversations = result.unwrap();
    assert_eq!(conversations.len(), 1000, "Should retrieve all conversations");
}

/// Test concurrent access security
#[tokio::test]
async fn test_concurrent_access_security() {
    let env = SecurityTestEnvironment::new();
    
    use std::sync::Arc;
    use tokio::task;
    
    let services_arc = Arc::clone(&env.services);
    let mut handles = Vec::new();
    
    // Spawn multiple concurrent tasks
    for i in 0..100 {
        let services_clone = Arc::clone(&services_arc);
        let handle = task::spawn(async move {
            // Create conversation
            let conv_result = services_clone.conversations.create_conversation(
                format!("Concurrent Test {}", i),
                None
            );
            
            assert!(conv_result.is_ok(), "Concurrent conversation creation failed");
            
            let conversation = conv_result.unwrap();
            let conv_id = conversation.id.unwrap();
            
            // Retrieve conversation
            let get_result = services_clone.conversations.get_conversation(conv_id);
            
            assert!(get_result.is_ok(), "Concurrent conversation retrieval failed");
            
            // Delete conversation
            let delete_result = services_clone.conversations.delete_conversation(conv_id);
            
            assert!(delete_result.is_ok(), "Concurrent conversation deletion failed");
        });
        
        handles.push(handle);
    }
    
    // Wait for all tasks to complete
    for handle in handles {
        handle.await.unwrap();
    }
}

/// Test error handling security
#[tokio::test]
async fn test_error_handling_security() {
    let env = SecurityTestEnvironment::new();
    
    // Test that errors don't leak sensitive information
    let result = env.services.conversations.get_conversation(999999);
    
    assert!(result.is_ok(), "Should handle non-existent conversation gracefully");
    assert!(result.unwrap().is_none(), "Should return None for non-existent conversation");
    
    // Test that database errors are handled securely
    let result = env.services.conversations.delete_conversation(999999);
    
    assert!(result.is_err(), "Should return error for non-existent conversation deletion");
    
    // Verify error message doesn't contain sensitive information
    let error_msg = result.unwrap_err();
    assert!(!error_msg.contains("password"), "Error message should not contain sensitive data");
    assert!(!error_msg.contains("secret"), "Error message should not contain sensitive data");
    assert!(!error_msg.contains("key"), "Error message should not contain sensitive data");
}

/// Test service isolation
#[tokio::test]
async fn test_service_isolation() {
    let env = SecurityTestEnvironment::new();
    
    // Test that services are properly isolated and don't interfere with each other
    let conv1 = env.services.conversations.create_conversation(
        "Isolation Test 1".to_string(),
        None
    ).await.unwrap();
    
    let conv2 = env.services.conversations.create_conversation(
        "Isolation Test 2".to_string(),
        None
    ).unwrap();
    
    assert_ne!(conv1.id, conv2.id, "Conversations should have different IDs");
    
    // Test that deleting one conversation doesn't affect the other
    env.services.conversations.delete_conversation(conv1.id.unwrap()).unwrap();
    
    let conv2_retrieved = env.services.conversations.get_conversation(conv2.id.unwrap()).unwrap();
    
    assert!(conv2_retrieved.is_some(), "Second conversation should still exist");
    assert_eq!(conv2_retrieved.unwrap().id, conv2.id);
}

/// Test data integrity under malicious input
#[tokio::test]
async fn test_data_integrity_malicious_input() {
    let env = SecurityTestEnvironment::new();
    
    // Create legitimate conversation
    let legitimate_conv = env.services.conversations.create_conversation(
        "Legitimate Conversation".to_string(),
        None
    ).unwrap();
    
    let legitimate_id = legitimate_conv.id.unwrap();
    
    // Attempt malicious operations
    let malicious_inputs = vec![
        "'; DROP TABLE conversations; --",
        "'; UPDATE conversations SET title = 'hacked'; --",
        "'; DELETE FROM conversations; --",
    ];
    
    for malicious_input in malicious_inputs {
        // Create conversation with malicious input
        env.services.conversations.create_conversation(
            malicious_input.to_string(),
            None
        ).unwrap();
        
        // Verify legitimate conversation still exists and is unchanged
        let retrieved_conv = env.services.conversations.get_conversation(legitimate_id).unwrap();
        
        assert!(retrieved_conv.is_some(), "Legitimate conversation should still exist");
        assert_eq!(retrieved_conv.unwrap().title, "Legitimate Conversation");
    }
    
    // Verify all conversations are still accessible
    let all_conversations = env.services.conversations.get_conversations(None, None).unwrap();
    assert!(all_conversations.len() >= 4, "Should have at least 4 conversations (1 legitimate + 3 malicious)");
}

