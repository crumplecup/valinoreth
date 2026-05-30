//! Agent configuration for AI combat participants.
//!
//! [`AgentConfig`] holds the LLM provider, model, and token limit for one agent.
//! Load from a TOML file with [`from_file`], or construct in code with [`new`].
//! Call [`create_llm_config`] to build the [`LlmConfig`] used by
//! [`LlmElicitCommunicator`]; the API key is read from the environment at that
//! point, never stored in the config struct.
//!
//! ## TOML format
//!
//! ```toml
//! name = "Ragnar"
//! llm_provider = "anthropic"
//! llm_model = "claude-haiku-4-5-20251001"
//! llm_max_tokens = 150
//! ```
//!
//! [`from_file`]: AgentConfig::from_file
//! [`new`]: AgentConfig::new
//! [`create_llm_config`]: AgentConfig::create_llm_config
//! [`LlmElicitCommunicator`]: crate::LlmElicitCommunicator

use std::path::{Path, PathBuf};

use derive_getters::Getters;
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};
use tracing::{debug, info, instrument};

use crate::{LlmConfig, LlmProvider};

fn default_provider() -> LlmProvider {
    LlmProvider::Anthropic
}

fn default_model() -> String {
    "claude-haiku-4-5-20251001".to_string()
}

fn default_max_tokens() -> u32 {
    150
}

/// Configuration for an AI agent participant in combat.
#[derive(Debug, Clone, Getters, Serialize, Deserialize)]
pub struct AgentConfig {
    /// Display name for this agent.
    name: String,
    /// LLM provider (Anthropic or OpenAI).
    #[serde(default = "default_provider")]
    llm_provider: LlmProvider,
    /// Model identifier (e.g. `"claude-haiku-4-5-20251001"`).
    #[serde(default = "default_model")]
    llm_model: String,
    /// Maximum tokens in each LLM response.
    #[serde(default = "default_max_tokens")]
    llm_max_tokens: u32,
    /// Path this config was loaded from (not serialized).
    #[serde(skip)]
    config_path: Option<PathBuf>,
}

impl AgentConfig {
    /// Creates an agent configuration with default LLM settings (Anthropic Haiku).
    #[instrument(skip_all)]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            llm_provider: default_provider(),
            llm_model: default_model(),
            llm_max_tokens: default_max_tokens(),
            config_path: None,
        }
    }

    /// Loads configuration from a TOML file.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] if the file cannot be read or parsed.
    #[instrument(skip(path), fields(path = %path.as_ref().display()))]
    pub fn from_file(path: impl AsRef<Path>) -> Result<Self, ConfigError> {
        debug!("Loading agent config from file");
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| ConfigError::new(format!("Failed to read config file: {e}")))?;
        let mut config: Self = toml::from_str(&content)
            .map_err(|e| ConfigError::new(format!("Failed to parse config TOML: {e}")))?;
        config.config_path = Some(path.as_ref().to_path_buf());
        info!(agent_name = %config.name, "Agent config loaded");
        Ok(config)
    }

    /// Builds an [`LlmConfig`] by reading the API key from the environment.
    ///
    /// Reads `ANTHROPIC_API_KEY` or `OPENAI_API_KEY` depending on the provider.
    ///
    /// # Errors
    ///
    /// Returns [`ConfigError`] if the required environment variable is not set.
    #[instrument(skip(self), fields(provider = %self.llm_provider, model = %self.llm_model))]
    pub fn create_llm_config(&self) -> Result<LlmConfig, ConfigError> {
        debug!("Building LLM config for agent");
        let api_key = match self.llm_provider {
            LlmProvider::Anthropic => std::env::var("ANTHROPIC_API_KEY")
                .map_err(|_| ConfigError::new("ANTHROPIC_API_KEY environment variable not set"))?,
            LlmProvider::OpenAI => std::env::var("OPENAI_API_KEY")
                .map_err(|_| ConfigError::new("OPENAI_API_KEY environment variable not set"))?,
        };
        info!(provider = %self.llm_provider, model = %self.llm_model, "LLM config created");
        Ok(LlmConfig {
            provider: self.llm_provider,
            api_key,
            model: self.llm_model.clone(),
            max_tokens: self.llm_max_tokens,
        })
    }
}

/// Error produced when agent configuration is invalid or incomplete.
#[derive(Debug, Clone, Display, Error)]
#[display("Config error: {} at {}:{}", message, file, line)]
pub struct ConfigError {
    /// Error description.
    pub message: String,
    /// Source line.
    pub line: u32,
    /// Source file.
    pub file: &'static str,
}

impl ConfigError {
    /// Creates a new error at the caller's location.
    #[track_caller]
    pub fn new(message: impl Into<String>) -> Self {
        let loc = std::panic::Location::caller();
        Self {
            message: message.into(),
            line: loc.line(),
            file: loc.file(),
        }
    }
}
