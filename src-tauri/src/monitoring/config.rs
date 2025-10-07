//! Performance configuration for monitoring
//!
//! This module provides configuration options for performance monitoring,
//! including thresholds for various operations.

/// Configuration for performance monitoring thresholds
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Threshold for startup time in milliseconds (default: 1000ms)
    pub startup_threshold_ms: u64,

    /// Threshold for database operations in milliseconds (default: 50ms)
    pub database_threshold_ms: u64,

    /// Threshold for IPC commands in milliseconds (default: 100ms)
    pub ipc_threshold_ms: u64,

    /// Threshold for AI requests in milliseconds (default: 2000ms)
    pub ai_request_threshold_ms: u64,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            startup_threshold_ms: 1000,
            database_threshold_ms: 50,
            ipc_threshold_ms: 100,
            ai_request_threshold_ms: 2000,
        }
    }
}

impl PerformanceConfig {
    /// Create a new performance configuration with default values
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new performance configuration for development environment
    ///
    /// This configuration has higher thresholds to account for development
    /// environment overhead.
    pub fn development() -> Self {
        Self {
            startup_threshold_ms: 3000,
            database_threshold_ms: 200,
            ipc_threshold_ms: 300,
            ai_request_threshold_ms: 5000,
        }
    }

    /// Create a new performance configuration for production environment
    ///
    /// This configuration has stricter thresholds for production use.
    pub fn production() -> Self {
        Self {
            startup_threshold_ms: 800,
            database_threshold_ms: 30,
            ipc_threshold_ms: 80,
            ai_request_threshold_ms: 1500,
        }
    }

    /// Create a new builder for custom configuration
    pub fn builder() -> PerformanceConfigBuilder {
        PerformanceConfigBuilder::default()
    }
}

/// Builder for custom performance configuration
#[derive(Debug, Default)]
pub struct PerformanceConfigBuilder {
    startup_threshold_ms: Option<u64>,
    database_threshold_ms: Option<u64>,
    ipc_threshold_ms: Option<u64>,
    ai_request_threshold_ms: Option<u64>,
}

impl PerformanceConfigBuilder {
    /// Set the startup threshold in milliseconds
    pub fn startup_threshold_ms(mut self, ms: u64) -> Self {
        self.startup_threshold_ms = Some(ms);
        self
    }

    /// Set the database operation threshold in milliseconds
    pub fn database_threshold_ms(mut self, ms: u64) -> Self {
        self.database_threshold_ms = Some(ms);
        self
    }

    /// Set the IPC command threshold in milliseconds
    pub fn ipc_threshold_ms(mut self, ms: u64) -> Self {
        self.ipc_threshold_ms = Some(ms);
        self
    }

    /// Set the AI request threshold in milliseconds
    pub fn ai_request_threshold_ms(mut self, ms: u64) -> Self {
        self.ai_request_threshold_ms = Some(ms);
        self
    }

    /// Build the performance configuration
    pub fn build(self) -> PerformanceConfig {
        let default = PerformanceConfig::default();

        PerformanceConfig {
            startup_threshold_ms: self
                .startup_threshold_ms
                .unwrap_or(default.startup_threshold_ms),
            database_threshold_ms: self
                .database_threshold_ms
                .unwrap_or(default.database_threshold_ms),
            ipc_threshold_ms: self.ipc_threshold_ms.unwrap_or(default.ipc_threshold_ms),
            ai_request_threshold_ms: self
                .ai_request_threshold_ms
                .unwrap_or(default.ai_request_threshold_ms),
        }
    }
}
