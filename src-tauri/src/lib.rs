// This file makes the crate a library so we can run unit tests
// Re-export the main modules for testing

pub mod ai_providers;
pub mod commands;
pub mod database;
pub mod errors;
pub mod keychain;
pub mod models;
pub mod monitoring;
pub mod platform;
pub mod services;
pub mod validation;

pub use ai_providers::*;
pub use commands::*;
pub use database::*;
pub use errors::*;
pub use keychain::*;
pub use models::*;
pub use monitoring::*;
pub use platform::*;
pub use services::*;
pub use validation::*;
