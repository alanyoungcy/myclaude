use serde::{Deserialize, Serialize};
use std::sync::Arc;
use autoagents::llm::LLMProvider;
use autoagents::llm::backends::openai::OpenAI;
use autoagents::llm::backends::anthropic::Anthropic;
use autoagents::llm::builder::LLMBuilder;
use autoagents::core::error::Error as AutoAgentsError;

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
            provider,
            config,
        })
    }

    /// Create the actual AutoAgents LLM provider
    async fn create_provider(config: &LLMConfig) -> Result<Arc<dyn LLMProvider>, AutoAgentsError> {
        match config.provider {
            LLMProviderType::OpenAI => {
                let mut builder = LLMBuilder::<OpenAI>::new()
                    .api_key(config.api_key.clone())
                    .model(&config.model);

                if let Some(base_url) = &config.base_url {
                    builder = builder.base_url(base_url);
                }

                if let Some(max_tokens) = config.max_tokens {
                    builder = builder.max_tokens(max_tokens);
                }

                if let Some(temperature) = config.temperature {
                    builder = builder.temperature(temperature);
                }

                let llm = builder.build()?;
                Ok(llm)
            }
            LLMProviderType::Anthropic => {
                let mut builder = LLMBuilder::<Anthropic>::new()
                    .api_key(config.api_key.clone())
                    .model(&config.model);

                if let Some(max_tokens) = config.max_tokens {
                    builder = builder.max_tokens(max_tokens);
                }

                if let Some(temperature) = config.temperature {
                    builder = builder.temperature(temperature);
                }

                let llm = builder.build()?;
                Ok(llm)
            }
            LLMProviderType::DeepSeek | LLMProviderType::Groq | LLMProviderType::Custom => {
                // For DeepSeek, Groq, and Custom providers, use OpenAI-compatible interface
                let mut builder = LLMBuilder::<OpenAI>::new()
                    .api_key(config.api_key.clone())
                    .model(&config.model);

                if let Some(base_url) = &config.base_url {
                    builder = builder.base_url(base_url);
                }

                if let Some(max_tokens) = config.max_tokens {
                    builder = builder.max_tokens(max_tokens);
                }

                if let Some(temperature) = config.temperature {
                    builder = builder.temperature(temperature);
                }

                let llm = builder.build()?;
                Ok(llm)
            }
        }
    }

    /// Get the underlying provider
    pub fn provider(&self) -> Arc<dyn LLMProvider> {
        Arc::clone(&self.provider)
    }

    /// Get the configuration
    pub fn config(&self) -> &LLMConfig {
        &self.config
    }

    /// Detect provider type from base URL
    pub fn detect_provider_type(base_url: &str) -> LLMProviderType {
        let url_lower = base_url.to_lowercase();

        if url_lower.contains("openai.com") {
            LLMProviderType::OpenAI
        } else if url_lower.contains("anthropic.com") {
            LLMProviderType::Anthropic
        } else if url_lower.contains("deepseek.com") {
            LLMProviderType::DeepSeek
        } else if url_lower.contains("groq.com") {
            LLMProviderType::Groq
        } else {
            LLMProviderType::Custom
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_provider_type() {
        assert!(matches!(
            LLMProviderWrapper::detect_provider_type("https://api.openai.com/v1"),
            LLMProviderType::OpenAI
        ));

        assert!(matches!(
            LLMProviderWrapper::detect_provider_type("https://api.anthropic.com/v1"),
            LLMProviderType::Anthropic
        ));

        assert!(matches!(
            LLMProviderWrapper::detect_provider_type("https://api.deepseek.com/v1"),
            LLMProviderType::DeepSeek
        ));

        assert!(matches!(
            LLMProviderWrapper::detect_provider_type("http://localhost:11434"),
            LLMProviderType::Custom
        ));
    }
}
