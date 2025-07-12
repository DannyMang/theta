use crate::models::{AIRequest, AIResponse, ContentAnalysis, ChatMessage, MessageRole};
use crate::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tauri::command]
pub async fn analyze_content(
    content: String,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<ContentAnalysis, String> {
    let app_state = state.read().await;
    
    if let Some(service) = app_state.ai_services.get("openai") {
        let request = AIRequest {
            content: format!("Analyze this content and provide a summary, keywords, sentiment (-1 to 1), topics, reading time (minutes), and complexity score (0-1): {}", content),
            context: Some("Content analysis".to_string()),
            model: Some(service.model.clone()),
            temperature: Some(0.3),
            max_tokens: Some(1000),
        };
        
        match call_ai_service(&service, &request, &app_state.http_client).await {
            Ok(response) => {
                let analysis = parse_content_analysis(&response.content);
                Ok(analysis)
            }
            Err(e) => Err(format!("AI service error: {}", e)),
        }
    } else {
        Err("No AI service available".to_string())
    }
}

#[tauri::command]
pub async fn generate_summary(
    content: String,
    max_length: Option<u32>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let app_state = state.read().await;
    
    if let Some(service) = app_state.ai_services.get("openai") {
        let max_len = max_length.unwrap_or(150);
        let request = AIRequest {
            content: format!("Summarize this content in {} words or less: {}", max_len, content),
            context: Some("Summary generation".to_string()),
            model: Some(service.model.clone()),
            temperature: Some(0.3),
            max_tokens: Some(max_len * 2),
        };
        
        match call_ai_service(&service, &request, &app_state.http_client).await {
            Ok(response) => Ok(response.content),
            Err(e) => Err(format!("AI service error: {}", e)),
        }
    } else {
        Err("No AI service available".to_string())
    }
}

#[tauri::command]
pub async fn chat_with_ai(
    message: String,
    context: Option<String>,
    model: Option<String>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<String, String> {
    let app_state = state.read().await;
    
    let service_name = model.as_deref().unwrap_or("openai");
    
    if let Some(service) = app_state.ai_services.get(service_name) {
        let request = AIRequest {
            content: message,
            context,
            model: Some(service.model.clone()),
            temperature: Some(0.7),
            max_tokens: Some(2000),
        };
        
        match call_ai_service(&service, &request, &app_state.http_client).await {
            Ok(response) => Ok(response.content),
            Err(e) => Err(format!("AI service error: {}", e)),
        }
    } else {
        Err(format!("AI service '{}' not available", service_name))
    }
}

async fn call_ai_service(
    service: &crate::state::AIService,
    request: &AIRequest,
    client: &reqwest::Client,
) -> Result<AIResponse, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    
    match service.service_type.as_str() {
        "openai" => call_openai_service(service, request, client).await,
        "anthropic" => call_anthropic_service(service, request, client).await,
        _ => Err("Unsupported AI service".into()),
    }
}

async fn call_openai_service(
    service: &crate::state::AIService,
    request: &AIRequest,
    client: &reqwest::Client,
) -> Result<AIResponse, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    
    let payload = serde_json::json!({
        "model": request.model.as_ref().unwrap_or(&service.model),
        "messages": [
            {
                "role": "user",
                "content": request.content
            }
        ],
        "temperature": request.temperature.unwrap_or(0.7),
        "max_tokens": request.max_tokens.unwrap_or(1000)
    });
    
    let response = client
        .post(&format!("{}/chat/completions", service.endpoint))
        .header("Authorization", format!("Bearer {}", service.api_key.as_ref().unwrap()))
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    
    let response_json: serde_json::Value = response.json().await?;
    
    let content = response_json["choices"][0]["message"]["content"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    let tokens_used = response_json["usage"]["total_tokens"]
        .as_u64()
        .unwrap_or(0) as u32;
    
    let processing_time = start_time.elapsed().as_millis() as u64;
    
    Ok(AIResponse {
        content,
        model: service.model.clone(),
        tokens_used,
        processing_time,
        confidence: None,
    })
}

async fn call_anthropic_service(
    service: &crate::state::AIService,
    request: &AIRequest,
    client: &reqwest::Client,
) -> Result<AIResponse, Box<dyn std::error::Error>> {
    let start_time = std::time::Instant::now();
    
    let payload = serde_json::json!({
        "model": request.model.as_ref().unwrap_or(&service.model),
        "max_tokens": request.max_tokens.unwrap_or(1000),
        "messages": [
            {
                "role": "user",
                "content": request.content
            }
        ]
    });
    
    let response = client
        .post(&format!("{}/messages", service.endpoint))
        .header("x-api-key", service.api_key.as_ref().unwrap())
        .header("anthropic-version", "2023-06-01")
        .header("Content-Type", "application/json")
        .json(&payload)
        .send()
        .await?;
    
    let response_json: serde_json::Value = response.json().await?;
    
    let content = response_json["content"][0]["text"]
        .as_str()
        .unwrap_or("")
        .to_string();
    
    let tokens_used = response_json["usage"]["output_tokens"]
        .as_u64()
        .unwrap_or(0) as u32;
    
    let processing_time = start_time.elapsed().as_millis() as u64;
    
    Ok(AIResponse {
        content,
        model: service.model.clone(),
        tokens_used,
        processing_time,
        confidence: None,
    })
}

fn parse_content_analysis(content: &str) -> ContentAnalysis {
    ContentAnalysis {
        summary: extract_field(content, "summary").unwrap_or_else(|| content.chars().take(200).collect()),
        keywords: extract_keywords(content),
        sentiment: extract_sentiment(content),
        topics: extract_topics(content),
        reading_time: estimate_reading_time(content),
        complexity_score: calculate_complexity_score(content),
    }
}

fn extract_field(content: &str, field: &str) -> Option<String> {
    content.lines()
        .find(|line| line.to_lowercase().contains(field))
        .map(|line| line.split(':').nth(1).unwrap_or("").trim().to_string())
}

fn extract_keywords(content: &str) -> Vec<String> {
    content.split_whitespace()
        .filter(|word| word.len() > 4)
        .take(10)
        .map(|word| word.to_lowercase())
        .collect()
}

fn extract_sentiment(content: &str) -> f32 {
    let positive_words = ["good", "great", "excellent", "amazing", "wonderful"];
    let negative_words = ["bad", "terrible", "awful", "horrible", "disappointing"];
    
    let words: Vec<&str> = content.split_whitespace().collect();
    let positive_count = words.iter().filter(|word| positive_words.contains(&word.to_lowercase().as_str())).count();
    let negative_count = words.iter().filter(|word| negative_words.contains(&word.to_lowercase().as_str())).count();
    
    if positive_count + negative_count == 0 {
        0.0
    } else {
        (positive_count as f32 - negative_count as f32) / (positive_count + negative_count) as f32
    }
}

fn extract_topics(content: &str) -> Vec<String> {
    vec!["Technology".to_string(), "AI".to_string(), "Browser".to_string()]
}

fn estimate_reading_time(content: &str) -> u32 {
    let words = content.split_whitespace().count();
    ((words as f32 / 200.0).ceil() as u32).max(1)
}

fn calculate_complexity_score(content: &str) -> f32 {
    let avg_word_length = content.split_whitespace()
        .map(|word| word.len())
        .sum::<usize>() as f32 / content.split_whitespace().count() as f32;
    
    (avg_word_length / 10.0).min(1.0)
} 