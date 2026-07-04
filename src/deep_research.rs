use serde::{Deserialize, Serialize};
use tauri::Emitter;

use crate::tavily::TavilyClient;
use crate::mem0::{Mem0Client, Message as Mem0Message};

/// Deep Research System
///
/// Simplified implementation inspired by LangChain's open_deep_research
/// Architecture:
/// 1. Scope Phase: Generate research questions
/// 2. Research Phase: Web search for each question
/// 3. Write Phase: Compile findings into report

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResearchBrief {
    pub topic: String,
    pub research_questions: Vec<String>,
    pub scope: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResearchFindings {
    pub findings: String,
    pub sources: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResearchReport {
    pub title: String,
    pub content: String,
    pub sources: Vec<String>,
}

pub struct DeepResearchManager {
    tavily_api_key: String,
    app_handle: tauri::AppHandle,
    mem0_client: Option<Mem0Client>,
    user_id: String,
}

impl DeepResearchManager {
    pub fn new(tavily_api_key: String, app_handle: tauri::AppHandle, mem0_api_key: Option<String>, user_id: String) -> Self {
        let mem0_client = mem0_api_key.map(|key| Mem0Client::new(key));

        Self {
            tavily_api_key,
            app_handle,
            mem0_client,
            user_id,
        }
    }

    fn emit_log(&self, message: &str) {
        let _ = self.app_handle.emit("research-log", message);
        println!("{}", message);
    }

    /// Search for relevant memories before starting research
    async fn search_relevant_memories(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching relevant memories...");

            match client.search_memories(
                query,
                &self.user_id,
                Some("research".to_string()),
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

    /// Save the research conversation to memory
    /// Mem0 will automatically extract key facts and relationships
    async fn save_to_memory(&self, query: &str, result: &str) {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Saving research insights to memory...");

            let messages = vec![
                Mem0Message {
                    role: "user".to_string(),
                    content: query.to_string(),
                },
                Mem0Message {
                    role: "assistant".to_string(),
                    content: result.to_string(),
                },
            ];

            match client.add_memory(
                messages,
                &self.user_id,
                Some("research".to_string()),
                Some(format!("research-{}", chrono::Utc::now().timestamp()))
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

    /// Run full deep research pipeline
    pub async fn run_research(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Search for relevant memories first
        let memories = self.search_relevant_memories(query).await;

        self.emit_log("Phase 1: Generating research questions...");

        // Phase 1: Scope - Generate research questions
        let mut brief = self.generate_brief(query);

        // Enhance research questions with memory context
        if let Some(mem_list) = &memories {
            if !mem_list.is_empty() {
                self.emit_log("Enhancing research plan with previous insights");
                // Add context to the brief
                brief.scope = format!(
                    "{}\n\nPrevious insights to consider:\n{}",
                    brief.scope,
                    mem_list.iter()
                        .map(|m| format!("- {}", m))
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
        }

        self.emit_log(&format!("Generated {} research questions", brief.research_questions.len()));

        // Phase 2: Research - Search for each question
        self.emit_log("Phase 2: Conducting research...");
        let findings = self.conduct_research(&brief).await?;

        // Phase 3: Write - Generate report
        self.emit_log("Phase 3: Generating report...");
        let report = self.generate_report(&brief, &findings);

        self.emit_log("Research completed successfully");

        // Save to memory - Mem0 will extract key insights
        self.save_to_memory(query, &report.content).await;

        Ok(report.content)
    }

    fn generate_brief(&self, query: &str) -> ResearchBrief {
        // Generate research questions from the main query
        let questions = vec![
            format!("What is the current state of {}?", query),
            format!("What are the key trends in {}?", query),
            format!("What are the main challenges and opportunities in {}?", query),
        ];

        ResearchBrief {
            topic: query.to_string(),
            research_questions: questions,
            scope: "comprehensive".to_string(),
        }
    }

    async fn conduct_research(&self, brief: &ResearchBrief) -> Result<ResearchFindings, Box<dyn std::error::Error>> {
        let client = TavilyClient::new(self.tavily_api_key.clone());

        let mut all_findings = String::new();
        let mut all_sources = Vec::new();

        // Research each question with advanced mode (10 results per question)
        for (idx, question) in brief.research_questions.iter().enumerate() {
            self.emit_log(&format!("Searching: {} (question {}/{})", question, idx + 1, brief.research_questions.len()));

            all_findings.push_str(&format!("\n## Research Question {}: {}\n\n", idx + 1, question));

            match client.search(question, 10).await {
                Ok(results) => {
                    self.emit_log(&format!("Found {} results for question {}", results.len(), idx + 1));

                    for result in results {
                        all_findings.push_str(&format!(
                            "**{}**\n\n{}\n\n",
                            result.title,
                            result.content
                        ));

                        all_sources.push(format!("{}: {}", result.title, result.url));
                    }
                }
                Err(e) => {
                    let error_msg = format!("Search error for question {}: {}", idx + 1, e);
                    self.emit_log(&error_msg);
                    eprintln!("{}", error_msg);
                    all_findings.push_str("*No results found for this question.*\n\n");
                }
            }
        }

        Ok(ResearchFindings {
            findings: all_findings,
            sources: all_sources,
        })
    }

    fn generate_report(&self, brief: &ResearchBrief, findings: &ResearchFindings) -> ResearchReport {
        let mut content = String::new();

        // Title
        content.push_str(&format!("# Research Report: {}\n\n", brief.topic));

        // Overview
        content.push_str("## Overview\n\n");
        content.push_str(&format!(
            "This comprehensive research report covers **{}**. ",
            brief.topic
        ));
        content.push_str("The research was conducted using multiple sources to provide a thorough analysis.\n\n");

        // Research Findings
        content.push_str("## Research Findings\n\n");
        content.push_str(&findings.findings);

        // Sources
        if !findings.sources.is_empty() {
            content.push_str("\n## Sources\n\n");
            for (idx, source) in findings.sources.iter().enumerate() {
                content.push_str(&format!("{}. {}\n", idx + 1, source));
            }
        }

        // Conclusion
        content.push_str("\n## Conclusion\n\n");
        content.push_str(&format!(
            "This research provides a comprehensive overview of {}. ",
            brief.topic
        ));
        content.push_str("The findings are based on current information from multiple authoritative sources.\n");

        ResearchReport {
            title: format!("Research Report: {}", brief.topic),
            content,
            sources: findings.sources.clone(),
        }
    }
}
