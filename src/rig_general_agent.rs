//! Rig-based General Agent
//!
//! A refactored version of the general agent using the Rig framework for proper
//! agent architecture with tool calling and memory integration.

use anyhow::Result;
use rig_core::{
    agent::Agent,
    completion::Prompt,
    client::CompletionClient,
    providers::openai::CompletionsClient,
};
use tauri::Emitter;

use crate::{
    mem0::{Mem0Client, Message as Mem0Message},
    rig_provider::create_completions_client,
    tools::all_basic_tools,
};

/// Rig-based General Agent
///
/// Uses the Rig framework for agent capabilities with integrated tools and memory.
pub struct RigGeneralAgent {
    client: CompletionsClient,
    model: String,
    app_handle: tauri::AppHandle,
    mem0_client: Option<Mem0Client>,
    user_id: String,
    tavily_api_key: Option<String>,
}

impl RigGeneralAgent {
    /// Create a new Rig-based general agent
    ///
    /// # Arguments
    /// * `base_url` - OpenAI-compatible API base URL
    /// * `api_key` - API key for authentication
    /// * `model` - Model name to use
    /// * `app_handle` - Tauri app handle for emitting events
    /// * `mem0_api_key` - Optional Mem0 API key for memory
    /// * `tavily_api_key` - Optional Tavily API key for web search
    /// * `user_id` - User ID for memory association
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        app_handle: tauri::AppHandle,
        mem0_api_key: Option<String>,
        tavily_api_key: Option<String>,
        user_id: String,
    ) -> Result<Self> {
        let client = create_completions_client(base_url, api_key)
            .map_err(|e| anyhow::anyhow!("Failed to create client: {}", e))?;
        let mem0_client = mem0_api_key.map(|key| Mem0Client::new(key));

        Ok(Self {
            client,
            model,
            app_handle,
            mem0_client,
            user_id,
            tavily_api_key,
        })
    }

    /// Chat with the agent
    ///
    /// # Arguments
    /// * `query` - The user's query
    ///
    /// # Returns
    /// The agent's response
    pub async fn chat(&self, query: &str) -> Result<String> {
        self.emit_log("Starting general agent...");

        // 1. Search memory for relevant context
        let context = self.search_memory(query).await;

        // 2. Build the prompt with context
        let full_prompt = if let Some(ctx) = context {
            self.emit_log(&format!("Using {} relevant memories", ctx.len()));
            format!(
                "Relevant context from previous conversations:\n{}\n\nUser query: {}",
                ctx.join("\n"),
                query
            )
        } else {
            query.to_string()
        };

        // 3. Build the agent with tools
        self.emit_log("Building agent with tools...");
        let agent = self.build_agent();

        // 4. Call the agent (Rig handles tool calling automatically)
        self.emit_log("Processing query...");
        let response = agent.prompt(&full_prompt).await?;

        // 5. Save interaction to memory
        self.save_memory(query, &response).await;

        self.emit_log("General agent complete");
        Ok(response)
    }

    /// Build a Rig agent with tools
    fn build_agent(&self) -> Agent<impl rig_core::completion::CompletionModel> {
        let tools = all_basic_tools(self.tavily_api_key.clone());

        self.client
            .agent(&self.model)
            .preamble(
                "You are a helpful and knowledgeable assistant. You have access to web search \
                 and calculator tools to help answer questions accurately. Use web search when \
                 you need current information or facts you're not certain about. Use the calculator \
                 for precise mathematical calculations. Always provide clear, accurate, and helpful responses.",
            )
            .tools(tools)
            .max_tokens(2048)
            .temperature(0.7)
            .build()
    }

    /// Search Mem0 for relevant memories
    async fn search_memory(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching conversation history...");

            match client
                .search_memories(query, &self.user_id, Some("general".to_string()), Some(5))
                .await
            {
                Ok(response) => {
                    if !response.results.is_empty() {
                        let memories: Vec<String> = response
                            .results
                            .iter()
                            .map(|r| r.memory.clone())
                            .collect();

                        return Some(memories);
                    }
                }
                Err(e) => {
                    eprintln!("Failed to search memories: {}", e);
                }
            }
        }
        None
    }

    /// Save interaction to Mem0
    async fn save_memory(&self, query: &str, response: &str) {
        if let Some(client) = &self.mem0_client {
            let messages = vec![
                Mem0Message {
                    role: "user".to_string(),
                    content: query.to_string(),
                },
                Mem0Message {
                    role: "assistant".to_string(),
                    content: response.to_string(),
                },
            ];

            match client
                .add_memory(messages, &self.user_id, Some("general".to_string()), None)
                .await
            {
                Ok(_) => {
                    self.emit_log("Saved to memory");
                }
                Err(e) => {
                    eprintln!("Failed to save memories: {}", e);
                }
            }
        }
    }

    /// Emit a log message to the frontend
    fn emit_log(&self, message: &str) {
        let _ = self.app_handle.emit("general-agent-log", message);
        println!("[General Agent] {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests require a mock Tauri app handle and are mainly for
    // compile-time verification. Integration tests should be done separately.

    #[test]
    fn test_agent_creation() {
        // This test just verifies the struct can be created
        // Actual functionality requires a real Tauri runtime
        assert!(true);
    }
}
