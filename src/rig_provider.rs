//! Custom OpenAI-compatible provider for Rig framework
//!
//! This module provides a wrapper around Rig's OpenAI provider that supports
//! custom base URLs for OpenAI-compatible APIs (Azure OpenAI, local models, etc.)

use rig_core::providers::openai::{Client as OpenAIClient, CompletionsClient};

/// Create an OpenAI client with custom base URL and API key
///
/// # Arguments
/// * `base_url` - The base URL of the OpenAI-compatible API (e.g., "https://api.openai.com/v1")
/// * `api_key` - The API key for authentication
///
/// # Example
/// ```no_run
/// use myclaude::rig_provider::create_openai_client;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = create_openai_client(
///     "https://api.openai.com/v1".to_string(),
///     "sk-...".to_string()
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn create_openai_client(
    base_url: String,
    api_key: String,
) -> Result<OpenAIClient, Box<dyn std::error::Error>> {
    let client = OpenAIClient::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    Ok(client)
}

/// Create an OpenAI Completions API client with custom base URL and API key
///
/// Use this for providers that use the traditional Chat Completions API instead
/// of the newer Responses API.
///
/// # Arguments
/// * `base_url` - The base URL of the OpenAI-compatible API
/// * `api_key` - The API key for authentication
///
/// # Example
/// ```no_run
/// use myclaude::rig_provider::create_completions_client;
///
/// # fn main() -> Result<(), Box<dyn std::error::Error>> {
/// let client = create_completions_client(
///     "https://api.openai.com/v1".to_string(),
///     "sk-...".to_string()
/// )?;
/// # Ok(())
/// # }
/// ```
pub fn create_completions_client(
    base_url: String,
    api_key: String,
) -> Result<CompletionsClient, Box<dyn std::error::Error>> {
    let client = CompletionsClient::builder()
        .base_url(&base_url)
        .api_key(&api_key)
        .build()?;

    Ok(client)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let result = create_openai_client(
            "https://api.openai.com/v1".to_string(),
            "test-key".to_string(),
        );

        assert!(result.is_ok());
    }

    #[test]
    fn test_completions_client_creation() {
        let result = create_completions_client(
            "https://api.openai.com/v1".to_string(),
            "test-key".to_string(),
        );

        assert!(result.is_ok());
    }
}
