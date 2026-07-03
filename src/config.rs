use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
    pub system_prompt: String,
    pub tavily_api_key: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.openai.com/v1".to_string(),
            model: "gpt-4".to_string(),
            system_prompt: "You are a helpful assistant.".to_string(),
            tavily_api_key: String::new(),
        }
    }
}

impl AppConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        println!("Loading configuration...");
        
        // Try to load from .env file first
        if let Ok(_) = dotenvy::dotenv() {
            println!(".env file found, loading environment variables");
            
            let api_key = std::env::var("API_KEY").unwrap_or_default();
            let base_url = std::env::var("BASE_URL")
                .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
            let model = std::env::var("MODEL").unwrap_or_else(|_| "gpt-4".to_string());
            let system_prompt = std::env::var("SYSTEM_PROMPT")
                .unwrap_or_else(|_| "You are a helpful assistant.".to_string())
                .trim_matches('"')  // Remove surrounding quotes
                .to_string();
            let tavily_api_key = std::env::var("TAVILY_API_KEY").unwrap_or_default();
            
            println!("Config loaded - API Key: {}, Base URL: {}, Model: {}, Tavily: {}", 
                if api_key.is_empty() { "EMPTY" } else { "SET" },
                base_url,
                model,
                if tavily_api_key.is_empty() { "EMPTY" } else { "SET" }
            );
            
            return Ok(Self {
                api_key,
                base_url,
                model,
                system_prompt,
                tavily_api_key,
            });
        }
        
        println!("No .env file found, using defaults");
        Ok(Self::default())
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let env_content = format!(
            "API_KEY={}\nBASE_URL={}\nMODEL={}\nSYSTEM_PROMPT=\"{}\"\nTAVILY_API_KEY={}",
            self.api_key, self.base_url, self.model, self.system_prompt, self.tavily_api_key
        );
        fs::write(".env", env_content)?;
        println!("Configuration saved to .env");
        Ok(())
    }
}

