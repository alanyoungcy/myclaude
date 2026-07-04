use crate::{AppState, config::AppConfig, llm::{LLMClient, ChatRequest, Message as LLMMessage}, storage::{SystemPrompt, Conversation, ChatMessage}, tavily::{TavilyClient, TavilySearchResult}, skills::{Skill, skill_to_tool}};
use tauri::State;
use tauri::Emitter;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageRequest {
    pub conversation_id: String,
    pub message: String,
    pub files: Option<Vec<FileAttachment>>,
    pub mode: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileAttachment {
    pub name: String,
    pub content: String,
    pub mime_type: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SendMessageResponse {
    pub message: ChatMessage,
    pub assistant_message: ChatMessage,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConversationWithMessages {
    pub conversation: Conversation,
    pub messages: Vec<ChatMessage>,
}

// Config commands
#[tauri::command]
pub async fn get_config(state: State<'_, AppState>) -> Result<AppConfig, String> {
    let config = state.config.lock().unwrap();
    Ok(config.clone())
}

#[tauri::command]
pub async fn update_config(
    state: State<'_, AppState>,
    config: AppConfig,
) -> Result<(), String> {
    let mut app_config = state.config.lock().unwrap();
    *app_config = config.clone();
    config.save().map_err(|e| e.to_string())?;
    Ok(())
}

// Model commands
#[tauri::command]
pub async fn get_models(state: State<'_, AppState>) -> Result<Vec<String>, String> {
    let config = state.config.lock().unwrap().clone();
    
    if config.api_key.is_empty() || config.base_url.is_empty() {
        return Err("API key and base URL must be configured".to_string());
    }
    
    let client = LLMClient::new(config.base_url, config.api_key);
    
    match client.list_models().await {
        Ok(models) => Ok(models.into_iter().map(|m| m.id).collect()),
        Err(e) => Err(e.to_string()),
    }
}

// Chat commands
#[tauri::command]
pub async fn send_message(
    app_handle: tauri::AppHandle,
    state: State<'_, AppState>,
    request: SendMessageRequest,
) -> Result<SendMessageResponse, String> {
    let config = state.config.lock().unwrap().clone();
    
    if config.api_key.is_empty() || config.base_url.is_empty() {
        return Err("API key and base URL must be configured".to_string());
    }
    
    // Save user message to database and get history
    let (user_message, history) = {
        let db = state.db.lock().unwrap();
        let user_message = db
            .add_message(&request.conversation_id, "user", &request.message)
            .map_err(|e| e.to_string())?;

        let history = db
            .get_messages(&request.conversation_id)
            .map_err(|e| e.to_string())?;

        (user_message, history)
    };

    // TEMP: Mode-specific agents disabled during Rig migration
    // TODO: Integrate RigGeneralAgent, RigDeepResearchAgent, RigCodeAgent, RigResumeAgent

    // Build messages for LLM
    let mut messages = vec![LLMMessage {
        role: "system".to_string(),
        content: Some(config.system_prompt.clone()),
        tool_calls: None,
        tool_call_id: None,
        name: None,
    }];
    
    for msg in history {
        messages.push(LLMMessage {
            role: msg.role,
            content: Some(msg.content),
            tool_calls: None,
            tool_call_id: None,
            name: None,
        });
    }
    
    // Load skills and convert to tools
    let mut tools = Vec::new();

    // Add web search tool if Tavily API key is configured
    if !config.tavily_api_key.is_empty() {
        tools.push(crate::llm::Tool {
            r#type: "function".to_string(),
            function: crate::llm::ToolFunction {
                name: "web_search".to_string(),
                description: "Search the web for current information, news, facts, or any information not in your knowledge base. Use this when you need up-to-date information or to verify facts.".to_string(),
                parameters: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "query": {
                            "type": "string",
                            "description": "The search query to find relevant information"
                        }
                    },
                    "required": ["query"]
                }),
            },
        });
    }

    // Add skills from .md files
    if let Ok(skills) = state.skill_loader.load_skills() {
        for skill in skills {
            tools.push(skill_to_tool(&skill));
        }
    }

    let tools = if tools.is_empty() { None } else { Some(tools) };
    
    // Call LLM
    let client = LLMClient::new(config.base_url.clone(), config.api_key.clone());
    let mut chat_request = ChatRequest {
        model: config.model.clone(),
        messages: messages.clone(),
        tools,
        stream: None,
    };
    
    let response = client
        .send_message(chat_request.clone())
        .await
        .map_err(|e| e.to_string())?;

    let assistant_message = response
        .choices
        .first()
        .ok_or("No response from LLM")?
        .message
        .clone();

    // Handle tool calls
    let final_content: String;
    
    if let Some(tool_calls) = &assistant_message.tool_calls {
        println!("Tool calls detected: {} calls", tool_calls.len());
        
        // Add assistant message with tool calls to conversation
        chat_request.messages.push(assistant_message.clone());
        
        for tool_call in tool_calls {
            if tool_call.function.name == "web_search" {
                println!("Executing web_search tool call: {}", tool_call.id);
                
                // Parse arguments
                let args: serde_json::Value = serde_json::from_str(&tool_call.function.arguments)
                    .map_err(|e| format!("Failed to parse tool arguments: {}", e))?;
                
                let query = args["query"].as_str().ok_or("Missing query parameter")?;
                
                // Execute search
                let tavily_client = TavilyClient::new(config.tavily_api_key.clone());
                let search_results = tavily_client
                    .search(query, 5)
                    .await
                    .map_err(|e| format!("Search failed: {}", e))?;
                
                // Format results
                let mut result_text = format!("Search results for '{}':\n\n", query);
                for (idx, result) in search_results.iter().enumerate() {
                    result_text.push_str(&format!(
                        "{}. {}\nURL: {}\nRelevance: {:.0}%\n{}\n\n",
                        idx + 1,
                        result.title,
                        result.url,
                        result.score * 100.0,
                        result.content
                    ));
                }
                
                // Add tool response to messages
                chat_request.messages.push(LLMMessage {
                    role: "tool".to_string(),
                    content: Some(result_text),
                    tool_calls: None,
                    tool_call_id: Some(tool_call.id.clone()),
                    name: Some("web_search".to_string()),
                });
            }
        }
        
        // Call LLM again with tool results
        println!("Calling LLM with tool results");
        let final_response = client
            .send_message(chat_request)
            .await
            .map_err(|e| e.to_string())?;
        
        final_content = final_response
            .choices
            .first()
            .ok_or("No response from LLM after tool call")?
            .message
            .content
            .clone()
            .unwrap_or_default();
    } else {
        final_content = assistant_message.content.unwrap_or_default();
    }
    
    // Save assistant message to database
    let assistant_message_record = {
        let db = state.db.lock().unwrap();
        db.add_message(&request.conversation_id, "assistant", &final_content)
            .map_err(|e| e.to_string())?
    };
    
    Ok(SendMessageResponse {
        message: user_message,
        assistant_message: assistant_message_record,
    })
}

// Conversation commands
#[tauri::command]
pub async fn get_conversations(state: State<'_, AppState>) -> Result<Vec<Conversation>, String> {
    let db = state.db.lock().unwrap();
    db.get_conversations().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_conversation(
    state: State<'_, AppState>,
    id: String,
) -> Result<Option<ConversationWithMessages>, String> {
    let db = state.db.lock().unwrap();
    
    let conversation = db.get_conversation(&id).map_err(|e| e.to_string())?;
    
    if let Some(conv) = conversation {
        let messages = db.get_messages(&id).map_err(|e| e.to_string())?;
        Ok(Some(ConversationWithMessages {
            conversation: conv,
            messages,
        }))
    } else {
        Ok(None)
    }
}

#[tauri::command]
pub async fn create_conversation(
    state: State<'_, AppState>,
    title: String,
) -> Result<Conversation, String> {
    let db = state.db.lock().unwrap();
    db.create_conversation(&title).map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_conversation(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_conversation(&id).map_err(|e| e.to_string())
}

// System prompt commands
#[tauri::command]
pub async fn get_system_prompts(state: State<'_, AppState>) -> Result<Vec<SystemPrompt>, String> {
    let db = state.db.lock().unwrap();
    db.get_system_prompts().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn save_system_prompt(
    state: State<'_, AppState>,
    name: String,
    prompt: String,
) -> Result<SystemPrompt, String> {
    let db = state.db.lock().unwrap();
    db.save_system_prompt(&name, &prompt)
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn delete_system_prompt(
    state: State<'_, AppState>,
    id: String,
) -> Result<(), String> {
    let db = state.db.lock().unwrap();
    db.delete_system_prompt(&id).map_err(|e| e.to_string())
}

// Web search command
#[tauri::command]
pub async fn web_search(
    state: State<'_, AppState>,
    query: String,
    max_results: Option<u32>,
) -> Result<Vec<TavilySearchResult>, String> {
    let config = state.config.lock().unwrap().clone();

    if config.tavily_api_key.is_empty() {
        return Err("Tavily API key not configured".to_string());
    }

    let client = TavilyClient::new(config.tavily_api_key);
    let results = client
        .search(&query, max_results.unwrap_or(5))
        .await
        .map_err(|e| e.to_string())?;

    Ok(results)
}

// Skills command
#[tauri::command]
pub async fn get_skills(state: State<'_, AppState>) -> Result<Vec<Skill>, String> {
    state.skill_loader
        .load_skills()
        .map_err(|e| e.to_string())
}
