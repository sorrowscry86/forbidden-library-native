# Immediate Sentry Implementation Guide
## Step-by-Step Backend Integration

**Date:** August 21, 2025  
**Responsible:** Pandora  
**Priority:** Critical  
**Timeline:** Immediate

---

## **Phase 1: Backend Sentry Integration (IMMEDIATE)**

### **Step 1: Create Sentry Projects**

1. **Access Sentry Organization**: `voidcat-rdc`
2. **Create Backend Project**: `forbidden-library-backend`
   - Platform: Rust
   - Environment: Development, Staging, Production
3. **Create Frontend Project**: `forbidden-library-frontend`
   - Platform: SvelteKit
   - Environment: Development, Staging, Production
4. **Create Desktop Project**: `forbidden-library-desktop`
   - Platform: Tauri
   - Environment: Windows, macOS, Linux

### **Step 2: Add Sentry Dependencies**

**Update `src-tauri/Cargo.toml`:**
```toml
[dependencies]
sentry = "0.35"
sentry-tauri = "0.3"
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

[dev-dependencies]
sentry-test = "0.35"
```

**Update `package.json`:**
```json
{
  "dependencies": {
    "@sentry/sveltekit": "^8.0.0",
    "@sentry/tracing": "^8.0.0"
  }
}
```

### **Step 3: Backend Integration**

**Update `src-tauri/src/main.rs`:**
```rust
use sentry::{init, ClientOptions};
use sentry_tauri::sentry;
use tracing::{info, error, warn};

#[tokio::main]
async fn main() {
    // Initialize Sentry
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN").unwrap_or_else(|_| "YOUR_SENTRY_DSN".to_string()),
        sentry::ClientOptions::default()
            .traces_sample_rate(1.0)
            .enable_profiling(true)
            .profiles_sample_rate(1.0)
            .environment(Some(std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())))
    ));

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter("info")
        .init();

    info!("Starting Forbidden Library backend...");

    // Initialize Tauri with Sentry
    tauri::Builder::default()
        .plugin(sentry_tauri::plugin())
        .setup(|app| {
            info!("Tauri app setup complete");
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### **Step 4: Service Layer Integration**

**Update `src-tauri/src/services/mod.rs`:**
```rust
use sentry::{add_breadcrumb, Breadcrumb};
use tracing::{info, error, warn, instrument};

#[instrument(skip(self))]
pub async fn create_conversation(&self, title: String) -> Result<Conversation, Box<dyn std::error::Error>> {
    add_breadcrumb(Breadcrumb {
        message: Some(format!("Creating conversation: {}", title)),
        category: Some("conversation".to_string()),
        level: sentry::Level::Info,
        ..Default::default()
    });

    let start_time = std::time::Instant::now();
    
    let result = self.database_manager.create_conversation(&title).await;
    
    let duration = start_time.elapsed();
    if duration.as_millis() > 100 {
        warn!("Slow conversation creation: {}ms", duration.as_millis());
    }

    match result {
        Ok(conversation) => {
            info!("Conversation created successfully: {}", conversation.id);
            Ok(conversation)
        }
        Err(e) => {
            error!("Failed to create conversation: {}", e);
            sentry::capture_error(&e);
            Err(e)
        }
    }
}
```

### **Step 5: IPC Command Integration**

**Update `src-tauri/src/commands.rs`:**
```rust
use sentry::{add_breadcrumb, Breadcrumb};
use tracing::{info, error, instrument};

#[tauri::command]
#[instrument]
pub async fn create_conversation(
    title: String,
    state: tauri::State<'_, AppState>,
) -> Result<Conversation, String> {
    add_breadcrumb(Breadcrumb {
        message: Some(format!("IPC: Creating conversation: {}", title)),
        category: Some("ipc".to_string()),
        level: sentry::Level::Info,
        ..Default::default()
    });

    let start_time = std::time::Instant::now();
    
    let result = state.conversation_service.create_conversation(title).await;
    
    let duration = start_time.elapsed();
    if duration.as_millis() > 50 {
        warn!("Slow IPC command: {}ms", duration.as_millis());
    }

    match result {
        Ok(conversation) => {
            info!("IPC command successful: create_conversation");
            Ok(conversation)
        }
        Err(e) => {
            error!("IPC command failed: create_conversation - {}", e);
            sentry::capture_error(&e);
            Err(e.to_string())
        }
    }
}
```

### **Step 6: Performance Monitoring**

**Create `src-tauri/src/monitoring.rs`:**
```rust
use sentry::{add_breadcrumb, Breadcrumb, start_transaction};
use tracing::{info, error, warn};

pub struct PerformanceMonitor;

impl PerformanceMonitor {
    pub fn track_startup_time() {
        let transaction = start_transaction(
            Some("app.startup".into()),
            Some("app.startup".into()),
        );
        
        // Measure startup time
        let start_time = std::time::Instant::now();
        
        // Your startup logic here
        
        let duration = start_time.elapsed();
        if duration.as_millis() > 1000 {
            error!("Startup time exceeded 1 second: {}ms", duration.as_millis());
            sentry::capture_message(
                &format!("Slow startup: {}ms", duration.as_millis()),
                sentry::Level::Error,
            );
        } else {
            info!("Startup time: {}ms", duration.as_millis());
        }
        
        transaction.finish();
    }

    pub fn track_database_operation<F, T>(operation: &str, f: F) -> Result<T, Box<dyn std::error::Error>>
    where
        F: FnOnce() -> Result<T, Box<dyn std::error::Error>>,
    {
        let transaction = start_transaction(
            Some(format!("db.{}", operation).into()),
            Some("db.operation".into()),
        );
        
        let start_time = std::time::Instant::now();
        let result = f();
        let duration = start_time.elapsed();
        
        if duration.as_millis() > 50 {
            warn!("Slow database operation {}: {}ms", operation, duration.as_millis());
        }
        
        transaction.finish();
        result
    }
}
```

### **Step 7: Environment Configuration**

**Create `.env` file:**
```env
SENTRY_DSN=https://your-sentry-dsn@sentry.io/project-id
ENVIRONMENT=development
SENTRY_TRACES_SAMPLE_RATE=1.0
SENTRY_PROFILES_SAMPLE_RATE=1.0
```

**Update `src-tauri/tauri.conf.json`:**
```json
{
  "tauri": {
    "bundle": {
      "active": true
    },
    "security": {
      "csp": "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval'; connect-src 'self' https://sentry.io"
    }
  }
}
```

---

## **Testing Sentry Integration**

### **Test Error Reporting**
```rust
#[tauri::command]
pub fn test_sentry_error() -> Result<(), String> {
    sentry::capture_message("Test error from Forbidden Library", sentry::Level::Error);
    Ok(())
}
```

### **Test Performance Monitoring**
```rust
#[tauri::command]
pub fn test_performance() -> Result<(), String> {
    PerformanceMonitor::track_database_operation("test_query", || {
        std::thread::sleep(std::time::Duration::from_millis(100));
        Ok(())
    })?;
    Ok(())
}
```

---

## **Next Steps**

1. **Immediate**: Implement backend Sentry integration (Steps 1-7)
2. **Next**: Test error reporting and performance monitoring
3. **Following**: Begin frontend Sentry integration
4. **Final**: Cross-platform monitoring setup

---

**This implementation provides immediate error tracking and performance monitoring for the Rust backend, establishing the foundation for comprehensive application monitoring.**

