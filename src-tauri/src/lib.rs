// This file makes the crate a library so we can run unit tests
// Re-export the main modules for testing

pub mod commands;
pub mod database;
pub mod errors;
pub mod models;
pub mod services;
pub mod monitoring;
pub mod validation;
pub mod platform;
pub mod ai_providers;

pub use commands::*;
pub use database::*;
pub use errors::*;
pub use models::*;
pub use services::*;
pub use monitoring::*;
pub use validation::*;
pub use platform::*;
pub use ai_providers::*;
