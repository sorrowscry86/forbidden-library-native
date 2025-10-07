//! AI Provider integrations for Forbidden Library
//!
//! This module provides integrations with various AI providers including:
//! - LM Studio (local models)
//! - Ollama (local models)
//! - OpenAI API compatible endpoints

use serde::{Deserialize, Serialize};
use crate::errors::{AppError, AppResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AIProvider {
    LMStudio { base_url: String },
    Ollama { base_url: String },
    OpenAICompatible { base_url: String, api_key: Option<String> },
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
            AIProvider::LMStudio { base_url } => {
                Self::send_openai_compatible_request(base_url, None, request).await
            }
            AIProvider::Ollama { base_url } => {
                Self::send_ollama_request(base_url, request).await
            }
            AIProvider::OpenAICompatible { base_url, api_key } => {
                Self::send_openai_compatible_request(base_url, api_key.clone(), request).await
            }
        }
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
    async fn send_ollama_request(
        base_url: &str,
        request: AIRequest,
    ) -> AppResult<AIResponse> {
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
        let url = match self {
            AIProvider::LMStudio { base_url } => format!("{}/v1/models", base_url),
            AIProvider::Ollama { base_url } => format!("{}/api/tags", base_url),
            AIProvider::OpenAICompatible { base_url, .. } => format!("{}/v1/models", base_url),
        };

        let client = reqwest::Client::new();
        match client.get(&url).send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(_) => Ok(false),
        }
    }

    /// List available models from the provider
    pub async fn list_models(&self) -> AppResult<Vec<String>> {
        match self {
            AIProvider::LMStudio { base_url } | AIProvider::OpenAICompatible { base_url, .. } => {
                Self::list_openai_compatible_models(base_url).await
            }
            AIProvider::Ollama { base_url } => Self::list_ollama_models(base_url).await,
        }
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
