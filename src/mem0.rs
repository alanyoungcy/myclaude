use serde::{Deserialize, Serialize};
use reqwest;

/// Mem0 API Client
///
/// A reusable client for interacting with Mem0 memory service.
/// Provides methods to add, search, retrieve, and delete memories.

#[derive(Debug, Clone)]
pub struct Mem0Client {
    api_key: String,
    base_url: String,
    client: reqwest::Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize)]
struct AddMemoryRequest {
    messages: Vec<Message>,
    user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct AddMemoryResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memories: Option<Vec<MemoryItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct MemoryItem {
    pub id: String,
    pub memory: String,
    pub hash: String,
    pub metadata: Option<serde_json::Value>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Serialize)]
struct SearchMemoriesRequest {
    query: String,
    user_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    filters: Option<SearchFilters>,
}

#[derive(Debug, Serialize)]
struct SearchFilters {
    #[serde(skip_serializing_if = "Option::is_none")]
    agent_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct SearchMemoriesResponse {
    pub results: Vec<SearchResult>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SearchResult {
    pub id: String,
    pub memory: String,
    pub hash: String,
    pub metadata: Option<serde_json::Value>,
    pub score: f64,
    pub created_at: String,
    pub updated_at: String,
}

impl Mem0Client {
    /// Create a new Mem0 client
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            base_url: "https://api.mem0.ai/v3".to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Add memories from a conversation
    ///
    /// # Arguments
    /// * `messages` - Conversation messages to extract memories from
    /// * `user_id` - Unique user identifier
    /// * `agent_id` - Optional agent identifier (e.g., "research", "code")
    /// * `run_id` - Optional run/session identifier
    pub async fn add_memory(
        &self,
        messages: Vec<Message>,
        user_id: &str,
        agent_id: Option<String>,
        run_id: Option<String>,
    ) -> Result<AddMemoryResponse, String> {
        let url = format!("{}/memories/add/", self.base_url);

        let request = AddMemoryRequest {
            messages,
            user_id: user_id.to_string(),
            agent_id,
            run_id,
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Mem0 API error ({}): {}", status, error_text));
        }

        // Try to get the response text first for debugging
        let response_text = response.text().await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        println!("[Mem0 Debug] Add memory response: {}", response_text);

        // Try to parse the response
        serde_json::from_str::<AddMemoryResponse>(&response_text)
            .map_err(|e| format!("Failed to parse response: {}. Response was: {}", e, response_text))
    }

    /// Search for relevant memories
    ///
    /// # Arguments
    /// * `query` - Search query
    /// * `user_id` - User identifier
    /// * `agent_id` - Optional agent identifier to filter memories
    /// * `limit` - Maximum number of results (default: 10)
    pub async fn search_memories(
        &self,
        query: &str,
        user_id: &str,
        agent_id: Option<String>,
        limit: Option<u32>,
    ) -> Result<SearchMemoriesResponse, String> {
        let url = format!("{}/memories/search/", self.base_url);

        let filters = if agent_id.is_some() {
            Some(SearchFilters {
                agent_id: agent_id.clone(),
            })
        } else {
            None
        };

        let request = SearchMemoriesRequest {
            query: query.to_string(),
            user_id: user_id.to_string(),
            agent_id: None, // Don't use top-level agent_id
            limit,
            filters,
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| format!("Failed to send request: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Mem0 API error ({}): {}", status, error_text));
        }

        response
            .json::<SearchMemoriesResponse>()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_mem0_client_creation() {
        let client = Mem0Client::new("test-key".to_string());
        assert_eq!(client.api_key, "test-key");
        assert_eq!(client.base_url, "https://api.mem0.ai/v3");
    }
}
