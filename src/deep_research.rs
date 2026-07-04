use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::llm_wrapper::LLMProviderWrapper;
use crate::tavily::TavilyClient;

/// Deep Research System using multi-agent coordination
///
/// Architecture (based on LangChain open_deep_research):
/// 1. Scope Phase: User Clarification + Brief Generation
/// 2. Research Phase: Research Supervisor + Research Sub-agents
/// 3. Write Phase: One-Shot Report Generation

// ==================== Research Brief ====================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResearchBrief {
    pub topic: String,
    pub research_questions: Vec<String>,
    pub scope: String,
    pub objectives: Vec<String>,
}

// ==================== Research Findings ====================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResearchFindings {
    pub topic: String,
    pub findings: String,
    pub sources: Vec<String>,
}

// ==================== Research Report ====================

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResearchReport {
    pub title: String,
    pub report: String,
    pub sources: Vec<String>,
}

// ==================== Deep Research Manager ====================

pub struct DeepResearchManager {
    llm_wrapper: Arc<LLMProviderWrapper>,
    tavily_api_key: String,
}

impl DeepResearchManager {
    pub fn new(llm_wrapper: Arc<LLMProviderWrapper>, tavily_api_key: String) -> Self {
        Self {
            llm_wrapper,
            tavily_api_key,
        }
    }

    /// Run simplified research (for MVP)
    pub async fn run_research(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Phase 1: Clarify and generate brief
        let brief = self.generate_brief(query).await?;

        // Phase 2: Conduct research using Tavily
        let findings = self.conduct_web_research(&brief).await?;

        // Phase 3: Generate report
        let report = self.generate_report(&brief, &findings).await?;

        Ok(report.report)
    }

    async fn generate_brief(&self, query: &str) -> Result<ResearchBrief, Box<dyn std::error::Error>> {
        // Generate research questions from the query
        let questions = vec![
            format!("What is {}?", query),
            format!("What are the key aspects of {}?", query),
            format!("What is the current state of {}?", query),
        ];

        Ok(ResearchBrief {
            topic: query.to_string(),
            research_questions: questions,
            scope: "comprehensive".to_string(),
            objectives: vec![
                "Gather current information".to_string(),
                "Identify key trends".to_string(),
                "Provide actionable insights".to_string(),
            ],
        })
    }

    async fn conduct_web_research(&self, brief: &ResearchBrief) -> Result<ResearchFindings, Box<dyn std::error::Error>> {
        let client = TavilyClient::new(self.tavily_api_key.clone());

        let mut all_findings = String::new();
        let mut all_sources = Vec::new();

        // Search for each research question
        for question in &brief.research_questions {
            match client.search(question, 3).await {
                Ok(results) => {
                    all_findings.push_str(&format!("\n## Research: {}\n\n", question));

                    for (idx, result) in results.iter().enumerate() {
                        all_findings.push_str(&format!(
                            "{}. **{}**\n{}\n\n",
                            idx + 1,
                            result.title,
                            result.content
                        ));

                        all_sources.push(format!("[{}] {}: {}",
                            all_sources.len() + 1,
                            result.title,
                            result.url
                        ));
                    }
                }
                Err(e) => {
                    eprintln!("Search error for '{}': {}", question, e);
                }
            }
        }

        Ok(ResearchFindings {
            topic: brief.topic.clone(),
            findings: all_findings,
            sources: all_sources,
        })
    }

    async fn generate_report(
        &self,
        brief: &ResearchBrief,
        findings: &ResearchFindings,
    ) -> Result<ResearchReport, Box<dyn std::error::Error>> {
        // Format the report
        let mut report = String::new();

        report.push_str(&format!("# Research Report: {}\n\n", brief.topic));
        report.push_str("## Overview\n\n");
        report.push_str(&format!("This report covers research on {}.\n\n", brief.topic));

        report.push_str("## Research Objectives\n\n");
        for (idx, obj) in brief.objectives.iter().enumerate() {
            report.push_str(&format!("{}. {}\n", idx + 1, obj));
        }
        report.push_str("\n");

        report.push_str("## Findings\n\n");
        report.push_str(&findings.findings);
        report.push_str("\n");

        report.push_str("## Sources\n\n");
        for source in &findings.sources {
            report.push_str(&format!("{}\n", source));
        }

        Ok(ResearchReport {
            title: format!("Research Report: {}", brief.topic),
            report,
            sources: findings.sources.clone(),
        })
    }

    /// Quick research for simple queries
    pub async fn run_simple_research(&self, query: &str) -> Result<String, Box<dyn std::error::Error>> {
        let client = TavilyClient::new(self.tavily_api_key.clone());
        let results = client.search(query, 5).await?;

        let mut output = format!("# Research Results: {}\n\n", query);

        for (idx, result) in results.iter().enumerate() {
            output.push_str(&format!(
                "## {}. {}\n\n{}\n\n**Source**: {}\n\n",
                idx + 1,
                result.title,
                result.content,
                result.url
            ));
        }

        Ok(output)
    }
}


/// Deep Research System using AutoAgents
///
/// Architecture:
/// 1. Scope Phase: User Clarification + Brief Generation
/// 2. Research Phase: Research Supervisor + Research Sub-agents
/// 3. Write Phase: One-Shot Report Generation

// ==================== Phase 1: Scope ====================

#[derive(Serialize, Deserialize, ToolInput, Debug)]
pub struct ClarifyArgs {
    #[input(description = "The user's research question or topic")]
    question: String,
}

#[tool(
    name = "clarify_research_scope",
    description = "Clarify the research scope with the user to understand what they want",
    input = ClarifyArgs,
)]
pub struct ClarifyTool;

#[async_trait::async_trait]
impl ToolRuntime for ClarifyTool {
    async fn execute(&self, args: Value) -> Result<Value, ToolCallError> {
        let typed_args: ClarifyArgs = serde_json::from_value(args)?;

        // In production, this would interact with the user
        // For now, return a structured clarification
        Ok(serde_json::json!({
            "clarified_topic": typed_args.question,
            "scope": "comprehensive",
            "time_frame": "current",
            "depth": "detailed"
        }))
    }
}

#[derive(Serialize, Deserialize, AgentOutput, Debug, Clone)]
pub struct ResearchBriefOutput {
    pub topic: String,
    pub research_questions: Vec<String>,
    pub scope: String,
    pub objectives: Vec<String>,
}

#[agent(
    name = "brief_generator",
    description = "Generate a detailed research brief from clarified user input",
    tools = [],
    output = ResearchBriefOutput,
)]
#[derive(Clone, AgentHooks)]
pub struct BriefGeneratorAgent;

// ==================== Phase 2: Research ====================

#[derive(Serialize, Deserialize, ToolInput, Debug)]
pub struct WebSearchArgs {
    #[input(description = "The search query")]
    query: String,

    #[input(description = "Maximum number of results to return")]
    max_results: Option<u32>,
}

#[tool(
    name = "web_search",
    description = "Search the web for information using Tavily API",
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
                let err_str = e.to_string();
                ToolCallError::RuntimeError(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    err_str
                )))
            })?;

        let mut result_text = format!("Search results for '{}':\n\n", typed_args.query);
        for (idx, result) in results.iter().enumerate() {
            result_text.push_str(&format!(
                "{}. {}\nURL: {}\nContent: {}\n\n",
                idx + 1,
                result.title,
                result.url,
                result.content
            ));
        }

        Ok(serde_json::json!({
            "results": result_text,
            "count": results.len()
        }))
    }
}

#[derive(Serialize, Deserialize, AgentOutput, Debug, Clone)]
pub struct ResearchSubAgentOutput {
    pub topic: String,
    pub findings: String,
    pub sources: Vec<String>,
}

#[agent(
    name = "research_sub_agent",
    description = "Research a specific sub-topic using web search",
    tools = [WebSearchTool],
    output = ResearchSubAgentOutput,
)]
#[derive(Clone, AgentHooks)]
pub struct ResearchSubAgent {
    tavily_api_key: String,
}

impl ResearchSubAgent {
    pub fn new(tavily_api_key: String) -> Self {
        Self { tavily_api_key }
    }
}

#[derive(Serialize, Deserialize, AgentOutput, Debug, Clone)]
pub struct ResearchSupervisorOutput {
    pub research_complete: bool,
    pub all_findings: String,
    pub sources: Vec<String>,
}

#[agent(
    name = "research_supervisor",
    description = "Supervise the research process by coordinating multiple sub-agents",
    tools = [],
    output = ResearchSupervisorOutput,
)]
#[derive(Clone, AgentHooks)]
pub struct ResearchSupervisorAgent;

// ==================== Phase 3: Write ====================

#[derive(Serialize, Deserialize, AgentOutput, Debug, Clone)]
pub struct ResearchReportOutput {
    pub title: String,
    pub report: String,
    pub sources: Vec<String>,
}

#[agent(
    name = "report_generator",
    description = "Generate a comprehensive research report from findings",
    tools = [],
    output = ResearchReportOutput,
)]
#[derive(Clone, AgentHooks)]
pub struct ReportGeneratorAgent;

// ==================== Deep Research Manager ====================

pub struct DeepResearchManager {
    llm_wrapper: Arc<LLMProviderWrapper>,
    tavily_api_key: String,
}

impl DeepResearchManager {
    pub fn new(llm_wrapper: Arc<LLMProviderWrapper>, tavily_api_key: String) -> Self {
        Self {
            llm_wrapper,
            tavily_api_key,
        }
    }

    /// Run the full deep research pipeline
    pub async fn run_deep_research(&self, user_query: &str) -> Result<ResearchReportOutput, AutoAgentsError> {
        // Phase 1: Scope - Generate Research Brief
        let brief = self.generate_brief(user_query).await?;

        // Phase 2: Research - Coordinate sub-agents
        let research_findings = self.conduct_research(&brief).await?;

        // Phase 3: Write - Generate final report
        let report = self.generate_report(&brief, &research_findings).await?;

        Ok(report)
    }

    async fn generate_brief(&self, user_query: &str) -> Result<ResearchBriefOutput, AutoAgentsError> {
        let brief_agent = BriefGeneratorAgent;
        let memory = Box::new(SlidingWindowMemory::new(5));

        let agent_handle = AgentBuilder::<_, DirectAgent>::new(ReActAgent::new(brief_agent))
            .llm(self.llm_wrapper.provider())
            .memory(memory)
            .build()
            .await?;

        let task = Task::new(user_query);
        let result = agent_handle.agent.run(task).await?;

        Ok(result)
    }

    async fn conduct_research(&self, brief: &ResearchBriefOutput) -> Result<ResearchSupervisorOutput, AutoAgentsError> {
        let supervisor = ResearchSupervisorAgent;
        let memory = Box::new(SlidingWindowMemory::new(20));

        let agent_handle = AgentBuilder::<_, DirectAgent>::new(ReActAgent::new(supervisor))
            .llm(self.llm_wrapper.provider())
            .memory(memory)
            .build()
            .await?;

        let research_prompt = format!(
            "Research the following topic: {}\nResearch questions: {:?}\nObjectives: {:?}",
            brief.topic, brief.research_questions, brief.objectives
        );

        let task = Task::new(&research_prompt);
        let result = agent_handle.agent.run(task).await?;

        Ok(result)
    }

    async fn generate_report(
        &self,
        brief: &ResearchBriefOutput,
        findings: &ResearchSupervisorOutput,
    ) -> Result<ResearchReportOutput, AutoAgentsError> {
        let report_agent = ReportGeneratorAgent;
        let memory = Box::new(SlidingWindowMemory::new(10));

        let agent_handle = AgentBuilder::<_, DirectAgent>::new(ReActAgent::new(report_agent))
            .llm(self.llm_wrapper.provider())
            .memory(memory)
            .build()
            .await?;

        let report_prompt = format!(
            "Generate a comprehensive research report.\n\nTopic: {}\n\nFindings:\n{}\n\nSources: {:?}",
            brief.topic, findings.all_findings, findings.sources
        );

        let task = Task::new(&report_prompt);
        let result = agent_handle.agent.run(task).await?;

        Ok(result)
    }

    /// Simplified research for quick queries
    pub async fn run_simple_research(&self, query: &str) -> Result<String, AutoAgentsError> {
        let sub_agent = ResearchSubAgent::new(self.tavily_api_key.clone());
        let memory = Box::new(SlidingWindowMemory::new(10));

        let agent_handle = AgentBuilder::<_, DirectAgent>::new(ReActAgent::new(sub_agent))
            .llm(self.llm_wrapper.provider())
            .memory(memory)
            .build()
            .await?;

        let task = Task::new(query);
        let result = agent_handle.agent.run(task).await?;

        Ok(result.findings)
    }
}
