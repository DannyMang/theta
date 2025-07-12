use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIProvider {
    pub name: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub models: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIModel {
    pub name: String,
    pub provider: String,
    pub max_tokens: u32,
    pub temperature_range: (f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIContext {
    pub conversation_id: String,
    pub messages: Vec<AIMessage>,
    pub model: String,
    pub provider: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIMessage {
    pub role: String,
    pub content: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AIProvider {
    pub fn new(name: String, endpoint: String, api_key: Option<String>) -> Self {
        Self {
            name,
            endpoint,
            api_key,
            models: vec![],
        }
    }

    pub fn add_model(&mut self, model: String) {
        self.models.push(model);
    }
}

impl AIContext {
    pub fn new(conversation_id: String, model: String, provider: String) -> Self {
        Self {
            conversation_id,
            messages: vec![],
            model,
            provider,
        }
    }

    pub fn add_message(&mut self, role: String, content: String) {
        self.messages.push(AIMessage {
            role,
            content,
            timestamp: chrono::Utc::now(),
        });
    }
} 