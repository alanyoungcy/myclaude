use serde::{Deserialize, Serialize};
use tauri::Emitter;

use crate::llm::{LLMClient, ChatRequest, Message as LLMMessage};
use crate::mem0::{Mem0Client, Message as Mem0Message};

/// Resume Agent System
///
/// A specialized agent for creating tailored resumes and job applications
/// Focuses on ATS-friendly formatting and job-specific optimization

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CandidateProfile {
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub location: Option<String>,
    pub links: std::collections::HashMap<String, String>,
    pub summary: Option<String>,
    pub skills: Vec<String>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub certifications: Vec<String>,
    pub projects: Vec<Project>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Experience {
    pub company: String,
    pub title: String,
    pub dates: String,
    pub bullets: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Education {
    pub institution: String,
    pub degree: String,
    pub field: Option<String>,
    pub dates: String,
    pub achievements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct Project {
    pub name: String,
    pub description: String,
    pub technologies: Vec<String>,
    pub achievements: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[allow(dead_code)]
pub struct JobTarget {
    pub title: String,
    pub company: String,
    pub description: String,
    pub region: String,
    pub mode: String, // "resume", "cover_letter", "both", "analysis"
}

#[derive(Debug, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct JobAnalysis {
    pub job_summary: String,
    pub top_keywords: Vec<String>,
    pub required_skills: Vec<String>,
    pub preferred_skills: Vec<String>,
    pub fit_strengths: Vec<String>,
    pub fit_gaps: Vec<String>,
    pub missing_information: Vec<String>,
    pub resume_strategy: String,
    pub cover_letter_strategy: String,
}

pub struct ResumeAgent {
    llm_client: LLMClient,
    app_handle: tauri::AppHandle,
    mem0_client: Option<Mem0Client>,
    user_id: String,
}

impl ResumeAgent {
    pub fn new(
        base_url: String,
        api_key: String,
        app_handle: tauri::AppHandle,
        mem0_api_key: Option<String>,
        user_id: String,
    ) -> Self {
        let llm_client = LLMClient::new(base_url, api_key);
        let mem0_client = mem0_api_key.map(|key| Mem0Client::new(key));

        Self {
            llm_client,
            app_handle,
            mem0_client,
            user_id,
        }
    }

    fn emit_log(&self, message: &str) {
        let _ = self.app_handle.emit("resume-agent-log", message);
        println!("[ResumeAgent] {}", message);
    }

    fn get_system_prompt() -> String {
        r#"You are a resume and job application agent.

Your purpose is to help the user create accurate, ATS-friendly, job-specific application materials, including a tailored resume, cover letter, application answers, and improvement suggestions.

Primary responsibilities:
1. Gather the minimum required inputs.
2. Analyze the target job description.
3. Tailor the user's experience to the role without inventing facts.
4. Generate polished, concise, professional application materials.
5. Review outputs for ATS compatibility, clarity, and evidence strength.

Operating rules:
- Never fabricate employers, job titles, dates, metrics, education, certifications, projects, or achievements.
- You may improve wording, prioritization, and framing, but not the underlying facts.
- If metrics are missing, prefer strong qualitative outcomes over fake numbers.
- Mirror important keywords from the job description naturally.
- Optimize for ATS readability: standard section headings, simple structure, no tables, no icons, no text boxes.
- Tailor content to the target role and seniority.
- Prefer evidence over adjectives. Replace vague claims with concrete actions and outcomes.
- Keep tone professional, direct, and specific.

Resume generation rules:
- Start with contact/header info.
- Use strong action verbs.
- Prefer bullet points over paragraphs.
- Use CAR-style bullets: context/problem, action, result.
- Aim for measurable impact.

Cover letter rules:
- Keep it concise, usually 3 to 4 paragraphs.
- Opening: why this role, why this company, why now.
- Middle: 2 to 3 strongest matches with evidence.
- Closing: interest, fit, and next-step language.

Output format:
- Always use markdown formatting
- Use proper headings (# for sections)
- Use bullet points for lists
- Use **bold** for emphasis
- Make it easy to read and copy"#.to_string()
    }

    /// Search for relevant memories (previous resumes, preferences)
    async fn search_relevant_memories(&self, query: &str) -> Option<Vec<String>> {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Searching previous resume preferences...");

            match client.search_memories(
                query,
                &self.user_id,
                Some("resume".to_string()),
                Some(5)
            ).await {
                Ok(response) => {
                    if !response.results.is_empty() {
                        let memories: Vec<String> = response.results
                            .iter()
                            .map(|r| r.memory.clone())
                            .collect();

                        self.emit_log(&format!("Found {} previous preferences", memories.len()));
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

    /// Save resume session to memory
    async fn save_to_memory(&self, conversation: Vec<Mem0Message>) {
        if let Some(client) = &self.mem0_client {
            self.emit_log("Saving resume preferences to memory...");

            match client.add_memory(
                conversation,
                &self.user_id,
                Some("resume".to_string()),
                Some(format!("resume-{}", chrono::Utc::now().timestamp()))
            ).await {
                Ok(response) => {
                    if let Some(status) = &response.status {
                        if status == "PENDING" {
                            self.emit_log("Memory extraction queued (processing async)");
                        }
                    } else if let Some(memories) = &response.memories {
                        if !memories.is_empty() {
                            self.emit_log(&format!("Learned {} preferences", memories.len()));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to save memories: {}", e);
                }
            }
        }
    }

    /// Analyze job description
    #[allow(dead_code)]
    pub async fn analyze_job(
        &self,
        profile: &CandidateProfile,
        job: &JobTarget,
        model: &str,
    ) -> Result<JobAnalysis, String> {
        self.emit_log("Analyzing job description...");

        let prompt = format!(
            r#"Analyze this job description and candidate profile.

Return JSON with:
- job_summary (string)
- top_keywords (array of strings)
- required_skills (array of strings)
- preferred_skills (array of strings)
- fit_strengths (array of strings)
- fit_gaps (array of strings)
- missing_information (array of strings)
- resume_strategy (string)
- cover_letter_strategy (string)

JOB:
Title: {}
Company: {}
Region: {}
Description:
{}

CANDIDATE PROFILE:
{}
"#,
            job.title,
            job.company,
            job.region,
            job.description,
            serde_json::to_string_pretty(profile).unwrap_or_default()
        );

        let messages = vec![
            LLMMessage {
                role: "system".to_string(),
                content: Some(Self::get_system_prompt()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
            LLMMessage {
                role: "user".to_string(),
                content: Some(prompt),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
        ];

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            tools: None,
            stream: None,
        };

        let response = self.llm_client.send_message(request).await
            .map_err(|e| e.to_string())?;

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.as_ref())
            .ok_or("No response from LLM")?;

        // Parse JSON response
        serde_json::from_str::<JobAnalysis>(content)
            .map_err(|e| format!("Failed to parse job analysis: {}", e))
    }

    /// Generate tailored resume
    #[allow(dead_code)]
    pub async fn generate_resume(
        &self,
        profile: &CandidateProfile,
        _job: &JobTarget,
        analysis: &JobAnalysis,
        model: &str,
    ) -> Result<String, String> {
        self.emit_log("Generating tailored resume...");

        let prompt = format!(
            r#"Create an ATS-friendly tailored resume for this role.

Requirements:
- Use only facts from the candidate profile
- Mirror relevant job keywords naturally
- Prioritize relevant experience
- Use strong bullet points with CAR structure
- Avoid tables, icons, and decorative formatting
- Output in clean markdown format

JOB ANALYSIS:
{}

CANDIDATE PROFILE:
{}
"#,
            serde_json::to_string_pretty(analysis).unwrap_or_default(),
            serde_json::to_string_pretty(profile).unwrap_or_default()
        );

        let messages = vec![
            LLMMessage {
                role: "system".to_string(),
                content: Some(Self::get_system_prompt()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
            LLMMessage {
                role: "user".to_string(),
                content: Some(prompt),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
        ];

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            tools: None,
            stream: None,
        };

        let response = self.llm_client.send_message(request).await
            .map_err(|e| e.to_string())?;

        response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or("No resume generated".to_string())
    }

    /// Generate cover letter
    #[allow(dead_code)]
    pub async fn generate_cover_letter(
        &self,
        profile: &CandidateProfile,
        job: &JobTarget,
        analysis: &JobAnalysis,
        model: &str,
    ) -> Result<String, String> {
        self.emit_log("Generating cover letter...");

        let prompt = format!(
            r#"Write a tailored cover letter for this role.

Requirements:
- 3 to 4 paragraphs
- Specific opening (why this role, why this company)
- 2 to 3 strongest evidence-based matches to job requirements
- Professional and concise
- No fabricated claims
- Output in clean markdown format

JOB:
Title: {}
Company: {}
Description: {}

JOB ANALYSIS:
{}

CANDIDATE PROFILE:
{}
"#,
            job.title,
            job.company,
            job.description,
            serde_json::to_string_pretty(analysis).unwrap_or_default(),
            serde_json::to_string_pretty(profile).unwrap_or_default()
        );

        let messages = vec![
            LLMMessage {
                role: "system".to_string(),
                content: Some(Self::get_system_prompt()),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
            LLMMessage {
                role: "user".to_string(),
                content: Some(prompt),
                tool_calls: None,
                tool_call_id: None,
                name: None,
            },
        ];

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            tools: None,
            stream: None,
        };

        let response = self.llm_client.send_message(request).await
            .map_err(|e| e.to_string())?;

        response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or("No cover letter generated".to_string())
    }

    /// Main entry point for resume agent
    pub async fn process_request(
        &self,
        user_message: &str,
        model: &str,
    ) -> Result<String, String> {
        self.emit_log("Resume agent starting...");

        // Search for memories
        let memories = self.search_relevant_memories(user_message).await;

        let mut system_prompt = Self::get_system_prompt();

        // Add memory context
        if let Some(mem_list) = &memories {
            if !mem_list.is_empty() {
                system_prompt.push_str("\n\n## User Preferences:\n\n");
                for (i, memory) in mem_list.iter().enumerate() {
                    system_prompt.push_str(&format!("{}. {}\n", i + 1, memory));
                }
            }
        }

        // Build conversation
        let messages = vec![
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

        let request = ChatRequest {
            model: model.to_string(),
            messages,
            tools: None,
            stream: None,
        };

        let response = self.llm_client.send_message(request).await
            .map_err(|e| e.to_string())?;

        let final_response = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .ok_or("No response generated".to_string())?;

        // Save to memory
        let conversation = vec![
            Mem0Message {
                role: "user".to_string(),
                content: user_message.to_string(),
            },
            Mem0Message {
                role: "assistant".to_string(),
                content: final_response.clone(),
            },
        ];

        self.save_to_memory(conversation).await;

        self.emit_log("Resume agent completed");

        Ok(final_response)
    }
}
