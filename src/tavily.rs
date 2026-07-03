use serde::{Deserialize, Serialize};
use reqwest::Client;

#[derive(Debug, Serialize)]
struct TavilySearchRequest {
    api_key: String,
    query: String,
    search_depth: String,
    max_results: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TavilySearchResult {
    pub title: String,
    pub url: String,
    pub content: String,
    pub score: f64,
}

#[derive(Debug, Deserialize)]
struct TavilyResponse {
    results: Vec<TavilySearchResult>,
}

pub struct TavilyClient {
    api_key: String,
    client: Client,
}

impl TavilyClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: Client::new(),
        }
    }

    pub async fn search(&self, query: &str, max_results: u32) -> Result<Vec<TavilySearchResult>, Box<dyn std::error::Error>> {
        println!("Tavily search: {} (max results: {})", query, max_results);
        
        let request = TavilySearchRequest {
            api_key: self.api_key.clone(),
            query: query.to_string(),
            search_depth: "basic".to_string(),
            max_results,
        };

        let response = self.client
            .post("https://api.tavily.com/search")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            return Err(format!("Tavily API error {}: {}", status, error_text).into());
        }

        let tavily_response: TavilyResponse = response.json().await?;
        println!("Tavily returned {} results", tavily_response.results.len());
        
        Ok(tavily_response.results)
    }
}
