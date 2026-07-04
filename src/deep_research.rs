use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::tavily::TavilyClient;

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
}

impl DeepResearchManager {
    pub fn new(tavily_api_key: String) -> Self {
        Self { tavily_api_key }
    }

    /// Run full deep research pipeline
    pub async fn run_research(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Phase 1: Scope - Generate research questions
        let brief = self.generate_brief(query);

        // Phase 2: Research - Search for each question
        let findings = self.conduct_research(&brief).await?;

        // Phase 3: Write - Generate report
        let report = self.generate_report(&brief, &findings);

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

        // Research each question
        for (idx, question) in brief.research_questions.iter().enumerate() {
            all_findings.push_str(&format!("\n## Research Question {}: {}\n\n", idx + 1, question));

            match client.search(question, 3).await {
                Ok(results) => {
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
                    eprintln!("Search error for '{}': {}", question, e);
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
