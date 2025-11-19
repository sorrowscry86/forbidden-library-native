//! AI Provider integrations for Forbidden Library
//!
//! This module provides integrations with various AI providers including:
//! - OpenAI (GPT-4, GPT-3.5, etc.)
//! - Anthropic Claude (Claude 3.5 Sonnet, Opus, Haiku)
//! - Google Gemini (Gemini 1.5 Pro, Flash)
//! - Azure OpenAI
//! - LM Studio (local models)
//! - Ollama (local models)
//! - OpenAI API compatible endpoints

use crate::errors::{AppError, AppResult};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// AI Provider variants with their specific configurations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    /// OpenAI official API (GPT-4, GPT-3.5-turbo, etc.)
    OpenAI {
        api_key: String,
        organization: Option<String>,
    },
    /// Anthropic Claude API (Claude 3.5 Sonnet, Opus, Haiku)
    Anthropic {
        api_key: String,
    },
    /// Google Gemini API (Gemini 1.5 Pro, Flash)
    GoogleGemini {
        api_key: String,
    },
    /// Azure OpenAI Service
    AzureOpenAI {
        api_key: String,
        endpoint: String,
        deployment_name: String,
        api_version: String,
    },
    /// LM Studio (local models)
    LMStudio {
        base_url: String,
    },
    /// Ollama (local models)
    Ollama {
        base_url: String,
    },
    /// Generic OpenAI-compatible endpoint
    OpenAICompatible {
        base_url: String,
        api_key: Option<String>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<i32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIResponse {
    pub content: String,
    pub model: String,
    pub tokens_used: Option<i32>,
}

impl AIProvider {
    /// Create a new OpenAI provider
    pub fn openai(api_key: String, organization: Option<String>) -> Self {
        AIProvider::OpenAI {
            api_key,
            organization,
        }
    }

    /// Create a new Anthropic Claude provider
    pub fn anthropic(api_key: String) -> Self {
        AIProvider::Anthropic { api_key }
    }

    /// Create a new Google Gemini provider
    pub fn google_gemini(api_key: String) -> Self {
        AIProvider::GoogleGemini { api_key }
    }

    /// Create a new Azure OpenAI provider
    pub fn azure_openai(
        api_key: String,
        endpoint: String,
        deployment_name: String,
        api_version: Option<String>,
    ) -> Self {
        AIProvider::AzureOpenAI {
            api_key,
            endpoint,
            deployment_name,
            api_version: api_version.unwrap_or_else(|| "2024-02-15-preview".to_string()),
        }
    }

    /// Create a new LM Studio provider
    pub fn lm_studio(port: Option<u16>) -> Self {
        let port = port.unwrap_or(1234);
        AIProvider::LMStudio {
            base_url: format!("http://localhost:{}", port),
        }
    }

    /// Create a new Ollama provider
    pub fn ollama(port: Option<u16>) -> Self {
        let port = port.unwrap_or(11434);
        AIProvider::Ollama {
            base_url: format!("http://localhost:{}", port),
        }
    }

    /// Create a new OpenAI-compatible provider
    pub fn openai_compatible(base_url: String, api_key: Option<String>) -> Self {
        AIProvider::OpenAICompatible { base_url, api_key }
    }

    /// Send a request to the AI provider
    pub async fn send_request(&self, request: AIRequest) -> AppResult<AIResponse> {
        match self {
            AIProvider::OpenAI {
                api_key,
                organization,
            } => {
                Self::send_openai_request(api_key, organization.clone(), request).await
            }
            AIProvider::Anthropic { api_key } => {
                Self::send_anthropic_request(api_key, request).await
            }
            AIProvider::GoogleGemini { api_key } => {
                Self::send_gemini_request(api_key, request).await
            }
            AIProvider::AzureOpenAI {
                api_key,
                endpoint,
                deployment_name,
                api_version,
            } => {
                Self::send_azure_request(api_key, endpoint, deployment_name, api_version, request)
                    .await
            }
            AIProvider::LMStudio { base_url } => {
                Self::send_openai_compatible_request(base_url, None, request).await
            }
            AIProvider::Ollama { base_url } => Self::send_ollama_request(base_url, request).await,
            AIProvider::OpenAICompatible { base_url, api_key } => {
                Self::send_openai_compatible_request(base_url, api_key.clone(), request).await
            }
        }
    }

    /// Send request to OpenAI official API
    async fn send_openai_request(
        api_key: &str,
        organization: Option<String>,
        request: AIRequest,
    ) -> AppResult<AIResponse> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| AppError::api(format!("Failed to create HTTP client: {}", e)))?;

        let url = "https://api.openai.com/v1/chat/completions";

        let mut req_builder = client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json");

        if let Some(org) = organization {
            req_builder = req_builder.header("OpenAI-Organization", org);
        }

        req_builder = req_builder.json(&serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens,
            "stream": request.stream,
        }));

        let response = req_builder
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to send OpenAI request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::api(format!(
                "OpenAI API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse OpenAI response: {}", e)))?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::api("Invalid OpenAI response format"))?
            .to_string();

        let tokens_used = response_json["usage"]["total_tokens"]
            .as_i64()
            .map(|t| t as i32);

        Ok(AIResponse {
            content,
            model: request.model,
            tokens_used,
        })
    }

    /// Send request to Anthropic Claude API
    async fn send_anthropic_request(
        api_key: &str,
        request: AIRequest,
    ) -> AppResult<AIResponse> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| AppError::api(format!("Failed to create HTTP client: {}", e)))?;

        let url = "https://api.anthropic.com/v1/messages";

        // Convert messages to Anthropic format (extract system message if present)
        let mut system_message = None;
        let mut messages_without_system = Vec::new();

        for msg in request.messages {
            if msg.role == "system" {
                system_message = Some(msg.content);
            } else {
                messages_without_system.push(msg);
            }
        }

        let mut body = serde_json::json!({
            "model": request.model,
            "messages": messages_without_system,
            "max_tokens": request.max_tokens.unwrap_or(4096),
        });

        if let Some(system) = system_message {
            body["system"] = serde_json::json!(system);
        }

        if let Some(temp) = request.temperature {
            body["temperature"] = serde_json::json!(temp);
        }

        let response = client
            .post(url)
            .header("x-api-key", api_key)
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to send Anthropic request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::api(format!(
                "Anthropic API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse Anthropic response: {}", e)))?;

        let content = response_json["content"][0]["text"]
            .as_str()
            .ok_or_else(|| AppError::api("Invalid Anthropic response format"))?
            .to_string();

        let tokens_used = response_json["usage"]["output_tokens"]
            .as_i64()
            .and_then(|output| {
                response_json["usage"]["input_tokens"]
                    .as_i64()
                    .map(|input| (output + input) as i32)
            });

        Ok(AIResponse {
            content,
            model: request.model,
            tokens_used,
        })
    }

    /// Send request to Google Gemini API
    async fn send_gemini_request(api_key: &str, request: AIRequest) -> AppResult<AIResponse> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| AppError::api(format!("Failed to create HTTP client: {}", e)))?;

        // Gemini uses a different URL pattern
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            request.model, api_key
        );

        // Convert messages to Gemini format
        let mut contents = Vec::new();
        for msg in request.messages {
            let role = if msg.role == "assistant" {
                "model"
            } else {
                "user"
            };
            contents.push(serde_json::json!({
                "role": role,
                "parts": [{"text": msg.content}]
            }));
        }

        let mut body = serde_json::json!({
            "contents": contents,
        });

        if let Some(temp) = request.temperature {
            body["generationConfig"] = serde_json::json!({
                "temperature": temp,
            });
        }

        if let Some(max_tokens) = request.max_tokens {
            if body["generationConfig"].is_null() {
                body["generationConfig"] = serde_json::json!({});
            }
            body["generationConfig"]["maxOutputTokens"] = serde_json::json!(max_tokens);
        }

        let response = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to send Gemini request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::api(format!(
                "Gemini API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse Gemini response: {}", e)))?;

        let content = response_json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| AppError::api("Invalid Gemini response format"))?
            .to_string();

        let tokens_used = response_json["usageMetadata"]["totalTokenCount"]
            .as_i64()
            .map(|t| t as i32);

        Ok(AIResponse {
            content,
            model: request.model,
            tokens_used,
        })
    }

    /// Send request to Azure OpenAI
    async fn send_azure_request(
        api_key: &str,
        endpoint: &str,
        deployment_name: &str,
        api_version: &str,
        request: AIRequest,
    ) -> AppResult<AIResponse> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(120))
            .build()
            .map_err(|e| AppError::api(format!("Failed to create HTTP client: {}", e)))?;

        let url = format!(
            "{}/openai/deployments/{}/chat/completions?api-version={}",
            endpoint.trim_end_matches('/'),
            deployment_name,
            api_version
        );

        let response = client
            .post(&url)
            .header("api-key", api_key)
            .header("Content-Type", "application/json")
            .json(&serde_json::json!({
                "messages": request.messages,
                "temperature": request.temperature.unwrap_or(0.7),
                "max_tokens": request.max_tokens,
                "stream": request.stream,
            }))
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to send Azure OpenAI request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::api(format!(
                "Azure OpenAI API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse Azure OpenAI response: {}", e)))?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::api("Invalid Azure OpenAI response format"))?
            .to_string();

        let tokens_used = response_json["usage"]["total_tokens"]
            .as_i64()
            .map(|t| t as i32);

        Ok(AIResponse {
            content,
            model: deployment_name.to_string(),
            tokens_used,
        })
    }

    /// Send request to OpenAI-compatible endpoint (LM Studio, OpenAI, etc.)
    async fn send_openai_compatible_request(
        base_url: &str,
        api_key: Option<String>,
        request: AIRequest,
    ) -> AppResult<AIResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/chat/completions", base_url);

        let mut req_builder = client.post(&url).json(&serde_json::json!({
            "model": request.model,
            "messages": request.messages,
            "temperature": request.temperature.unwrap_or(0.7),
            "max_tokens": request.max_tokens,
            "stream": request.stream,
        }));

        if let Some(key) = api_key {
            req_builder = req_builder.header("Authorization", format!("Bearer {}", key));
        }

        let response = req_builder
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to send request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::api(format!(
                "API request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse response: {}", e)))?;

        let content = response_json["choices"][0]["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::api("Invalid response format"))?
            .to_string();

        let tokens_used = response_json["usage"]["total_tokens"]
            .as_i64()
            .map(|t| t as i32);

        Ok(AIResponse {
            content,
            model: request.model,
            tokens_used,
        })
    }

    /// Send request to Ollama endpoint
    async fn send_ollama_request(base_url: &str, request: AIRequest) -> AppResult<AIResponse> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/chat", base_url);

        let response = client
            .post(&url)
            .json(&serde_json::json!({
                "model": request.model,
                "messages": request.messages,
                "stream": request.stream,
                "options": {
                    "temperature": request.temperature.unwrap_or(0.7),
                    "num_predict": request.max_tokens,
                }
            }))
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to send Ollama request: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(AppError::api(format!(
                "Ollama request failed with status {}: {}",
                status, error_text
            )));
        }

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse Ollama response: {}", e)))?;

        let content = response_json["message"]["content"]
            .as_str()
            .ok_or_else(|| AppError::api("Invalid Ollama response format"))?
            .to_string();

        Ok(AIResponse {
            content,
            model: request.model,
            tokens_used: None, // Ollama doesn't provide token counts in the same way
        })
    }

    /// Check if the provider is available
    pub async fn check_availability(&self) -> AppResult<bool> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|e| AppError::api(format!("Failed to create HTTP client: {}", e)))?;

        match self {
            AIProvider::OpenAI { api_key, .. } => {
                let response = client
                    .get("https://api.openai.com/v1/models")
                    .header("Authorization", format!("Bearer {}", api_key))
                    .send()
                    .await;
                Ok(response.map(|r| r.status().is_success()).unwrap_or(false))
            }
            AIProvider::Anthropic { api_key } => {
                let response = client
                    .get("https://api.anthropic.com/v1/messages")
                    .header("x-api-key", api_key)
                    .header("anthropic-version", "2023-06-01")
                    .send()
                    .await;
                // Anthropic returns 400 for GET on messages endpoint, but API key is validated
                Ok(response.is_ok())
            }
            AIProvider::GoogleGemini { api_key } => {
                let response = client
                    .get(format!(
                        "https://generativelanguage.googleapis.com/v1beta/models?key={}",
                        api_key
                    ))
                    .send()
                    .await;
                Ok(response.map(|r| r.status().is_success()).unwrap_or(false))
            }
            AIProvider::AzureOpenAI { endpoint, .. } => {
                // Simple connectivity check
                let response = client.get(endpoint).send().await;
                Ok(response.is_ok())
            }
            AIProvider::LMStudio { base_url } => {
                let response = client
                    .get(format!("{}/v1/models", base_url))
                    .send()
                    .await;
                Ok(response.map(|r| r.status().is_success()).unwrap_or(false))
            }
            AIProvider::Ollama { base_url } => {
                let response = client
                    .get(format!("{}/api/tags", base_url))
                    .send()
                    .await;
                Ok(response.map(|r| r.status().is_success()).unwrap_or(false))
            }
            AIProvider::OpenAICompatible { base_url, .. } => {
                let response = client
                    .get(format!("{}/v1/models", base_url))
                    .send()
                    .await;
                Ok(response.map(|r| r.status().is_success()).unwrap_or(false))
            }
        }
    }

    /// List available models from the provider
    pub async fn list_models(&self) -> AppResult<Vec<String>> {
        match self {
            AIProvider::OpenAI { api_key, .. } => {
                Self::list_openai_official_models(api_key).await
            }
            AIProvider::Anthropic { .. } => {
                // Anthropic models are known/fixed list
                Ok(vec![
                    "claude-3-5-sonnet-20241022".to_string(),
                    "claude-3-5-haiku-20241022".to_string(),
                    "claude-3-opus-20240229".to_string(),
                    "claude-3-sonnet-20240229".to_string(),
                    "claude-3-haiku-20240307".to_string(),
                ])
            }
            AIProvider::GoogleGemini { api_key } => {
                Self::list_gemini_models(api_key).await
            }
            AIProvider::AzureOpenAI { deployment_name, .. } => {
                // Azure deployments are configured, return the deployment name
                Ok(vec![deployment_name.clone()])
            }
            AIProvider::LMStudio { base_url } | AIProvider::OpenAICompatible { base_url, .. } => {
                Self::list_openai_compatible_models(base_url).await
            }
            AIProvider::Ollama { base_url } => Self::list_ollama_models(base_url).await,
        }
    }

    async fn list_openai_official_models(api_key: &str) -> AppResult<Vec<String>> {
        let client = reqwest::Client::new();
        let url = "https://api.openai.com/v1/models";

        let response = client
            .get(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to list OpenAI models: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse OpenAI models response: {}", e)))?;

        let models = response_json["data"]
            .as_array()
            .ok_or_else(|| AppError::api("Invalid OpenAI models response format"))?
            .iter()
            .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
            .filter(|id| id.starts_with("gpt-")) // Only show GPT models
            .collect();

        Ok(models)
    }

    async fn list_gemini_models(api_key: &str) -> AppResult<Vec<String>> {
        let client = reqwest::Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models?key={}",
            api_key
        );

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to list Gemini models: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse Gemini models response: {}", e)))?;

        let models = response_json["models"]
            .as_array()
            .ok_or_else(|| AppError::api("Invalid Gemini models response format"))?
            .iter()
            .filter_map(|m| {
                m["name"].as_str().and_then(|name| {
                    // Convert "models/gemini-1.5-pro" to "gemini-1.5-pro"
                    name.strip_prefix("models/").map(|s| s.to_string())
                })
            })
            .collect();

        Ok(models)
    }

    async fn list_openai_compatible_models(base_url: &str) -> AppResult<Vec<String>> {
        let client = reqwest::Client::new();
        let url = format!("{}/v1/models", base_url);

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to list models: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse models response: {}", e)))?;

        let models = response_json["data"]
            .as_array()
            .ok_or_else(|| AppError::api("Invalid models response format"))?
            .iter()
            .filter_map(|m| m["id"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(models)
    }

    async fn list_ollama_models(base_url: &str) -> AppResult<Vec<String>> {
        let client = reqwest::Client::new();
        let url = format!("{}/api/tags", base_url);

        let response = client
            .get(&url)
            .send()
            .await
            .map_err(|e| AppError::api(format!("Failed to list Ollama models: {}", e)))?;

        let response_json: serde_json::Value = response
            .json()
            .await
            .map_err(|e| AppError::api(format!("Failed to parse Ollama models response: {}", e)))?;

        let models = response_json["models"]
            .as_array()
            .ok_or_else(|| AppError::api("Invalid Ollama models response format"))?
            .iter()
            .filter_map(|m| m["name"].as_str().map(|s| s.to_string()))
            .collect();

        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_provider_creation() {
        let lm_studio = AIProvider::lm_studio(None);
        assert!(matches!(lm_studio, AIProvider::LMStudio { .. }));

        let ollama = AIProvider::ollama(Some(11434));
        assert!(matches!(ollama, AIProvider::Ollama { .. }));
    }

    #[tokio::test]
    async fn test_availability_check() {
        let provider = AIProvider::lm_studio(Some(1234));
        // This will fail in CI, but that's expected
        let _ = provider.check_availability().await;
    }
}
