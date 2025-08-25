use criterion::{black_box, criterion_group, criterion_main, Criterion};
use forbidden_library_native::database::DatabaseManager;
use forbidden_library_native::services::Services;
use forbidden_library_native::models::MessageRole;
use std::sync::Arc;
use std::time::Instant;

/// Performance benchmarks for the Forbidden Library
/// Validates sub-second startup and 60 FPS UI requirements (16.67ms per operation)

fn setup_test_environment() -> Arc<Services> {
    let db_manager = DatabaseManager::new_in_memory()
        .expect("Failed to create test database");
    Arc::new(Services::new(Arc::new(db_manager)))
}

/// Benchmark database initialization performance
fn benchmark_database_initialization(c: &mut Criterion) {
    c.bench_function("database_initialization", |b| {
        b.iter(|| {
            let _db_manager = DatabaseManager::new_in_memory()
                .expect("Failed to create test database");
        });
    });
}

/// Benchmark services initialization performance
fn benchmark_services_initialization(c: &mut Criterion) {
    c.bench_function("services_initialization", |b| {
        b.iter(|| {
            let db_manager = DatabaseManager::new_in_memory()
                .expect("Failed to create test database");
            let _services = Services::new(Arc::new(db_manager));
        });
    });
}

/// Benchmark conversation creation performance
fn benchmark_conversation_creation(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("conversation_creation", |b| {
        b.iter(|| {
            let _conversation = services.conversations.create_conversation(
                black_box("Benchmark Conversation".to_string()),
                None
            ).expect("Failed to create conversation");
        });
    });
}

/// Benchmark bulk conversation creation performance
fn benchmark_bulk_conversation_creation(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("bulk_conversation_creation_100", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _conversation = services.conversations.create_conversation(
                    format!("Benchmark Conversation {}", i),
                    None
                ).expect("Failed to create conversation");
            }
        });
    });
}

/// Benchmark conversation retrieval performance
fn benchmark_conversation_retrieval(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversation
    let conversation = services.conversations.create_conversation(
        "Benchmark Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    c.bench_function("conversation_retrieval", |b| {
        b.iter(|| {
            let _conversation = services.conversations.get_conversation(conversation_id)
                .expect("Failed to get conversation");
        });
    });
}

/// Benchmark conversation listing performance
fn benchmark_conversation_listing(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversations
    for i in 0..50 {
        services.conversations.create_conversation(
            format!("Benchmark Conversation {}", i),
            None
        ).expect("Failed to create conversation");
    }
    
    c.bench_function("conversation_listing_50", |b| {
        b.iter(|| {
            let _conversations = services.conversations.get_conversations(None, None)
                .expect("Failed to get conversations");
        });
    });
}

/// Benchmark message creation performance
fn benchmark_message_creation(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversation
    let conversation = services.conversations.create_conversation(
        "Benchmark Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    c.bench_function("message_creation", |b| {
        b.iter(|| {
            let _message = services.conversations.add_message(
                conversation_id,
                MessageRole::User,
                black_box("Benchmark message content".to_string()),
                None,
                None,
            ).expect("Failed to create message");
        });
    });
}

/// Benchmark bulk message creation performance
fn benchmark_bulk_message_creation(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversation
    let conversation = services.conversations.create_conversation(
        "Benchmark Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    c.bench_function("bulk_message_creation_100", |b| {
        b.iter(|| {
            for i in 0..100 {
                let _message = services.conversations.add_message(
                    conversation_id,
                    MessageRole::User,
                    format!("Benchmark message {}", i),
                    None,
                    None,
                ).expect("Failed to create message");
            }
        });
    });
}

/// Benchmark message retrieval performance
fn benchmark_message_retrieval(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversation and messages
    let conversation = services.conversations.create_conversation(
        "Benchmark Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    for i in 0..50 {
        services.conversations.add_message(
            conversation_id,
            MessageRole::User,
            format!("Benchmark message {}", i),
            None,
            None,
        ).expect("Failed to create message");
    }
    
    c.bench_function("message_retrieval_50", |b| {
        b.iter(|| {
            let _messages = services.conversations.get_messages(conversation_id)
                .expect("Failed to get messages");
        });
    });
}

/// Benchmark persona creation performance
fn benchmark_persona_creation(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("persona_creation", |b| {
        b.iter(|| {
            let _persona = services.personas.create_persona(
                black_box("Benchmark Persona".to_string()),
                black_box(Some("A benchmark persona".to_string())),
                black_box("You are a benchmark persona.".to_string()),
            ).expect("Failed to create persona");
        });
    });
}

/// Benchmark persona retrieval performance
fn benchmark_persona_retrieval(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test persona
    let persona = services.personas.create_persona(
        "Benchmark Persona".to_string(),
        Some("A benchmark persona".to_string()),
        "You are a benchmark persona.".to_string(),
    ).expect("Failed to create persona");
    let persona_id = persona.id.unwrap();
    
    c.bench_function("persona_retrieval", |b| {
        b.iter(|| {
            let _persona = services.personas.get_persona(persona_id)
                .expect("Failed to get persona");
        });
    });
}

/// Benchmark conversation archiving performance
fn benchmark_conversation_archiving(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversation
    let conversation = services.conversations.create_conversation(
        "Benchmark Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    c.bench_function("conversation_archiving", |b| {
        b.iter(|| {
            services.conversations.set_conversation_archived(conversation_id, true)
                .expect("Failed to archive conversation");
        });
    });
}

/// Benchmark API configuration storage performance
fn benchmark_api_config_storage(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("api_config_storage", |b| {
        b.iter(|| {
            services.apis.store_api_config(
                black_box("benchmark_provider".to_string()),
                black_box("benchmark_api_key".to_string()),
                Some("https://api.benchmark.com".to_string()),
            ).expect("Failed to store API config");
        });
    });
}

/// Benchmark API configuration retrieval performance
fn benchmark_api_config_retrieval(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Store test config
    services.apis.store_api_config(
        "benchmark_provider".to_string(),
        "benchmark_api_key".to_string(),
        Some("https://api.benchmark.com".to_string()),
    ).expect("Failed to store API config");
    
    c.bench_function("api_config_retrieval", |b| {
        b.iter(|| {
            let _config = services.apis.get_api_config("benchmark_provider")
                .expect("Failed to get API config");
        });
    });
}

/// Benchmark conversation with persona creation performance
fn benchmark_conversation_with_persona(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test persona
    let persona = services.personas.create_persona(
        "Benchmark Persona".to_string(),
        Some("A benchmark persona".to_string()),
        "You are a benchmark persona.".to_string(),
    ).expect("Failed to create persona");
    let persona_id = persona.id.unwrap();
    
    c.bench_function("conversation_with_persona", |b| {
        b.iter(|| {
            let _conversation = services.conversations.create_conversation(
                black_box("Benchmark Conversation".to_string()),
                Some(persona_id)
            ).expect("Failed to create conversation");
        });
    });
}

/// Benchmark error handling performance (non-existent resources)
fn benchmark_error_handling(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("error_handling_conversation_not_found", |b| {
        b.iter(|| {
            let _result = services.conversations.get_conversation(999999);
        });
    });
    
    c.bench_function("error_handling_persona_not_found", |b| {
        b.iter(|| {
            let _result = services.personas.get_persona(999999);
        });
    });
}

/// Benchmark concurrent operations performance
fn benchmark_concurrent_operations(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("concurrent_conversation_creation", |b| {
        b.iter(|| {
            let services_clone = services.clone();
            let handle = std::thread::spawn(move || {
                for i in 0..10 {
                    let _conversation = services_clone.conversations.create_conversation(
                        format!("Concurrent Conversation {}", i),
                        None
                    ).expect("Failed to create conversation");
                }
            });
            handle.join().expect("Thread failed");
        });
    });
}

/// Benchmark memory usage and cleanup
fn benchmark_memory_usage(c: &mut Criterion) {
    let services = setup_test_environment();
    
    c.bench_function("memory_usage_large_dataset", |b| {
        b.iter(|| {
            // Create large dataset
            for i in 0..1000 {
                let conversation = services.conversations.create_conversation(
                    format!("Memory Test Conversation {}", i),
                    None
                ).expect("Failed to create conversation");
                
                let conversation_id = conversation.id.unwrap();
                
                // Add messages to each conversation
                for j in 0..10 {
                    services.conversations.add_message(
                        conversation_id,
                        MessageRole::User,
                        format!("Message {} in conversation {}", j, i),
                        None,
                        None,
                    ).expect("Failed to create message");
                }
            }
            
            // Retrieve all conversations
            let _conversations = services.conversations.get_conversations(None, None)
                .expect("Failed to get conversations");
        });
    });
}

/// Benchmark startup time performance
fn benchmark_startup_time(c: &mut Criterion) {
    c.bench_function("startup_time", |b| {
        b.iter(|| {
            let start = Instant::now();
            
            // Simulate application startup
            let db_manager = DatabaseManager::new_in_memory()
                .expect("Failed to create database");
            let services = Services::new(Arc::new(db_manager));
            
            // Create initial data
            for i in 0..10 {
                services.conversations.create_conversation(
                    format!("Startup Conversation {}", i),
                    None
                ).expect("Failed to create conversation");
            }
            
            let duration = start.elapsed();
            black_box(duration);
        });
    });
}

/// Benchmark UI responsiveness (simulated)
fn benchmark_ui_responsiveness(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test data
    let conversation = services.conversations.create_conversation(
        "UI Test Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    for i in 0..20 {
        services.conversations.add_message(
            conversation_id,
            MessageRole::User,
            format!("UI test message {}", i),
            None,
            None,
        ).expect("Failed to create message");
    }
    
    c.bench_function("ui_conversation_load", |b| {
        b.iter(|| {
            let start = Instant::now();
            
            // Simulate UI conversation loading
            let _conversation = services.conversations.get_conversation(conversation_id)
                .expect("Failed to get conversation");
            let _messages = services.conversations.get_messages(conversation_id)
                .expect("Failed to get messages");
            
            let duration = start.elapsed();
            black_box(duration);
        });
    });
    
    c.bench_function("ui_conversation_list_load", |b| {
        b.iter(|| {
            let start = Instant::now();
            
            // Simulate UI conversation list loading
            let _conversations = services.conversations.get_conversations(None, None)
                .expect("Failed to get conversations");
            
            let duration = start.elapsed();
            black_box(duration);
        });
    });
}

/// Benchmark search and filtering performance
fn benchmark_search_performance(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test data with various titles
    for i in 0..100 {
        services.conversations.create_conversation(
            format!("Search Test Conversation {}", i),
            None
        ).expect("Failed to create conversation");
    }
    
    c.bench_function("search_conversations", |b| {
        b.iter(|| {
            let _conversations = services.conversations.get_conversations(None, None)
                .expect("Failed to get conversations");
            
            // Simulate client-side filtering
            let filtered: Vec<_> = _conversations.into_iter()
                .filter(|c| c.title.contains("Search"))
                .collect();
            
            black_box(filtered);
        });
    });
}

/// Benchmark data export performance
fn benchmark_export_performance(c: &mut Criterion) {
    let services = setup_test_environment();
    
    // Create test conversation with messages
    let conversation = services.conversations.create_conversation(
        "Export Test Conversation".to_string(),
        None
    ).expect("Failed to create conversation");
    let conversation_id = conversation.id.unwrap();
    
    for i in 0..50 {
        services.conversations.add_message(
            conversation_id,
            MessageRole::User,
            format!("Export test message {}", i),
            None,
            None,
        ).expect("Failed to create message");
    }
    
    c.bench_function("conversation_export", |b| {
        b.iter(|| {
            let _conversation = services.conversations.get_conversation(conversation_id)
                .expect("Failed to get conversation");
            let _messages = services.conversations.get_messages(conversation_id)
                .expect("Failed to get messages");
            
            // Simulate export data structure
            let export_data = serde_json::json!({
                "conversation": _conversation,
                "messages": _messages,
                "exported_at": chrono::Utc::now(),
            });
            
            black_box(export_data);
        });
    });
}

criterion_group!(
    benches,
    benchmark_database_initialization,
    benchmark_services_initialization,
    benchmark_conversation_creation,
    benchmark_bulk_conversation_creation,
    benchmark_conversation_retrieval,
    benchmark_conversation_listing,
    benchmark_message_creation,
    benchmark_bulk_message_creation,
    benchmark_message_retrieval,
    benchmark_persona_creation,
    benchmark_persona_retrieval,
    benchmark_conversation_archiving,
    benchmark_api_config_storage,
    benchmark_api_config_retrieval,
    benchmark_conversation_with_persona,
    benchmark_error_handling,
    benchmark_concurrent_operations,
    benchmark_memory_usage,
    benchmark_startup_time,
    benchmark_ui_responsiveness,
    benchmark_search_performance,
    benchmark_export_performance,
);

criterion_main!(benches);


