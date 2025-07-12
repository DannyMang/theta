use reqwest::Client;
use serde_json::Value;
use urlencoding::encode;

pub struct SearchEngine {
    client: Client,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub title: String,
    pub url: String,
    pub snippet: String,
    pub relevance_score: f32,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub async fn search_duckduckgo(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let url = format!("https://api.duckduckgo.com/?q={}&format=json&no_html=1", encode(query));
        
        let response = self.client
            .get(&url)
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
            .send()
            .await
            .map_err(|e| format!("Search request failed: {}", e))?;

        let json: Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut results = Vec::new();
        
        // Check RelatedTopics for results
        if let Some(related) = json["RelatedTopics"].as_array() {
            for item in related {
                if let (Some(text), Some(url)) = (item["Text"].as_str(), item["FirstURL"].as_str()) {
                    results.push(SearchResult {
                        title: text.split(" - ").next().unwrap_or(text).to_string(),
                        url: url.to_string(),
                        snippet: text.to_string(),
                        relevance_score: 0.8,
                    });
                }
            }
        }

        // Check Abstract for main result
        if let (Some(abstract_text), Some(abstract_url)) = (json["Abstract"].as_str(), json["AbstractURL"].as_str()) {
            if !abstract_text.is_empty() {
                results.insert(0, SearchResult {
                    title: json["Heading"].as_str().unwrap_or("DuckDuckGo Result").to_string(),
                    url: abstract_url.to_string(),
                    snippet: abstract_text.to_string(),
                    relevance_score: 0.9,
                });
            }
        }

        Ok(results)
    }

    pub async fn search_google(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let api_key = std::env::var("GOOGLE_API_KEY").map_err(|_| "Google API key not set")?;
        let cx = std::env::var("GOOGLE_CX").map_err(|_| "Google Custom Search Engine ID not set")?;
        
        let url = format!(
            "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
            api_key, cx, encode(query)
        );
        
        let response = self.client
            .get(&url)
            .send()
            .await
            .map_err(|e| format!("Search request failed: {}", e))?;

        let json: Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut results = Vec::new();
        
        if let Some(items) = json["items"].as_array() {
            for item in items {
                if let (Some(title), Some(link), Some(snippet)) = (
                    item["title"].as_str(),
                    item["link"].as_str(),
                    item["snippet"].as_str(),
                ) {
                    results.push(SearchResult {
                        title: title.to_string(),
                        url: link.to_string(),
                        snippet: snippet.to_string(),
                        relevance_score: 0.9,
                    });
                }
            }
        }
        
        Ok(results)
    }

    pub async fn search_bing(&self, query: &str) -> Result<Vec<SearchResult>, String> {
        let api_key = std::env::var("BING_API_KEY").map_err(|_| "Bing API key not set")?;
        
        let url = format!("https://api.bing.microsoft.com/v7.0/search?q={}", encode(query));
        
        let response = self.client
            .get(&url)
            .header("Ocp-Apim-Subscription-Key", api_key)
            .send()
            .await
            .map_err(|e| format!("Search request failed: {}", e))?;

        let json: Value = response.json().await.map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut results = Vec::new();
        
        if let Some(web_pages) = json["webPages"]["value"].as_array() {
            for page in web_pages {
                if let (Some(name), Some(url), Some(snippet)) = (
                    page["name"].as_str(),
                    page["url"].as_str(),
                    page["snippet"].as_str(),
                ) {
                    results.push(SearchResult {
                        title: name.to_string(),
                        url: url.to_string(),
                        snippet: snippet.to_string(),
                        relevance_score: 0.85,
                    });
                }
            }
        }
        
        Ok(results)
    }

    pub async fn search(&self, query: &str, engine: &str) -> Result<Vec<SearchResult>, String> {
        match engine {
            "google" => self.search_google(query).await,
            "bing" => self.search_bing(query).await,
            "duckduckgo" | _ => self.search_duckduckgo(query).await,
        }
    }
} 