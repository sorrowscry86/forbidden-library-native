//! Performance monitoring and error tracking
//! 
//! VoidCat RDC production monitoring system with Sentry integration
//! Provides comprehensive tracking for startup time, database operations,
//! and IPC command performance.

mod config;
mod transactions;
mod error_handling;

#[cfg(test)]
mod tests;

pub use config::PerformanceConfig;
pub use transactions::*;
pub use error_handling::*;

use sentry::{add_breadcrumb, Breadcrumb, start_transaction, TransactionContext};
use sentry::protocol::Value;
use tracing::{info, error, warn, instrument};
use std::time::Instant;
use std::panic::{self, AssertUnwindSafe};

/// Performance monitoring utilities for VoidCat RDC
pub struct PerformanceMonitor;

impl PerformanceMonitor {
    /// Start tracking application startup time
    #[instrument]
    pub fn start_startup_tracking() -> Instant {
        info!("🎯 Starting performance monitoring - VoidCat RDC Excellence Protocol");
        Instant::now()
    }

    /// Finish tracking application startup time and report slow startups
    #[instrument]
    pub fn finish_startup_tracking(start_time: Instant, config: Option<&PerformanceConfig>) {
        let transaction = start_transaction(
            TransactionContext::new("app.startup", "app.startup"),
        );
        
        let duration = start_time.elapsed();
        let threshold = config.map_or(1000, |c| c.startup_threshold_ms);
        
        if duration.as_millis() > threshold as u128 {
            error!("❌ Startup time exceeded {} ms: {}ms", threshold, duration.as_millis());
            sentry::capture_message(
                &format!("Slow startup: {}ms - VoidCat RDC Performance Alert", duration.as_millis()),
                sentry::Level::Error,
            );
        } else {
            info!("✅ Startup time: {}ms - VoidCat RDC Performance Standard", duration.as_millis());
        }

        transaction.finish();
    }

    /// Track database operations with performance monitoring
    #[instrument(skip(f))]
    pub fn track_database_operation<F, T, E>(
        operation: &str, 
        f: F,
        config: Option<&PerformanceConfig>
    ) -> Result<T, MonitoringError<E>>
    where
        F: FnOnce() -> Result<T, E>,
        E: std::error::Error + 'static,
    {
        let transaction = start_transaction(
            TransactionContext::new(
                &format!("db.{}", operation),
                "db.operation"
            ),
        );

        add_breadcrumb(Breadcrumb {
            message: Some(format!("Database operation: {}", operation)),
            category: Some("database".to_string()),
            level: sentry::Level::Info,
            ..Default::default()
        });

        let start_time = Instant::now();
        
        let result = panic::catch_unwind(AssertUnwindSafe(|| f()))
            .map_err(|panic_err| {
                let panic_msg = if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = panic_err.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic".to_string()
                };
                
                error!("❌ Database operation panicked: {} - {}", operation, panic_msg);
                sentry::capture_message(
                    &format!("Database operation panicked: {} - {}", operation, panic_msg),
                    sentry::Level::Error,
                );
                
                MonitoringError::Panic(panic_msg)
            })
            .and_then(|res| res.map_err(MonitoringError::Operation));
            
        let duration = start_time.elapsed();

        let threshold = config.map_or(50, |c| c.database_threshold_ms);
        if duration.as_millis() > threshold as u128 {
            warn!("⚠️ Slow database operation {}: {}ms", operation, duration.as_millis());
            sentry::add_breadcrumb(Breadcrumb {
                message: Some(format!("Slow database operation: {} ({}ms)", operation, duration.as_millis())),
                category: Some("performance".to_string()),
                level: sentry::Level::Warning,
                ..Default::default()
            });
        }

        transaction.finish();
        result
    }

    /// Track IPC command performance
    #[instrument(skip(f))]
    pub fn track_ipc_command<F, T, E>(
        command: &str, 
        f: F,
        config: Option<&PerformanceConfig>
    ) -> Result<T, MonitoringError<E>>
    where
        F: FnOnce() -> Result<T, E>,
        E: std::fmt::Display + 'static,
    {
        let transaction = start_transaction(
            TransactionContext::new(
                &format!("ipc.{}", command),
                "ipc.command"
            ),
        );

        add_breadcrumb(Breadcrumb {
            message: Some(format!("IPC command: {}", command)),
            category: Some("ipc".to_string()),
            level: sentry::Level::Info,
            ..Default::default()
        });

        let start_time = Instant::now();
        
        let result = panic::catch_unwind(AssertUnwindSafe(|| f()))
            .map_err(|panic_err| {
                let panic_msg = if let Some(s) = panic_err.downcast_ref::<String>() {
                    s.clone()
                } else if let Some(s) = panic_err.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "Unknown panic".to_string()
                };
                
                error!("❌ IPC command panicked: {} - {}", command, panic_msg);
                sentry::capture_message(
                    &format!("IPC command panicked: {} - {}", command, panic_msg),
                    sentry::Level::Error,
                );
                
                MonitoringError::Panic(panic_msg)
            })
            .and_then(|res| res.map_err(|e| MonitoringError::Operation(e)));
            
        let duration = start_time.elapsed();

        let threshold = config.map_or(100, |c| c.ipc_threshold_ms);
        if duration.as_millis() > threshold as u128 {
            warn!("⚠️ Slow IPC command {}: {}ms", command, duration.as_millis());
        }

        match &result {
            Ok(_) => info!("✅ IPC command successful: {} ({}ms)", command, duration.as_millis()),
            Err(e) => {
                error!("❌ IPC command failed: {} - {}", command, e);
                sentry::capture_message(
                    &format!("IPC command failed: {} - {}", command, e),
                    sentry::Level::Error,
                );
            }
        }

        transaction.finish();
        result
    }

    /// Track AI request performance and token usage
    pub fn track_ai_request<E: std::fmt::Display>(
        model: &str, 
        tokens_used: Option<i32>, 
        status: Result<(), E>
    ) {
        add_breadcrumb(Breadcrumb {
            message: Some(format!("AI request to model: {}", model)),
            category: Some("ai".to_string()),
            level: sentry::Level::Info,
            data: {
                let mut data = std::collections::BTreeMap::new();
                data.insert("model".to_string(), Value::String(model.to_string()));
                if let Some(tokens) = tokens_used {
                    data.insert("tokens_used".to_string(), Value::Number(tokens.into()));
                }
                data
            },
            ..Default::default()
        });

        match status {
            Ok(_) => {
                if let Some(tokens) = tokens_used {
                    info!("🤖 AI request completed: {} model, {} tokens", model, tokens);
                } else {
                    info!("🤖 AI request completed: {} model", model);
                }
            },
            Err(e) => {
                error!("❌ AI request failed: {} model - {}", model, e);
                sentry::capture_message(
                    &format!("AI request failed: {} - {}", model, e),
                    sentry::Level::Error,
                );
            }
        }
    }
}

/// Test Sentry integration with error reporting
pub fn test_sentry_integration() -> Result<(), String> {
    info!("🧪 Testing Sentry integration - VoidCat RDC");

    sentry::capture_message(
        "Test message from Forbidden Library - VoidCat RDC",
        sentry::Level::Info
    );

    add_breadcrumb(Breadcrumb {
        message: Some("Sentry integration test completed".to_string()),
        category: Some("test".to_string()),
        level: sentry::Level::Info,
        ..Default::default()
    });

    Ok(())
}