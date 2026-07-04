use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Emitter;

use crate::llm::{LLMClient, ChatRequest, Message as LLMMessage, Tool, ToolFunction};
use crate::mem0::{Mem0Client, Message as Mem0Message};

/// General Agent System
///
/// A versatile agent for answering general questions with tool support
/// Can use web search, calculations, and other utilities

pub struct GeneralAgent {
    llm_client: LLMClient,
    app_handle: tauri::AppHandle,
    mem0_client: Option<Mem0Client>,
    user_id: String,
    tavily_api_key: Option<String>,
}

impl GeneralAgent {
    pub fn new(
        base_url: String,
        api_key: String,
        app_handle: tauri::AppHandle,
        mem0_api_key: Option<String>,
        tavily_api_key: Option<String>,
        user_id: String,
    ) -> Self {
        let llm_client = LLMClient::new(base_url, api_key);
        let mem0_client = mem0_api_key.map(|key| Mem0Client::new(key));

        Self {
            llm_client,
            app_handle,
            mem0_client,
            user_id,
            tavily_api_key,
        }
    }

    fn emit_log(&self, message: &str) {
        let _ = self.app_handle.emit("general-agent-log", message);
        println!("[GeneralAgent] {}", message);
    }

    fn get_system_prompt() -> String {
        r#"You are a helpful, knowledgeable AI assistant with access to tools.

Your role is to:
- Answer questions accurately and helpfully
- Use tools when they can improve your response
- Provide clear, well-structured answers
- Cite sources when using web search
- Explain your reasoning

Available tools:
- web_search: Search the web for current information, facts, news, or specific data
- calculator: Perform mathematical calculations

When to use tools:
- Use web_search when you need current information, recent events, specific facts, or real-world data
- Use calculator for complex mathematical operations
- Don't use tools for questions you can answer directly from your knowledge

Response guidelines:
- Be concise but thorough
- Use markdown formatting for clarity
- Structure complex answers with headings and lists
- When using web search, cite your sources
- Explain technical concepts in simple terms"#.to_string()
    }

    fn get_available_tools() -> Vec<Tool> {
        vec![
            Tool {
                r#type: "function".to_string(),
                function: ToolFunction {
                    name: "web_search".to_string(),
                    description: "Search the web for current information, facts, news, or specific data".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "query": {
                                "type": "string",
                                "description": "The search query"
                            },
                            "max_results": {
                                "type": "integer",
                                "description": "Maximum number of results (default: 5)"
                            }
                        },
                        "required": ["query"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: ToolFunction {
                    name: "calculator".to_string(),
                    description: "Perform mathematical calculations".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "expression": {
                                "type": "string",
                                "description": "Mathematical expression to evaluate (e.g., '2 + 2', '15% of 200')"
                            }
                        },
                        "required": ["expression"]
                    }),
                },
            },
        ]
    }

    async fn execute_tool(
        &self,
        tool_name: &str,
        arguments: &serde_json::Value,
    ) -> Result<String, String> {
        match tool_name {
            "web_search" => {
                let query = arguments["query"].as_str().ok_or("Missing query parameter")?;
                let max_results = arguments["max_results"].as_u64().unwrap_or(5) as u32;

                self.emit_log(&format!("Searching web: {}", query));

                // Check if Tavily API key is available
                if let Some(api_key) = &self.tavily_api_key {
                    let client = crate::tavily::TavilyClient::new(api_key.clone());

                    match client.search(query, max_results).await {
                        Ok(results) => {
                            let mut formatted = String::new();
                            formatted.push_str(&format!("Found {} results:\n\n", results.len()));

                            for (i, result) in results.iter().enumerate() {
                                formatted.push_str(&format!(
                                    "{}. **{}**\n{}\nSource: {}\n\n",
                                    i + 1,
                                    result.title,
                                    result.content,
                                    result.url
                                ));
                            }

                            Ok(formatted)
                        }
                        Err(e) => Err(format!("Web search failed: {}", e)),
                    }
                } else {
                    Err("Web search not available (Tavily API key not configured)".to_string())
                }
            }
            "calculator" => {
                let expression = arguments["expression"].as_str().ok_or("Missing expression parameter")?;

                self.emit_log(&format!("Calculating: {}", expression));

                // Simple calculator implementation
                // For production, you might want to use a proper math parser library
                match meval::eval_str(expression) {
                    Ok(result) => Ok(format!("Result: {}", result)),
                    Err(e) => Err(format!("Calculation error: {}", e)),
                }
            }
            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }

    /// Search for relevant memories
    async fn search_relevant_memories(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching conversation history...");

            match client.search_memories(
                query,
                &self.user_id,
                Some("general".to_string()),
                Some(5)
            ).await {
                Ok(response) => {
                    if !response.results.is_empty() {
                        let memories: Vec<String> = response.results
                            .iter()
                            .map(|r| r.memory.clone())
                            .collect();

                        self.emit_log(&format!("Found {} relevant memories", memories.len()));
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

    /// Save conversation to memory
    async fn save_to_memory(&self, conversation: Vec<Mem0Message>) {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Saving conversation to memory...");

            match client.add_memory(
                conversation,
                &self.user_id,
                Some("general".to_string()),
                Some(format!("general-{}", chrono::Utc::now().timestamp()))
            ).await {
                Ok(response) => {
                    if let Some(status) = &response.status {
                        if status == "PENDING" {
                            self.emit_log("Memory extraction queued");
                        }
                    } else if let Some(memories) = &response.memories {
                        if !memories.is_empty() {
                            self.emit_log(&format!("Learned {} new things", memories.len()));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to save memories: {}", e);
                }
            }
        }
    }

    pub async fn process_request(
        &self,
        user_message: &str,
        model: &str,
    ) -> Result<String, String> {
        self.emit_log("General agent starting...");

        // Search for relevant memories
        let memories = self.search_relevant_memories(user_message).await;

        let mut system_prompt = Self::get_system_prompt();

        // Add memory context
        if let Some(mem_list) = &memories {
            if !mem_list.is_empty() {
                system_prompt.push_str("\n\n## Previous Context:\n\n");
                for (i, memory) in mem_list.iter().enumerate() {
                    system_prompt.push_str(&format!("{}. {}\n", i + 1, memory));
                }
            }
        }

        let mut messages = vec![
            LLMMessage {
                role: "system".to_string(),
                content: Some(system_prompt),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
            LLMMessage {
                role: "user".to_string(),
                content: Some(user_message.to_string()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
        ];

        let tools = Self::get_available_tools();

        // Track conversation for memory
        let mut conversation_for_memory = vec![
            Mem0Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            }
        ];

        // Agent loop: max 5 iterations
        for iteration in 0..5 {
            self.emit_log(&format!("Agent iteration {}", iteration + 1));

            let request = ChatRequest {
                model: model.to_string(),
                messages: messages.clone(),
                tools: Some(tools.clone()),
                stream: None,
            };

            let response = self.llm_client.send_message(request).await
                .map_err(|e| e.to_string())?;

            let assistant_message = response
                .choices
                .first()
                .ok_or("No response from LLM")?
                .message
                .clone();

            // Check if the assistant wants to use tools
            if let Some(tool_calls) = &assistant_message.tool_calls {
                self.emit_log(&format!("Agent requested {} tool(s)", tool_calls.len()));

                // Add assistant message to conversation
                messages.push(assistant_message.clone());

                // Execute each tool call
                for tool_call in tool_calls {
                    let tool_name = &tool_call.function.name;
                    let arguments: serde_json::Value = serde_json::from_str(&tool_call.function.arguments)
                        .map_err(|e| e.to_string())?;

                    self.emit_log(&format!("Executing tool: {}", tool_name));

                    let result = self.execute_tool(tool_name, &arguments).await?;

                    // Add tool result to conversation
                    messages.push(LLMMessage {
                        role: "tool".to_string(),
                        content: Some(result),
                        tool_calls: None,
                        tool_call_id: Some(tool_call.id.clone()),
                        name: Some(tool_name.clone()),
                    });
                }
            } else {
                // No more tool calls, return the final response
                self.emit_log("Agent completed task");

                let final_response = assistant_message.content.clone().unwrap_or_default();

                // Add final assistant response to memory conversation
                conversation_for_memory.push(Mem0Message {
                    role: "assistant".to_string(),
                    content: final_response.clone(),
                });

                // Save conversation to memory
                self.save_to_memory(conversation_for_memory).await;

                return Ok(final_response);
            }
        }

        Err("Agent exceeded maximum iterations".to_string())
    }
}
