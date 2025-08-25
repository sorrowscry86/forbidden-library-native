//! Error handling for monitoring
//!
//! This module provides standardized error types for monitoring operations.

use std::fmt;

/// Error type for monitoring operations
#[derive(Debug)]
pub enum MonitoringError<E> {
    /// Error from the operation being monitored
    Operation(E),
    
    /// Panic occurred during the operation
    Panic(String),
    
    /// Timeout occurred during the operation
    Timeout(String),
}

impl<E: fmt::Display> fmt::Display for MonitoringError<E> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Operation(e) => write!(f, "Operation error: {}", e),
            Self::Panic(msg) => write!(f, "Operation panicked: {}", msg),
            Self::Timeout(msg) => write!(f, "Operation timed out: {}", msg),
        }
    }
}

impl<E: std::error::Error + 'static> std::error::Error for MonitoringError<E> {}

/// Convert a string error to a monitoring error
impl From<String> for MonitoringError<String> {
    fn from(error: String) -> Self {
        MonitoringError::Operation(error)
    }
}

/// Convert a &str error to a monitoring error
impl From<&str> for MonitoringError<String> {
    fn from(error: &str) -> Self {
        MonitoringError::Operation(error.to_string())
    }
}

/// Convert a Box<dyn std::error::Error> to a monitoring error
impl From<Box<dyn std::error::Error>> for MonitoringError<Box<dyn std::error::Error>> {
    fn from(error: Box<dyn std::error::Error>) -> Self {
        MonitoringError::Operation(error)
    }
}
