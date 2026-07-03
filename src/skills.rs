use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub instructions: String,
    pub parameters: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillMetadata {
    pub name: String,
    pub description: String,
    pub parameters: Option<Vec<SkillParameter>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillParameter {
    pub name: String,
    pub r#type: String,
    pub description: String,
    pub required: bool,
}

pub struct SkillLoader {
    skills_dir: PathBuf,
}

impl SkillLoader {
    pub fn new(skills_dir: PathBuf) -> Self {
        Self { skills_dir }
    }

    /// Get the skills directory path
    pub fn skills_dir(&self) -> &Path {
        &self.skills_dir
    }

    /// Load all skills from the skills directory
    pub fn load_skills(&self) -> Result<Vec<Skill>, Box<dyn Error>> {
        let mut skills = Vec::new();

        if !self.skills_dir.exists() {
            fs::create_dir_all(&self.skills_dir)?;
            return Ok(skills);
        }

        for entry in fs::read_dir(&self.skills_dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("md") {
                if let Ok(skill) = self.load_skill(&path) {
                    skills.push(skill);
                }
            }
        }

        Ok(skills)
    }

    /// Load a single skill from a markdown file
    fn load_skill(&self, path: &Path) -> Result<Skill, Box<dyn Error>> {
        let content = fs::read_to_string(path)?;
        let (metadata, instructions) = self.parse_skill_markdown(&content)?;

        // Build JSON schema for parameters
        let parameters = if let Some(params) = metadata.parameters {
            let mut properties = serde_json::Map::new();
            let mut required = Vec::new();

            for param in params {
                properties.insert(
                    param.name.clone(),
                    serde_json::json!({
                        "type": param.r#type,
                        "description": param.description
                    }),
                );

                if param.required {
                    required.push(param.name);
                }
            }

            serde_json::json!({
                "type": "object",
                "properties": properties,
                "required": required
            })
        } else {
            serde_json::json!({
                "type": "object",
                "properties": {}
            })
        };

        Ok(Skill {
            name: metadata.name,
            description: metadata.description,
            instructions,
            parameters,
        })
    }

    /// Parse skill markdown format with YAML frontmatter
    fn parse_skill_markdown(&self, content: &str) -> Result<(SkillMetadata, String), Box<dyn Error>> {
        let lines: Vec<&str> = content.lines().collect();

        // Check for YAML frontmatter
        if lines.first() == Some(&"---") {
            // Find closing ---
            let mut frontmatter_end = 0;
            for (i, line) in lines.iter().enumerate().skip(1) {
                if *line == "---" {
                    frontmatter_end = i;
                    break;
                }
            }

            if frontmatter_end > 0 {
                let frontmatter = lines[1..frontmatter_end].join("\n");
                let instructions = lines[frontmatter_end + 1..].join("\n").trim().to_string();

                let metadata: SkillMetadata = serde_yaml::from_str(&frontmatter)
                    .map_err(|e| format!("Failed to parse frontmatter: {}", e))?;

                return Ok((metadata, instructions));
            }
        }

        // Fallback: use filename as name and first paragraph as description
        Err("Invalid skill format: missing frontmatter".into())
    }
}

/// Convert a skill into a tool definition for LLM
pub fn skill_to_tool(skill: &Skill) -> crate::llm::Tool {
    crate::llm::Tool {
        r#type: "function".to_string(),
        function: crate::llm::ToolFunction {
            name: skill.name.clone(),
            description: skill.description.clone(),
            parameters: skill.parameters.clone(),
        },
    }
}
