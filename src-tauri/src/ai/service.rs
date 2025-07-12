use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;

pub struct AIServiceManager {
    pub services: HashMap<String, AIService>,
    pub client: Client,
}

#[derive(Clone)]
pub struct AIService {
    pub name: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub model: String,
}

impl AIServiceManager {
    pub fn new() -> Self {
        Self {
            services: HashMap::new(),
            client: Client::new(),
        }
    }

    pub fn add_service(&mut self, name: String, service: AIService) {
        self.services.insert(name, service);
    }

    pub async fn call_service(&self, service_name: &str, prompt: &str) -> Result<String, String> {
        if let Some(service) = self.services.get(service_name) {
            match service.name.as_str() {
                "openai" => self.call_openai(service, prompt).await,
                "anthropic" => self.call_anthropic(service, prompt).await,
                _ => Err("Unknown service".to_string()),
            }
        } else {
            Err("Service not found".to_string())
        }
    }

    async fn call_openai(&self, service: &AIService, prompt: &str) -> Result<String, String> {
        let payload = serde_json::json!({
            "model": service.model,
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        });

        let response = self.client
            .post(&format!("{}/chat/completions", service.endpoint))
            .header("Authorization", format!("Bearer {}", service.api_key.as_ref().unwrap_or(&"".to_string())))
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: Value = response.json().await.map_err(|e| e.to_string())?;
        
        Ok(json["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
    }

    async fn call_anthropic(&self, service: &AIService, prompt: &str) -> Result<String, String> {
        let payload = serde_json::json!({
            "model": service.model,
            "max_tokens": 1000,
            "messages": [{"role": "user", "content": prompt}]
        });

        let response = self.client
            .post(&format!("{}/messages", service.endpoint))
            .header("x-api-key", service.api_key.as_ref().unwrap_or(&"".to_string()))
            .header("anthropic-version", "2023-06-01")
            .header("Content-Type", "application/json")
            .json(&payload)
            .send()
            .await
            .map_err(|e| e.to_string())?;

        let json: Value = response.json().await.map_err(|e| e.to_string())?;
        
        Ok(json["content"][0]["text"].as_str().unwrap_or("").to_string())
    }
} 