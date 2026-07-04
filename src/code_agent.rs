use serde::{Deserialize, Serialize};
use serde_json::json;
use tauri::Emitter;
use std::path::Path;

use crate::llm::{LLMClient, ChatRequest, Message as LLMMessage, Tool, ToolFunction};
use crate::mem0::{Mem0Client, Message as Mem0Message};

/// Code Agent System
///
/// An intelligent coding assistant that can:
/// - Analyze code and provide suggestions
/// - Read and write files
/// - Execute commands
/// - Search and refactor code

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentStep {
    pub step_type: String,
    pub description: String,
    pub status: String, // "running" | "completed" | "error"
}

pub struct CodeAgent {
    llm_client: LLMClient,
    app_handle: tauri::AppHandle,
    workspace_path: String,
    mem0_client: Option<Mem0Client>,
    user_id: String,
}

impl CodeAgent {
    pub fn new(
        base_url: String,
        api_key: String,
        _model: String,
        app_handle: tauri::AppHandle,
        workspace_path: String,
        mem0_api_key: Option<String>,
        user_id: String,
    ) -> Self {
        let llm_client = LLMClient::new(base_url, api_key);
        let mem0_client = mem0_api_key.map(|key| Mem0Client::new(key));

        Self {
            llm_client,
            app_handle,
            workspace_path,
            mem0_client,
            user_id,
        }
    }

    fn emit_log(&self, message: &str) {
        let _ = self.app_handle.emit("code-agent-log", message);
        println!("[CodeAgent] {}", message);
    }

    /// Search for relevant memories
    async fn search_relevant_memories(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching relevant coding memories...");

            match client.search_memories(
                query,
                &self.user_id,
                Some("code".to_string()),
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

    /// Save the coding session to memory
    /// Mem0 will automatically extract key learnings, patterns, and preferences
    async fn save_to_memory(&self, conversation: Vec<Mem0Message>) {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Saving coding session insights to memory...");

            match client.add_memory(
                conversation,
                &self.user_id,
                Some("code".to_string()),
                Some(format!("code-{}", chrono::Utc::now().timestamp()))
            ).await {
                Ok(response) => {
                    // Handle both immediate and async responses
                    if let Some(status) = &response.status {
                        if status == "PENDING" {
                            self.emit_log("Memory extraction queued (processing async)");
                            if let Some(event_id) = &response.event_id {
                                println!("  [Memory Event ID] {}", event_id);
                            }
                        }
                    } else if let Some(memories) = &response.memories {
                        if !memories.is_empty() {
                            self.emit_log(&format!("Extracted {} new insights", memories.len()));
                            for memory in memories {
                                println!("  [Memory] {}", memory.memory);
                            }
                        } else {
                            self.emit_log("No new insights extracted (information may already be known)");
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to save memories: {}", e);
                }
            }
        }
    }

    fn get_system_prompt() -> String {
        r#"You are an expert software engineer and coding assistant. Your PRIMARY job is to write, modify, and debug code.

## Your Core Responsibilities:

1. **ALWAYS WRITE ACTUAL CODE** - When asked to create something, you MUST generate the complete, working code
2. **USE TOOLS PROACTIVELY** - Don't just explain, actually create/modify files using write_file
3. **BE HANDS-ON** - Write the code first, then explain what you did
4. **SHOW THE CODE** - Always display the full code in your response using markdown code blocks

## Workflow for Code Requests:

When a user asks you to create/build/implement something:

1. **WRITE** the complete code using write_file tool
2. **SHOW** the code in your response using markdown code blocks with syntax highlighting
3. **EXPLAIN** what you created and how to use it

## Response Format (MANDATORY):

After using write_file, your response MUST include:

```language
[FULL CODE CONTENT HERE]
```

Then explain what you created.

**Example Response:**

I've created `server.js` with a complete HTTP server:

```javascript
const http = require('http');

const server = http.createServer((req, res) => {
  res.writeHead(200, {'Content-Type': 'text/plain'});
  res.end('Hello World\n');
});

server.listen(3000, () => {
  console.log('Server running at http://localhost:3000/');
});
```

This creates a basic HTTP server on port 3000. Run it with:
```bash
node server.js
```

## Tool Usage Guidelines:

- **read_file**: Use to examine existing code before making changes
- **write_file**: Use to create NEW files or COMPLETELY REWRITE files. Include the FULL file content.
- **list_files**: Use to explore project structure
- **execute_command**: Use to run tests, build projects, or execute code

## Code Generation Rules:

1. Always generate COMPLETE, WORKING code (no placeholders like "// your code here")
2. Include necessary imports, dependencies, and setup
3. Add clear comments explaining key parts
4. Follow best practices for the language
5. Make code production-ready with error handling
6. **ALWAYS show the code in your response using code blocks**

## Important Notes:

- **NEVER** just say "I've created the file" - always SHOW the code
- Include the FULL code content in markdown code blocks
- Use proper syntax highlighting (```javascript, ```python, ```html, etc.)
- Don't truncate the code - show everything
- Code blocks should come BEFORE the explanation

Your goal: Be a PRODUCTIVE coding assistant who WRITES CODE and SHOWS IT to the user."#.to_string()
    }

    fn get_available_tools() -> Vec<Tool> {
        vec![
            Tool {
                r#type: "function".to_string(),
                function: ToolFunction {
                    name: "read_file".to_string(),
                    description: "Read the contents of a file at the specified path".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "The file path to read (relative to workspace)"
                            }
                        },
                        "required": ["path"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: ToolFunction {
                    name: "write_file".to_string(),
                    description: "Write content to a file, creating it if it doesn't exist".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "The file path to write to (relative to workspace)"
                            },
                            "content": {
                                "type": "string",
                                "description": "The complete content to write to the file"
                            }
                        },
                        "required": ["path", "content"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: ToolFunction {
                    name: "list_files".to_string(),
                    description: "List files in a directory".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "path": {
                                "type": "string",
                                "description": "The directory path to list (relative to workspace)"
                            }
                        },
                        "required": ["path"]
                    }),
                },
            },
            Tool {
                r#type: "function".to_string(),
                function: ToolFunction {
                    name: "execute_command".to_string(),
                    description: "Execute a shell command in the workspace directory".to_string(),
                    parameters: json!({
                        "type": "object",
                        "properties": {
                            "command": {
                                "type": "string",
                                "description": "The command to execute"
                            }
                        },
                        "required": ["command"]
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
            "read_file" => {
                let path = arguments["path"].as_str().ok_or("Missing path parameter")?;
                let full_path = Path::new(&self.workspace_path).join(path);

                self.emit_log(&format!("Reading file: {}", path));

                let content = std::fs::read_to_string(&full_path)
                    .map_err(|e| format!("Failed to read file {}: {}", path, e))?;

                Ok(content)
            }
            "write_file" => {
                let path = arguments["path"].as_str().ok_or("Missing path parameter")?;
                let content = arguments["content"].as_str().ok_or("Missing content parameter")?;
                let full_path = Path::new(&self.workspace_path).join(path);

                self.emit_log(&format!("Writing file: {}", path));

                // Create parent directories if needed
                if let Some(parent) = full_path.parent() {
                    std::fs::create_dir_all(parent)
                        .map_err(|e| format!("Failed to create directories: {}", e))?;
                }

                std::fs::write(&full_path, content)
                    .map_err(|e| format!("Failed to write file {}: {}", path, e))?;

                Ok(format!("Successfully wrote to {}", path))
            }
            "list_files" => {
                let path = arguments["path"].as_str().ok_or("Missing path parameter")?;
                let full_path = Path::new(&self.workspace_path).join(path);

                self.emit_log(&format!("Listing directory: {}", path));

                let entries = std::fs::read_dir(&full_path)
                    .map_err(|e| format!("Failed to read directory {}: {}", path, e))?;

                let mut files = Vec::new();
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(name) = entry.file_name().to_str() {
                            let is_dir = entry.file_type().map(|t| t.is_dir()).unwrap_or(false);
                            files.push(format!("{}{}", name, if is_dir { "/" } else { "" }));
                        }
                    }
                }

                Ok(files.join("\n"))
            }
            "execute_command" => {
                let command = arguments["command"].as_str().ok_or("Missing command parameter")?;

                self.emit_log(&format!("Executing command: {}", command));

                let output = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(command)
                    .current_dir(&self.workspace_path)
                    .output()
                    .map_err(|e| format!("Failed to execute command: {}", e))?;

                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);

                let result = if output.status.success() {
                    format!("Command succeeded:\n{}", stdout)
                } else {
                    format!("Command failed:\nstdout: {}\nstderr: {}", stdout, stderr)
                };

                Ok(result)
            }
            _ => Err(format!("Unknown tool: {}", tool_name)),
        }
    }

    pub async fn process_request(
        &self,
        user_message: &str,
        model: &str,
    ) -> Result<String, String> {
        self.emit_log("Code agent starting...");

        // Search for relevant memories
        let memories = self.search_relevant_memories(user_message).await;

        let mut system_prompt = Self::get_system_prompt();

        // Add memory context to system prompt if available
        if let Some(mem_list) = &memories {
            if !mem_list.is_empty() {
                system_prompt.push_str("\n\n## Previous Context & Learnings:\n\n");
                for (i, memory) in mem_list.iter().enumerate() {
                    system_prompt.push_str(&format!("{}. {}\n", i + 1, memory));
                }
                self.emit_log("Enhanced with previous coding context");
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

        // Agent loop: max 10 iterations to prevent infinite loops
        for iteration in 0..10 {
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

                // Save entire conversation to memory - Mem0 will extract insights
                self.save_to_memory(conversation_for_memory).await;

                return Ok(final_response);
            }
        }

        Err("Agent exceeded maximum iterations".to_string())
    }
}
