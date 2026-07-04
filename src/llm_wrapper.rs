use autoagents::llm::openai::OpenAIProvider;
use autoagents::llm::anthropic::AnthropicProvider;
use autoagents::core::llm::LLMProvider;
use autoagents::core::error::Error as AutoAgentsError;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LLMProviderType {
    OpenAI,
    Anthropic,
    DeepSeek,
    Groq,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LLMConfig {
    pub provider: LLMProviderType,
    pub api_key: String,
    pub base_url: Option<String>,
    pub model: String,
    pub max_tokens: Option<u32>,
    pub temperature: Option<f32>,
}

pub struct LLMProviderWrapper {
    provider: Arc<dyn LLMProvider>,
    config: LLMConfig,
}

impl LLMProviderWrapper {
    /// Create a new LLM provider wrapper with AutoAgents
    pub async fn new(config: LLMConfig) -> Result<Self, AutoAgentsError> {
        let provider = Self::create_provider(&config).await?;

        Ok(Self {
            provider: Arc::new(provider),
            config,
        })
    }

    async fn create_provider(config: &LLMConfig) -> Result<Box<dyn LLMProvider>, AutoAgentsError> {
        match config.provider {
            LLMProviderType::OpenAI | LLMProviderType::DeepSeek | LLMProviderType::Groq | LLMProviderType::Custom => {
                let mut provider = OpenAIProvider::new(
                    config.api_key.clone(),
                    config.model.clone(),
                );

                if let Some(base_url) = &config.base_url {
                    provider = provider.with_base_url(base_url.clone());
                }

                if let Some(max_tokens) = config.max_tokens {
                    provider = provider.with_max_tokens(max_tokens);
                }

                if let Some(temperature) = config.temperature {
                    provider = provider.with_temperature(temperature);
                }

                Ok(Box::new(provider))
            }
            LLMProviderType::Anthropic => {
                let mut provider = AnthropicProvider::new(
                    config.api_key.clone(),
                    config.model.clone(),
                );

                if let Some(max_tokens) = config.max_tokens {
                    provider = provider.with_max_tokens(max_tokens);
                }

                if let Some(temperature) = config.temperature {
                    provider = provider.with_temperature(temperature);
                }

                Ok(Box::new(provider))
            }
        }
    }

    pub fn provider(&self) -> Arc<dyn LLMProvider> {
        self.provider.clone()
    }

    pub fn config(&self) -> &LLMConfig {
        &self.config
    }

    pub fn detect_provider_type(base_url: &str) -> LLMProviderType {
        if base_url.contains("anthropic") || base_url.contains("claude") {
            LLMProviderType::Anthropic
        } else if base_url.contains("deepseek") {
            LLMProviderType::DeepSeek
        } else if base_url.contains("groq") {
            LLMProviderType::Groq
        } else if base_url.contains("openai") {
            LLMProviderType::OpenAI
        } else {
            LLMProviderType::Custom
        }
    }
}
