use crate::models::{SearchRequest, SearchResult, Bookmark, WebPageContent};
use crate::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tauri::command]
pub async fn navigate_to_url(
    url: String,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let mut app_state = state.write().await;
    
    let tab_id = app_state.create_tab(url.clone(), "Loading...".to_string());
    
    Ok(tab_id.to_string())
}

#[tauri::command]
pub async fn search_web(
    query: String,
    search_engine: Option<String>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<SearchResult>, String> {
    let app_state = state.read().await;
    
    let engine = search_engine.unwrap_or_else(|| "duckduckgo".to_string());
    
    match engine.as_str() {
        "duckduckgo" => search_duckduckgo(&query, &app_state.http_client).await,
        "google" => search_google(&query, &app_state.http_client).await,
        "bing" => search_bing(&query, &app_state.http_client).await,
        _ => Err("Unsupported search engine".to_string()),
    }
}

#[tauri::command]
pub async fn bookmark_page(
    url: String,
    title: String,
    tags: Vec<String>,
    folder: Option<String>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<Bookmark, String> {
    let app_state = state.read().await;
    
    let bookmark = Bookmark {
        id: Uuid::new_v4(),
        title,
        url,
        tags,
        created_at: chrono::Utc::now(),
        folder,
        ai_summary: None,
    };
    
    if let Some(pool) = &app_state.database {
        match save_bookmark_to_db(&bookmark, pool).await {
            Ok(_) => Ok(bookmark),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_page_content(
    url: String,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<WebPageContent, String> {
    let app_state = state.read().await;
    
    match extract_page_content(&url, &app_state.http_client).await {
        Ok(content) => Ok(content),
        Err(e) => Err(format!("Failed to extract page content: {}", e)),
    }
}

async fn search_duckduckgo(
    query: &str,
    client: &reqwest::Client,
) -> Result<Vec<SearchResult>, String> {
    let url = format!("https://api.duckduckgo.com/?q={}&format=json&no_html=1", 
                     urlencoding::encode(query));
    
    match client.get(&url).send().await {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(json) => {
                    let mut results = Vec::new();
                    
                    if let Some(results_array) = json["Results"].as_array() {
                        for result in results_array {
                            if let (Some(title), Some(url), Some(text)) = (
                                result["Text"].as_str(),
                                result["FirstURL"].as_str(),
                                result["Text"].as_str(),
                            ) {
                                results.push(SearchResult {
                                    title: title.to_string(),
                                    url: url.to_string(),
                                    snippet: text.to_string(),
                                    relevance_score: 0.8,
                                    metadata: serde_json::Value::Object(serde_json::Map::new()),
                                });
                            }
                        }
                    }
                    
                    Ok(results)
                }
                Err(e) => Err(format!("Failed to parse search results: {}", e)),
            }
        }
        Err(e) => Err(format!("Search request failed: {}", e)),
    }
}

async fn search_google(
    query: &str,
    client: &reqwest::Client,
) -> Result<Vec<SearchResult>, String> {
    let api_key = std::env::var("GOOGLE_API_KEY").map_err(|_| "Google API key not set")?;
    let cx = std::env::var("GOOGLE_CX").map_err(|_| "Google Custom Search Engine ID not set")?;
    
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}",
        api_key, cx, urlencoding::encode(query)
    );
    
    match client.get(&url).send().await {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(json) => {
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
                                    metadata: item.clone(),
                                });
                            }
                        }
                    }
                    
                    Ok(results)
                }
                Err(e) => Err(format!("Failed to parse search results: {}", e)),
            }
        }
        Err(e) => Err(format!("Search request failed: {}", e)),
    }
}

async fn search_bing(
    query: &str,
    client: &reqwest::Client,
) -> Result<Vec<SearchResult>, String> {
    let api_key = std::env::var("BING_API_KEY").map_err(|_| "Bing API key not set")?;
    
    let url = format!("https://api.bing.microsoft.com/v7.0/search?q={}", 
                     urlencoding::encode(query));
    
    match client
        .get(&url)
        .header("Ocp-Apim-Subscription-Key", api_key)
        .send()
        .await
    {
        Ok(response) => {
            match response.json::<serde_json::Value>().await {
                Ok(json) => {
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
                                    metadata: page.clone(),
                                });
                            }
                        }
                    }
                    
                    Ok(results)
                }
                Err(e) => Err(format!("Failed to parse search results: {}", e)),
            }
        }
        Err(e) => Err(format!("Search request failed: {}", e)),
    }
}

async fn extract_page_content(
    url: &str,
    client: &reqwest::Client,
) -> Result<WebPageContent, Box<dyn std::error::Error>> {
    let response = client.get(url).send().await?;
    let html = response.text().await?;
    
    let content = extract_main_content(&html);
    let title = extract_title(&html);
    let links = extract_links(&html);
    let images = extract_images(&html);
    
    Ok(WebPageContent {
        url: url.to_string(),
        title,
        content,
        html,
        links,
        images,
        metadata: serde_json::Value::Object(serde_json::Map::new()),
        extracted_at: chrono::Utc::now(),
    })
}

fn extract_main_content(html: &str) -> String {
    use regex::Regex;
    
    let re = Regex::new(r"<[^>]*>").unwrap();
    let text = re.replace_all(html, " ");
    
    text.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .chars()
        .take(10000)
        .collect()
}

fn extract_title(html: &str) -> String {
    use regex::Regex;
    
    let re = Regex::new(r"<title[^>]*>([^<]+)</title>").unwrap();
    
    if let Some(captures) = re.captures(html) {
        captures.get(1).map_or("".to_string(), |m| m.as_str().to_string())
    } else {
        "Untitled".to_string()
    }
}

fn extract_links(html: &str) -> Vec<String> {
    use regex::Regex;
    
    let re = Regex::new(r#"<a[^>]*href=["']([^"']+)["'][^>]*>"#).unwrap();
    
    re.captures_iter(html)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

fn extract_images(html: &str) -> Vec<String> {
    use regex::Regex;
    
    let re = Regex::new(r#"<img[^>]*src=["']([^"']+)["'][^>]*>"#).unwrap();
    
    re.captures_iter(html)
        .filter_map(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
        .collect()
}

async fn save_bookmark_to_db(
    bookmark: &Bookmark,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO bookmarks (id, title, url, tags, created_at, folder, ai_summary) 
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
        bookmark.id,
        bookmark.title,
        bookmark.url,
        &bookmark.tags,
        bookmark.created_at,
        bookmark.folder,
        bookmark.ai_summary
    )
    .execute(pool)
    .await?;
    
    Ok(())
} 