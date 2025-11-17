//! Data models and structures
//!
//! This module defines all the data structures used throughout the Forbidden Library application.
//! Comprehensive models for conversations, personas, grimoire entries, and system configuration.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Core conversation model - Enhanced for native application
///
/// Represents a single conversation thread between the user and an AI assistant.
/// Each conversation can be associated with a specific persona for customized interactions.
///
/// # Fields
///
/// * `id` - Optional database identifier (None for new conversations)
/// * `uuid` - Universally unique identifier for cross-platform synchronization
/// * `title` - User-facing title of the conversation
/// * `persona_id` - Optional reference to the persona used in this conversation
/// * `created_at` - Timestamp when the conversation was created
/// * `updated_at` - Timestamp of the last modification
/// * `archived` - Whether the conversation is archived (hidden from main view)
/// * `metadata` - Optional extended metadata for analytics and tracking
///
/// # Examples
///
/// ```
/// use forbidden_library_native::models::Conversation;
///
/// let conversation = Conversation::new("My First Chat".to_string(), None);
/// assert_eq!(conversation.title, "My First Chat");
/// assert!(!conversation.archived);
/// ```
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
///
/// Represents a single message in a conversation thread. Messages can be from
/// the user, the AI assistant, or the system (for prompts and context).
///
/// # Fields
///
/// * `id` - Optional database identifier
/// * `conversation_id` - Foreign key reference to the parent conversation
/// * `role` - Who sent the message (User, Assistant, or System)
/// * `content` - The actual message text
/// * `metadata` - Optional extended metadata for tracking and analytics
/// * `created_at` - Timestamp when the message was created
/// * `tokens_used` - Number of tokens consumed by this message (for cost tracking)
/// * `model_used` - AI model that generated this response (for assistant messages)
///
/// # Examples
///
/// ```
/// use forbidden_library_native::models::{Message, MessageRole};
///
/// let message = Message::new(1, MessageRole::User, "Hello!".to_string());
/// assert_eq!(message.conversation_id, 1);
/// assert_eq!(message.role, MessageRole::User);
/// ```
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

/// Role of the message sender in a conversation
///
/// Determines who created the message and how it should be processed
/// and displayed in the UI.
///
/// # Variants
///
/// * `User` - Message from the human user
/// * `Assistant` - Response from the AI assistant
/// * `System` - System-generated message (prompts, context, instructions)
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
///
/// Represents a customizable AI character with specific behavior, personality,
/// and settings. Personas allow users to create specialized assistants for
/// different tasks and contexts.
///
/// # Fields
///
/// * `id` - Optional database identifier
/// * `name` - Display name of the persona
/// * `description` - Optional human-readable description of the persona's purpose
/// * `system_prompt` - The core instruction that defines the persona's behavior
/// * `avatar_path` - Optional file path to the persona's avatar image
/// * `memory_context` - Optional JSON object for persona memory and context
/// * `settings` - Optional persona-specific settings (temperature, model, etc.)
/// * `created_at` - Timestamp when the persona was created
/// * `updated_at` - Timestamp of the last modification
/// * `active` - Whether this persona is currently active and usable
///
/// # Examples
///
/// ```
/// use forbidden_library_native::models::Persona;
///
/// let persona = Persona::new(
///     "Coding Assistant".to_string(),
///     Some("Helps with programming tasks".to_string()),
///     "You are an expert software engineer.".to_string()
/// );
/// assert_eq!(persona.name, "Coding Assistant");
/// assert!(persona.active);
/// ```
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
    /// Create a new conversation with the given title and optional persona
    ///
    /// Initializes a new conversation with generated UUID, current timestamps,
    /// and default values for archived status and metadata.
    ///
    /// # Arguments
    ///
    /// * `title` - The display title for the conversation
    /// * `persona_id` - Optional persona to associate with this conversation
    ///
    /// # Returns
    ///
    /// A new `Conversation` instance ready to be saved to the database
    ///
    /// # Examples
    ///
    /// ```
    /// use forbidden_library_native::models::Conversation;
    ///
    /// // Create conversation without persona
    /// let conv1 = Conversation::new("General Chat".to_string(), None);
    ///
    /// // Create conversation with persona
    /// let conv2 = Conversation::new("Coding Help".to_string(), Some(5));
    /// assert_eq!(conv2.persona_id, Some(5));
    /// ```
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
    /// Create a new message for a conversation
    ///
    /// Initializes a new message with the current timestamp and default values
    /// for optional fields like metadata, tokens, and model.
    ///
    /// # Arguments
    ///
    /// * `conversation_id` - ID of the parent conversation
    /// * `role` - Who is sending this message (User, Assistant, or System)
    /// * `content` - The message text content
    ///
    /// # Returns
    ///
    /// A new `Message` instance ready to be saved to the database
    ///
    /// # Examples
    ///
    /// ```
    /// use forbidden_library_native::models::{Message, MessageRole};
    ///
    /// let user_msg = Message::new(
    ///     1,
    ///     MessageRole::User,
    ///     "What is Rust?".to_string()
    /// );
    ///
    /// let assistant_msg = Message::new(
    ///     1,
    ///     MessageRole::Assistant,
    ///     "Rust is a systems programming language.".to_string()
    /// );
    /// ```
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
    /// Create a new persona with the given name, description, and system prompt
    ///
    /// Initializes a new persona with current timestamps and default active status.
    /// The system prompt defines the persona's behavior and personality.
    ///
    /// # Arguments
    ///
    /// * `name` - The display name for the persona
    /// * `description` - Optional human-readable description of the persona's purpose
    /// * `system_prompt` - The core instruction that defines the persona's behavior
    ///
    /// # Returns
    ///
    /// A new `Persona` instance ready to be saved to the database
    ///
    /// # Examples
    ///
    /// ```
    /// use forbidden_library_native::models::Persona;
    ///
    /// let coding_assistant = Persona::new(
    ///     "Code Helper".to_string(),
    ///     Some("Assists with programming and debugging".to_string()),
    ///     "You are an expert software engineer specializing in Rust.".to_string()
    /// );
    /// assert_eq!(coding_assistant.name, "Code Helper");
    /// assert!(coding_assistant.active);
    /// ```
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
