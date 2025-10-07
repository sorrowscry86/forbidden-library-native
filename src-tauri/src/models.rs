//! Data models and structures
//!
//! This module defines all the data structures used throughout the Forbidden Library application.
//! Comprehensive models for conversations, personas, grimoire entries, and system configuration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core conversation model - Enhanced for native application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Conversation {
    pub id: Option<i64>,
    pub uuid: Uuid,
    pub title: String,
    pub persona_id: Option<i64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub archived: bool,
    pub metadata: Option<ConversationMetadata>,
}

/// Extended metadata for conversations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationMetadata {
    pub total_messages: i32,
    pub total_tokens: i32,
    pub last_model_used: Option<String>,
    pub average_response_time: Option<f64>,
    pub tags: Vec<String>,
    pub priority: ConversationPriority,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConversationPriority {
    Low,
    Normal,
    High,
    Critical,
}

impl Default for ConversationPriority {
    fn default() -> Self {
        ConversationPriority::Normal
    }
}

/// Individual message within a conversation - Enhanced for native application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: Option<i64>,
    pub conversation_id: i64,
    pub role: MessageRole,
    pub content: String,
    pub metadata: Option<MessageMetadata>,
    pub created_at: DateTime<Utc>,
    pub tokens_used: Option<i32>,
    pub model_used: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    User,
    Assistant,
    System,
}

/// Enhanced message metadata for native application
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub processing_time_ms: Option<i64>,
    pub confidence_score: Option<f32>,
    pub flagged_content: bool,
    pub attachments: Vec<MessageAttachment>,
    pub legacy_metadata: Option<serde_json::Value>, // For migration compatibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageAttachment {
    pub id: String,
    pub filename: String,
    pub file_type: String,
    pub size_bytes: i64,
    pub file_path: String,
}

/// Persona model - Enhanced AI character definitions
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Persona {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub system_prompt: String,
    pub avatar_path: Option<String>,
    pub memory_context: Option<serde_json::Value>,
    pub settings: Option<PersonaSettings>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaSettings {
    pub preferred_model: Option<String>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub response_style: ResponseStyle,
    pub expertise_domains: Vec<String>,
    pub personality_traits: Vec<PersonalityTrait>,
    pub legacy_settings: Option<serde_json::Value>, // For migration compatibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseStyle {
    Concise,
    Detailed,
    Creative,
    Technical,
    Conversational,
    Formal,
}

impl Default for ResponseStyle {
    fn default() -> Self {
        ResponseStyle::Conversational
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonalityTrait {
    pub trait_name: String,
    pub intensity: f32, // 0.0 to 1.0
}

/// Enhanced Grimoire model - Knowledge base and MCP server management
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Grimoire {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub server_path: String,
    pub configuration: Option<GrimoireConfiguration>,
    pub enabled: bool,
    pub created_at: DateTime<Utc>,
    pub accessed_count: i32,
    pub last_accessed: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrimoireConfiguration {
    pub server_type: GrimoireServerType,
    pub connection_settings: ConnectionSettings,
    pub capabilities: Vec<GrimoireCapability>,
    pub metadata: std::collections::HashMap<String, serde_json::Value>,
    pub legacy_configuration: Option<serde_json::Value>, // For migration compatibility
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrimoireServerType {
    MCP,
    HTTP,
    WebSocket,
    Local,
    Plugin,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionSettings {
    pub host: Option<String>,
    pub port: Option<u16>,
    pub authentication: Option<AuthenticationConfig>,
    pub timeout_ms: Option<u32>,
    pub retry_attempts: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthenticationConfig {
    pub auth_type: AuthType,
    pub credentials: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthType {
    None,
    ApiKey,
    Bearer,
    Basic,
    OAuth2,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GrimoireCapability {
    ResourceAccess,
    ToolExecution,
    FileManagement,
    DatabaseAccess,
    NetworkRequest,
    SystemIntegration,
}

/// Enhanced API provider model for multiple AI services
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ApiProvider {
    pub id: Option<i64>,
    pub name: String,
    pub provider_type: ApiProviderType,
    pub base_url: String,
    pub api_key: Option<String>, // Encrypted in storage
    pub model_list: Vec<String>,
    pub capabilities: ModelCapabilities,
    pub rate_limits: Option<RateLimits>,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiProviderType {
    OpenAI,
    Anthropic,
    Google,
    Cohere,
    HuggingFace,
    LocalLM,
    Custom(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelCapabilities {
    pub supports_vision: bool,
    pub supports_function_calling: bool,
    pub supports_streaming: bool,
    pub context_window: u32,
    pub max_tokens: u32,
    pub supports_system_messages: bool,
    pub supports_tool_use: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RateLimits {
    pub requests_per_minute: Option<i32>,
    pub tokens_per_minute: Option<i32>,
    pub requests_per_day: Option<i32>,
    pub tokens_per_day: Option<i32>,
}

/// Project model - Development project tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub description: Option<String>,
    pub repository_url: Option<String>,
    pub status: ProjectStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: Option<ProjectMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Active,
    Paused,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMetadata {
    pub technology_stack: Vec<String>,
    pub team_members: Vec<String>,
    pub milestones: Vec<ProjectMilestone>,
    pub repository_stats: Option<RepositoryStats>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMilestone {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub completed: bool,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepositoryStats {
    pub total_commits: i32,
    pub total_lines: i32,
    pub languages: std::collections::HashMap<String, i32>,
    pub last_commit_date: DateTime<Utc>,
    pub contributors: Vec<String>,
}

// Utility implementations for model creation
impl Conversation {
    pub fn new(title: String, persona_id: Option<i64>) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            uuid: Uuid::new_v4(),
            title,
            persona_id,
            created_at: now,
            updated_at: now,
            archived: false,
            metadata: None,
        }
    }
}

impl Message {
    pub fn new(conversation_id: i64, role: MessageRole, content: String) -> Self {
        Self {
            id: None,
            conversation_id,
            role,
            content,
            metadata: None,
            created_at: Utc::now(),
            tokens_used: None,
            model_used: None,
        }
    }
}

impl Persona {
    pub fn new(name: String, description: Option<String>, system_prompt: String) -> Self {
        let now = Utc::now();
        Self {
            id: None,
            name,
            description,
            system_prompt,
            avatar_path: None,
            memory_context: None,
            settings: None,
            created_at: now,
            updated_at: now,
            active: true,
        }
    }
}

impl Grimoire {
    pub fn new(name: String, description: Option<String>, server_path: String) -> Self {
        Self {
            id: None,
            name,
            description,
            server_path,
            configuration: None,
            enabled: true,
            created_at: Utc::now(),
            accessed_count: 0,
            last_accessed: None,
        }
    }
}
