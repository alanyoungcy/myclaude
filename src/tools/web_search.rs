//! Web Search Tool using Tavily API
//!
//! This tool provides web search capabilities for agents using the Tavily search API.

use rig_core::{
    completion::ToolDefinition,
    tool::Tool,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use thiserror::Error;

use crate::tavily::{TavilyClient, TavilySearchResult};

/// Arguments for web search tool
#[derive(Debug, Deserialize)]
pub struct WebSearchArgs {
    /// The search query
    pub query: String,
    /// Maximum number of results to return (default: 10 for research)
    #[serde(default = "default_max_results")]
    pub max_results: u32,
}

fn default_max_results() -> u32 {
    10  // Changed to 10 for deeper research
}

/// Web search tool errors
#[derive(Debug, Error)]
pub enum WebSearchError {
    #[error("Tavily API error: {0}")]
    TavilyError(String),
    #[error("No API key provided")]
    NoApiKey,
    #[error("Invalid query")]
    InvalidQuery,
}

/// Web Search Tool
///
/// Provides web search capabilities using Tavily API.
///
/// # Example
/// ```no_run
/// use myclaude::tools::web_search::WebSearchTool;
///
/// let tool = WebSearchTool::new(Some("tvly-...".to_string()));
/// ```
#[derive(Clone, Serialize)]
pub struct WebSearchTool {
    #[serde(skip)]
    api_key: Option<String>,
}

impl WebSearchTool {
    /// Create a new web search tool with optional API key
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }

    /// Check if the tool is configured with an API key
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }
}

impl Tool for WebSearchTool {
    const NAME: &'static str = "web_search";

    type Error = WebSearchError;
    type Args = WebSearchArgs;
    type Output = String;

    async fn definition(&self, _prompt: String) -> ToolDefinition {
        ToolDefinition {
            name: Self::NAME.to_string(),
            description: "Search the web for current information, facts, news, or specific data. Uses Tavily advanced search with up to 10 pages and 5 chunks per page for comprehensive results.".to_string(),
            parameters: json!({
                "type": "object",
                "properties": {
                    "query": {
                        "type": "string",
                        "description": "The search query to find information about"
                    },
                    "max_results": {
                        "type": "integer",
                        "description": "Maximum number of pages to return (default: 10 for deep research)",
                        "default": 10,
                        "minimum": 1,
                        "maximum": 20
                    }
                },
                "required": ["query"]
            }),
        }
    }

    async fn call(&self, args: Self::Args) -> Result<Self::Output, Self::Error> {
        // Validate query
        if args.query.trim().is_empty() {
            return Err(WebSearchError::InvalidQuery);
        }

        // Check API key
        let api_key = self.api_key
            .as_ref()
            .ok_or(WebSearchError::NoApiKey)?;

        // Create Tavily client and search
        let client = TavilyClient::new(api_key.clone());

        let results = client
            .search(&args.query, args.max_results)
            .await
            .map_err(|e| WebSearchError::TavilyError(e.to_string()))?;

        // Format results as a string
        Ok(format_search_results(&results))
    }
}

/// Format search results into a readable string
fn format_search_results(results: &[TavilySearchResult]) -> String {
    if results.is_empty() {
        return "No results found.".to_string();
    }

    let mut output = format!("Found {} results:\n\n", results.len());

    for (i, result) in results.iter().enumerate() {
        output.push_str(&format!(
            "{}. {}\n   URL: {}\n   {}\n\n",
            i + 1,
            result.title,
            result.url,
            result.content
        ));
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_web_search_tool_creation() {
        let tool = WebSearchTool::new(Some("test-key".to_string()));
        assert!(tool.is_configured());

        let tool_no_key = WebSearchTool::new(None);
        assert!(!tool_no_key.is_configured());
    }

    #[test]
    fn test_format_empty_results() {
        let results = vec![];
        let output = format_search_results(&results);
        assert_eq!(output, "No results found.");
    }

    #[test]
    fn test_format_search_results() {
        let results = vec![
            TavilySearchResult {
                title: "Test Result".to_string(),
                url: "https://example.com".to_string(),
                content: "Test content".to_string(),
                score: 0.9,
            },
        ];

        let output = format_search_results(&results);
        assert!(output.contains("Test Result"));
        assert!(output.contains("https://example.com"));
        assert!(output.contains("Test content"));
    }

    #[tokio::test]
    async fn test_invalid_query() {
        let tool = WebSearchTool::new(Some("test-key".to_string()));
        let args = WebSearchArgs {
            query: "".to_string(),
            max_results: 5,
        };

        let result = tool.call(args).await;
        assert!(matches!(result, Err(WebSearchError::InvalidQuery)));
    }

    #[tokio::test]
    async fn test_no_api_key() {
        let tool = WebSearchTool::new(None);
        let args = WebSearchArgs {
            query: "test query".to_string(),
            max_results: 5,
        };

        let result = tool.call(args).await;
        assert!(matches!(result, Err(WebSearchError::NoApiKey)));
    }
}
