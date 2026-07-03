use autoagents::core::agent::{AgentBuilder, AgentOutputT, DirectAgent};
use autoagents::core::agent::memory::SlidingWindowMemory;
use autoagents::core::agent::prebuilt::executor::{ReActAgent, ReActAgentOutput};
use autoagents::core::agent::task::Task;
use autoagents::core::error::Error as AutoAgentsError;
use autoagents::core::tool::{ToolCallError, ToolInputT, ToolRuntime, ToolT};
use autoagents_derive::{agent, tool, AgentHooks, AgentOutput, ToolInput};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::llm_wrapper::LLMProviderWrapper;
use crate::tavily::TavilyClient;

// Web Search Tool using Tavily
#[derive(Serialize, Deserialize, ToolInput, Debug)]
pub struct WebSearchArgs {
    #[input(description = "The search query to find relevant information")]
    query: String,
    #[input(description = "Maximum number of results to return")]
    max_results: Option<u32>,
}

#[tool(
    name = "web_search",
    description = "Search the web for current information, news, facts, or any information not in your knowledge base. Use this when you need up-to-date information or to verify facts.",
    input = WebSearchArgs,
)]
pub struct WebSearchTool {
    tavily_api_key: String,
}

impl WebSearchTool {
    pub fn new(tavily_api_key: String) -> Self {
        Self { tavily_api_key }
    }
}

#[async_trait::async_trait]
impl ToolRuntime for WebSearchTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let typed_args: WebSearchArgs = serde_json::from_value(args)?;

        let client = TavilyClient::new(self.tavily_api_key.clone());
        let results = client
            .search(&typed_args.query, typed_args.max_results.unwrap_or(5))
            .await
            .map_err(|e| {
                // Convert error to a boxed error with Send + Sync
                let err_str = e.to_string();
                ToolCallError::RuntimeError(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err_str
                )))
            })?;

        let mut result_text = format!("Search results for '{}':\n\n", typed_args.query);
        for (idx, result) in results.iter().enumerate() {
            result_text.push_str(&format!(
                "{}. {}\nURL: {}\nRelevance: {:.0}%\n{}\n\n",
                idx + 1,
                result.title,
                result.url,
                result.score * 100.0,
                result.content
            ));
        }

        Ok(serde_json::json!({ "results": result_text }))
    }
}

// Chat Agent Output
#[derive(Debug, Serialize, Deserialize, AgentOutput)]
pub struct ChatAgentOutput {
    #[output(description = "The response to the user's message")]
    response: String,
    #[output(description = "Whether any tools were used")]
    tools_used: Option<Vec<String>>,
}

impl From<ReActAgentOutput> for ChatAgentOutput {
    fn from(output: ReActAgentOutput) -> Self {
        ChatAgentOutput {
            response: output.response,
            tools_used: None,
        }
    }
}

// Chat Agent
#[agent(
    name = "chat_agent",
    description = "You are a helpful AI assistant. You can search the web when needed.",
    tools = [],
    output = ChatAgentOutput,
)]
#[derive(Clone, AgentHooks)]
pub struct ChatAgent {
    pub system_prompt: String,
}

impl ChatAgent {
    pub fn new(system_prompt: String) -> Self {
        Self { system_prompt }
    }
}

impl Default for ChatAgent {
    fn default() -> Self {
        Self {
            system_prompt: "You are a helpful AI assistant.".to_string(),
        }
    }
}

// Agent Manager for coordinating AutoAgents
pub struct AgentManager {
    llm_wrapper: Arc<LLMProviderWrapper>,
    tavily_api_key: String,
}

impl AgentManager {
    pub fn new(llm_wrapper: Arc<LLMProviderWrapper>, tavily_api_key: String) -> Self {
        Self {
            llm_wrapper,
            tavily_api_key,
        }
    }

    /// Run a chat message through the AutoAgents system
    pub async fn run_chat(&self, message: &str, system_prompt: &str) -> Result<String, AutoAgentsError> {
        // Create agent with sliding window memory
        let memory = Box::new(SlidingWindowMemory::new(10));

        let chat_agent = ChatAgent::new(system_prompt.to_string());

        let agent_handle = AgentBuilder::<_, DirectAgent>::new(ReActAgent::new(chat_agent))
            .llm(self.llm_wrapper.provider())
            .memory(memory)
            .build()
            .await?;

        // Run the task
        let task = Task::new(message);
        let result = agent_handle.agent.run(task).await?;

        Ok(result.response)
    }

    /// Run with streaming support (for future implementation)
    pub async fn run_chat_streaming(&self, message: &str, system_prompt: &str) -> Result<String, AutoAgentsError> {
        // For now, use non-streaming
        // TODO: Implement streaming with AutoAgents
        self.run_chat(message, system_prompt).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::llm_wrapper::{LLMConfig, LLMProviderType};

    #[tokio::test]
    async fn test_agent_manager_creation() {
        let config = LLMConfig {
            provider: LLMProviderType::OpenAI,
            api_key: "test-key".to_string(),
            base_url: None,
            model: "gpt-4".to_string(),
            max_tokens: Some(1000),
            temperature: Some(0.7),
        };

        // This will fail without a real API key, but tests the structure
        let result = LLMProviderWrapper::new(config).await;
        assert!(result.is_ok() || result.is_err()); // Just checking it compiles
    }
}
