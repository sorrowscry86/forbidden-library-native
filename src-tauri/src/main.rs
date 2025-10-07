// Prevents additional console window on Windows in release, DO NOT REMOVE!!
//! Forbidden Library - Native Desktop Application
//!
//! High-performance AI conversation manager built with Rust and Tauri.
//! Transforms the web-based prototype into a secure, offline-capable desktop application.
//!
//! ## VoidCat RDC Production Standards
//! - Sub-second launch time
//! - 60 FPS UI performance
//! - Encrypted SQLite database
//! - Local-first privacy architecture
//!
//! ## Contact & Support
//! - Developer: @sorrowscry86
//! - Organization: VoidCat RDC
//! - Contact: SorrowsCry86@voidcat.org
//! - Support: CashApp $WykeveTF

use std::sync::Arc;
use tauri::Manager;
use tracing::{error, info};

mod commands;
mod database;
mod models;
mod monitoring;
mod services;

use commands::AppState;
use database::DatabaseManager;
use monitoring::{PerformanceConfig, PerformanceMonitor};
use services::Services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Sentry for production error tracking
    let _guard = sentry::init((
        std::env::var("SENTRY_DSN")
            .unwrap_or_else(|_| "https://b9f589b354fd05ee3e2c5d67f4bc3699@o4509552575053824.ingest.us.sentry.io/4509884862169088".to_string()),
        sentry::ClientOptions {
            traces_sample_rate: 1.0,
            environment: Some(std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()).into()),
            ..Default::default()
        }
    ));

    // Track startup performance
    let startup_start_time = PerformanceMonitor::start_startup_tracking();

    // Create performance config based on environment
    let perf_config = if std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string())
        == "production"
    {
        PerformanceConfig::production()
    } else {
        PerformanceConfig::development()
    };

    // Initialize comprehensive logging system
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "forbidden_library=info,tauri=warn".into()),
        )
        .with_target(true)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();

    info!(
        "🚀 Forbidden Library v{} - VoidCat RDC Production",
        env!("CARGO_PKG_VERSION")
    );
    info!("📧 Support: SorrowsCry86@voidcat.org | 💰 CashApp: $WykeveTF");
    info!("🔍 Sentry monitoring active - VoidCat RDC Excellence Protocol");

    // Build and launch Tauri application
    let app_result = tauri::Builder::default()
        .setup(|app| {
            info!("⚙️ Initializing application systems...");

            // Initialize database with encryption
            match DatabaseManager::new(&app.handle()) {
                Ok(db_manager) => {
                    info!("✅ Database initialized with encryption");
                    let db_arc = Arc::new(db_manager);

                    // Initialize services layer
                    let services = Arc::new(Services::new(db_arc));
                    info!("✅ Services layer initialized");

                    // Set up application state
                    app.manage(AppState { services });
                    info!("✅ Application state configured");

                    info!("🎉 Forbidden Library ready - VoidCat RDC Excellence Protocol Active");
                    Ok(())
                }
                Err(e) => {
                    error!("❌ Database initialization failed: {}", e);
                    Err(Box::new(std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("Database setup failed: {}", e),
                    )))
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            // Basic application commands
            commands::greet,
            commands::get_app_version,
            commands::initialize_database,
            commands::get_database_stats,
            // Conversation management commands
            commands::create_conversation,
            commands::get_conversations,
            commands::get_conversation,
            commands::delete_conversation,
            commands::archive_conversation,
            // Message management commands
            commands::add_message,
            commands::get_messages,
            // Persona management commands
            commands::create_persona,
            commands::get_personas,
            commands::get_persona,
            commands::update_persona,
            commands::delete_persona,
            // API configuration commands
            commands::store_api_config,
            commands::get_api_config,
            commands::delete_api_config,
            // AI integration commands
            commands::send_ai_request,
            // File management commands
            commands::export_conversation,
            commands::backup_database,
            // Monitoring and testing commands
            commands::test_sentry,
            // Desktop-specific commands
            commands::get_system_info,
            commands::show_open_dialog,
            commands::show_save_dialog,
            commands::write_file_to_disk,
            commands::read_file_from_disk,
            commands::show_notification,
            commands::copy_to_clipboard,
            commands::read_from_clipboard,
            commands::get_app_data_dir,
            commands::open_external_url,
            commands::create_desktop_shortcut,
            commands::is_dark_mode,
            commands::get_window_state,
            commands::set_window_always_on_top,
            commands::minimize_to_tray,
            commands::check_for_updates,
            // AI Provider commands
            commands::check_ai_provider_availability,
            commands::list_ai_provider_models,
            commands::send_ai_provider_request,
        ])
        .run(tauri::generate_context!());

    // Finish tracking startup time
    PerformanceMonitor::finish_startup_tracking(startup_start_time, Some(&perf_config));

    // Handle application lifecycle
    match app_result {
        Ok(_) => {
            info!("✅ Forbidden Library terminated gracefully");
            Ok(())
        }
        Err(e) => {
            error!("❌ Application error: {}", e);
            Err(Box::new(e) as Box<dyn std::error::Error>)
        }
    }
}
