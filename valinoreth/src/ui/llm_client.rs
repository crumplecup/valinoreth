//! Direct LLM API client for agent communicators.
//!
//! Supports Anthropic (Claude) via the Messages API.  The client is a thin
//! `reqwest` wrapper — no SDK dependency — keeping the valinoreth binary small.

use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use tracing::{debug, error, instrument};

/// LLM provider selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum LlmProvider {
    /// Anthropic (Claude models).
    Anthropic,
    /// OpenAI (GPT models).
    OpenAI,
}

impl std::fmt::Display for LlmProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Anthropic => f.write_str("anthropic"),
            Self::OpenAI => f.write_str("openai"),
        }
    }
}

/// Configuration for the LLM client.
#[derive(Debug, Clone)]
pub struct LlmConfig {
    /// Which provider to call.
    pub provider: LlmProvider,
    /// API key (from environment variable).
    pub api_key: String,
    /// Model identifier (e.g. `"claude-haiku-4-5-20251001"`).
    pub model: String,
    /// Maximum tokens in the completion.
    pub max_tokens: u32,
}

/// LLM API error.
#[derive(Debug, Clone, Display, Error)]
#[display("LLM error: {} at {}:{}", message, file, line)]
pub struct LlmError {
    /// Error description.
    pub message: String,
    /// Source line.
    pub line: u32,
    /// Source file.
    pub file: &'static str,
}

impl LlmError {
    /// Creates a new error at the caller's location.
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self { message: message.into(), line: loc.line(), file: loc.file() }
    }
}

/// Lightweight LLM client that sends a single user turn and returns the reply.
#[derive(Debug, Clone)]
pub struct LlmClient {
    config: LlmConfig,
}

impl LlmClient {
    /// Creates a new client from the given configuration.
    #[instrument(skip(config), fields(provider = %config.provider, model = %config.model))]
    pub fn new(config: LlmConfig) -> Self {
        Self { config }
    }

    /// Sends `user_message` with a system prompt and returns the LLM's reply.
    ///
    /// # Errors
    ///
    /// Returns [`LlmError`] on network failure, non-200 response, or missing
    /// content in the API reply.
    #[instrument(skip(self, system_prompt, user_message), fields(provider = %self.config.provider, model = %self.config.model))]
    pub async fn generate(
        &self,
        system_prompt: &str,
        user_message: &str,
    ) -> Result<String, LlmError> {
        match self.config.provider {
            LlmProvider::Anthropic => {
                self.generate_anthropic(system_prompt, user_message).await
            }
            LlmProvider::OpenAI => {
                self.generate_openai(system_prompt, user_message).await
            }
        }
    }

    #[instrument(skip(self, system_prompt, user_message))]
    async fn generate_anthropic(
        &self,
        system_prompt: &str,
        user_message: &str,
    ) -> Result<String, LlmError> {
        debug!("Sending request to Anthropic");
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": self.config.max_tokens,
            "system": system_prompt,
            "messages": [{"role": "user", "content": user_message}],
        });

        let resp = client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.config.api_key)
            .header("anthropic-version", "2023-06-01")
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::new(format!("Anthropic request failed: {e}")))?;

        let status = resp.status();
        let text = resp.text().await
            .map_err(|e| LlmError::new(format!("Failed to read response: {e}")))?;

        if !status.is_success() {
            error!(status = %status, body = %text, "Anthropic API error");
            return Err(LlmError::new(format!("Anthropic error {status}: {text}")));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| LlmError::new(format!("Invalid JSON: {e}")))?;

        json["content"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|item| item["text"].as_str())
            .map(str::to_owned)
            .ok_or_else(|| {
                error!(response = %json, "No text content in Anthropic response");
                LlmError::new("No text content in Anthropic response")
            })
    }

    #[instrument(skip(self, system_prompt, user_message))]
    async fn generate_openai(
        &self,
        system_prompt: &str,
        user_message: &str,
    ) -> Result<String, LlmError> {
        debug!("Sending request to OpenAI");
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "model": self.config.model,
            "max_tokens": self.config.max_tokens,
            "messages": [
                {"role": "system", "content": system_prompt},
                {"role": "user", "content": user_message},
            ],
        });

        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("content-type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| LlmError::new(format!("OpenAI request failed: {e}")))?;

        let status = resp.status();
        let text = resp.text().await
            .map_err(|e| LlmError::new(format!("Failed to read response: {e}")))?;

        if !status.is_success() {
            error!(status = %status, body = %text, "OpenAI API error");
            return Err(LlmError::new(format!("OpenAI error {status}: {text}")));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| LlmError::new(format!("Invalid JSON: {e}")))?;

        json["choices"]
            .as_array()
            .and_then(|arr| arr.first())
            .and_then(|c| c["message"]["content"].as_str())
            .map(str::to_owned)
            .ok_or_else(|| {
                error!(response = %json, "No content in OpenAI response");
                LlmError::new("No content in OpenAI response")
            })
    }
}
