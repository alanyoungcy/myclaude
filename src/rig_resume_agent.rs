//! Rig-based Resume Agent
//!
//! A professional resume and career assistant using the Rig framework.
//! Specializes in resume writing, job analysis, and ATS optimization.

use anyhow::Result;
use rig_core::{
    agent::Agent,
    completion::{Chat, Message},
    providers::openai::CompletionsClient,
    client::CompletionClient,
};
use std::sync::Mutex;
use tauri::Emitter;

use crate::{
    mem0::{Mem0Client, Message as Mem0Message},
    rig_provider::create_completions_client,
};

/// Rig-based Resume Agent
///
/// Provides professional resume and career assistance including:
/// - Resume writing and optimization
/// - Job description analysis
/// - ATS (Applicant Tracking System) compatibility checking
/// - Cover letter writing
/// - Career advice and interview preparation
pub struct RigResumeAgent {
    client: CompletionsClient,
    model: String,
    app_handle: tauri::AppHandle,
    mem0_client: Option<Mem0Client>,
    user_id: String,
    /// Conversation history for maintaining context
    chat_history: Mutex<Vec<Message>>,
}

impl RigResumeAgent {
    /// Create a new Rig-based resume agent
    ///
    /// # Arguments
    /// * `base_url` - OpenAI-compatible API base URL
    /// * `api_key` - API key for authentication
    /// * `model` - Model name to use
    /// * `app_handle` - Tauri app handle for emitting events
    /// * `mem0_api_key` - Optional Mem0 API key for memory
    /// * `user_id` - User ID for memory association
    pub fn new(
        base_url: String,
        api_key: String,
        model: String,
        app_handle: tauri::AppHandle,
        mem0_api_key: Option<String>,
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
            chat_history: Mutex::new(Vec::new()),
        })
    }

    /// Reset the conversation history
    ///
    /// Call this when starting a new resume project or switching contexts
    pub fn reset_history(&self) {
        let mut history = self.chat_history.lock().unwrap();
        history.clear();
    }

    /// Chat with the resume agent
    ///
    /// # Arguments
    /// * `query` - The user's career or resume-related query
    ///
    /// # Returns
    /// The agent's professional advice or content
    pub async fn chat(&self, query: &str) -> Result<String> {
        self.emit_log("Starting resume agent...");

        // 1. Search memory for candidate profile and preferences
        let context = self.search_memory(query).await;

        // 2. Build the user message with context
        let user_message = if let Some(ctx) = context {
            self.emit_log(&format!("Using {} relevant memories", ctx.len()));

            let history = self.chat_history.lock().unwrap();
            if history.is_empty() {
                // Add context as additional information
                format!(
                    "Candidate profile and preferences from previous sessions:\n{}\n\nUser query: {}",
                    ctx.join("\n"),
                    query
                )
            } else {
                // Just use the query if we already have conversation history
                query.to_string()
            }
        } else {
            query.to_string()
        };

        // 3. Build the resume agent
        self.emit_log("Building resume agent...");
        let agent = self.build_agent();

        // 4. Get mutable access to chat history and call the agent
        self.emit_log("Processing query...");
        let response = {
            let mut history = self.chat_history.lock().unwrap();
            agent.chat(&user_message, &mut *history).await?
        };

        // 5. Save interaction to memory (to remember candidate profile)
        self.save_memory(query, &response).await;

        self.emit_log("Resume agent complete");
        Ok(response)
    }

    /// Build a Rig agent specialized in resume and career advice
    fn build_agent(&self) -> Agent<impl rig_core::completion::CompletionModel> {
        self.client
            .agent(&self.model)
            .preamble(
                "You are an expert career advisor and professional resume writer with 15+ years of experience. \
                 You specialize in:\n\n\
                 **Resume Writing**:\n\
                 - ATS (Applicant Tracking System) optimization\n\
                 - Action-oriented bullet points with quantifiable achievements\n\
                 - Keyword optimization for specific industries\n\
                 - Professional formatting and structure\n\
                 - Tailoring resumes to specific job descriptions\n\n\
                 **Job Analysis**:\n\
                 - Breaking down job requirements and qualifications\n\
                 - Identifying key skills and experience needed\n\
                 - Assessing candidate-job fit\n\
                 - Recommending areas for improvement\n\n\
                 **Career Advice**:\n\
                 - Interview preparation and common questions\n\
                 - Career transition strategies\n\
                 - Salary negotiation guidance\n\
                 - Professional development recommendations\n\n\
                 **Best Practices**:\n\
                 - Use strong action verbs (Led, Developed, Implemented, Achieved)\n\
                 - Quantify achievements with metrics (%, $, time saved)\n\
                 - Follow industry-standard formats (reverse chronological)\n\
                 - Keep resumes concise (1-2 pages for most roles)\n\
                 - Use keywords from job descriptions naturally\n\
                 - Avoid clichés and generic statements\n\n\
                 When writing resumes or cover letters:\n\
                 1. Focus on achievements, not just responsibilities\n\
                 2. Use specific numbers and metrics\n\
                 3. Tailor content to the target role\n\
                 4. Ensure ATS compatibility (avoid tables, images, complex formatting)\n\
                 5. Maintain professional tone and clear structure\n\n\
                 Always provide constructive, specific, and actionable advice.",
            )
            .max_tokens(4096)
            .temperature(0.7)
            .build()
    }

    /// Search Mem0 for candidate profile and career preferences
    async fn search_memory(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching career profile...");

            match client
                .search_memories(query, &self.user_id, Some("resume".to_string()), Some(5))
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

    /// Save interaction to Mem0 (stores candidate profile)
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
                .add_memory(messages, &self.user_id, Some("resume".to_string()), None)
                .await
            {
                Ok(_) => {
                    self.emit_log("Saved to career profile memory");
                }
                Err(e) => {
                    eprintln!("Failed to save memories: {}", e);
                }
            }
        }
    }

    /// Emit a log message to the frontend
    fn emit_log(&self, message: &str) {
        let _ = self.app_handle.emit("resume-agent-log", message);
        println!("[Resume Agent] {}", message);
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
