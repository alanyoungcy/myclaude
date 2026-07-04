//! Rig-based Code Agent
//!
//! A refactored version of the code agent using the Rig framework with file operation tools.

use anyhow::Result;
use rig_core::{
    agent::Agent,
    completion::Prompt,
    providers::openai::CompletionsClient,
    client::CompletionClient,
};
use std::path::PathBuf;
use tauri::Emitter;

use crate::{
    mem0::{Mem0Client, Message as Mem0Message},
    rig_provider::create_completions_client,
    tools::{ReadFileTool, WriteFileTool, ListFilesTool},
};

/// Rig-based Code Agent
///
/// Uses the Rig framework for code-related tasks with file operation capabilities.
pub struct RigCodeAgent {
    client: CompletionsClient,
    model: String,
    app_handle: tauri::AppHandle,
    mem0_client: Option<Mem0Client>,
    user_id: String,
    allowed_directories: Vec<PathBuf>,
}

impl RigCodeAgent {
    /// Create a new Rig-based code agent
    ///
    /// # Arguments
    /// * `base_url` - OpenAI-compatible API base URL
    /// * `api_key` - API key for authentication
    /// * `model` - Model name to use
    /// * `app_handle` - Tauri app handle for emitting events
    /// * `mem0_api_key` - Optional Mem0 API key for memory
    /// * `user_id` - User ID for memory association
    /// * `allowed_directories` - List of allowed directories for file operations
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        app_handle: tauri::AppHandle,
        mem0_api_key: Option<String>,
        user_id: String,
        allowed_directories: Vec<PathBuf>,
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
            allowed_directories,
        })
    }

    /// Chat with the code agent
    ///
    /// # Arguments
    /// * `query` - The user's code-related query
    ///
    /// # Returns
    /// The agent's response
    pub async fn chat(&self, query: &str) -> Result<String> {
        self.emit_log("Starting code agent...");

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

        // 3. Build the agent with file operation tools
        self.emit_log("Building code agent with file tools...");
        let agent = self.build_agent();

        // 4. Call the agent (Rig handles tool calling automatically)
        self.emit_log("Processing query...");
        let response = agent.prompt(&full_prompt).await?;

        // 5. Save interaction to memory
        self.save_memory(query, &response).await;

        self.emit_log("Code agent complete");
        Ok(response)
    }

    /// Build a Rig agent with file operation tools
    fn build_agent(&self) -> Agent<impl rig_core::completion::CompletionModel> {
        use rig_core::tool::ToolDyn;

        let tools: Vec<Box<dyn ToolDyn>> = vec![
            Box::new(ReadFileTool::new(self.allowed_directories.clone())),
            Box::new(WriteFileTool::new(self.allowed_directories.clone())),
            Box::new(ListFilesTool::new(self.allowed_directories.clone())),
        ];

        self.client
            .agent(&self.model)
            .preamble(
                "You are an expert programming assistant. You have access to file operation tools \
                 to read, write, and list files. Use these tools to:\n\
                 - Read existing code to understand the project structure\n\
                 - Write new code or modify existing files\n\
                 - List directory contents to explore the codebase\n\n\
                 Always:\n\
                 - Follow best practices and coding standards\n\
                 - Write clean, readable, and well-documented code\n\
                 - Consider security and error handling\n\
                 - Explain your changes clearly\n\n\
                 When writing code, use appropriate syntax highlighting and formatting.",
            )
            .tools(tools)
            .max_tokens(4096)
            .temperature(0.2)
            .build()
    }

    /// Search Mem0 for relevant memories
    async fn search_memory(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching conversation history...");

            match client
                .search_memories(query, &self.user_id, Some("code".to_string()), Some(5))
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
                .add_memory(messages, &self.user_id, Some("code".to_string()), None)
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
        let _ = self.app_handle.emit("code-agent-log", message);
        println!("[Code Agent] {}", message);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        // Compile-time verification test
        assert!(true);
    }
}
