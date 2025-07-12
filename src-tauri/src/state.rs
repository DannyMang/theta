use std::collections::HashMap;
use sqlx::PgPool;
use redis::Client as RedisClient;
use reqwest::Client as HttpClient;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct AIService {
    pub service_type: String,
    pub endpoint: String,
    pub api_key: Option<String>,
    pub model: String,
}

#[derive(Clone)]
pub struct Integration {
    pub name: String,
    pub endpoint: String,
    pub auth_token: Option<String>,
    pub config: HashMap<String, String>,
}

pub struct AppState {
    pub database: Option<PgPool>,
    pub redis: Option<RedisClient>,
    pub http_client: HttpClient,
    pub ai_services: HashMap<String, AIService>,
    pub integrations: HashMap<String, Integration>,
    pub active_tabs: HashMap<Uuid, TabState>,
    pub user_preferences: HashMap<String, String>,
}

#[derive(Clone)]
pub struct TabState {
    pub id: Uuid,
    pub url: String,
    pub title: String,
    pub content: Option<String>,
    pub ai_context: Option<String>,
    pub last_updated: chrono::DateTime<chrono::Utc>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            database: None,
            redis: None,
            http_client: HttpClient::new(),
            ai_services: HashMap::new(),
            integrations: HashMap::new(),
            active_tabs: HashMap::new(),
            user_preferences: HashMap::new(),
        }
    }

    pub async fn initialize_database(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost/theta_browser".to_string());
        
        let pool = PgPool::connect(&database_url).await?;
        self.database = Some(pool);
        
        let redis_url = std::env::var("REDIS_URL")
            .unwrap_or_else(|_| "redis://localhost".to_string());
        
        let redis_client = RedisClient::open(redis_url)?;
        self.redis = Some(redis_client);
        
        Ok(())
    }

    pub async fn initialize_ai_services(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let openai_key = std::env::var("OPENAI_API_KEY").ok();
        let anthropic_key = std::env::var("ANTHROPIC_API_KEY").ok();
        
        if let Some(key) = openai_key {
            self.ai_services.insert("openai".to_string(), AIService {
                service_type: "openai".to_string(),
                endpoint: "https://api.openai.com/v1".to_string(),
                api_key: Some(key),
                model: "gpt-4".to_string(),
            });
        }
        
        if let Some(key) = anthropic_key {
            self.ai_services.insert("anthropic".to_string(), AIService {
                service_type: "anthropic".to_string(),
                endpoint: "https://api.anthropic.com/v1".to_string(),
                api_key: Some(key),
                model: "claude-3-sonnet-20240229".to_string(),
            });
        }
        
        Ok(())
    }

    pub async fn initialize_integrations(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let n8n_endpoint = std::env::var("N8N_ENDPOINT").ok();
        let n8n_token = std::env::var("N8N_API_KEY").ok();
        
        if let Some(endpoint) = n8n_endpoint {
            self.integrations.insert("n8n".to_string(), Integration {
                name: "n8n".to_string(),
                endpoint,
                auth_token: n8n_token,
                config: HashMap::new(),
            });
        }
        
        Ok(())
    }

    pub fn create_tab(&mut self, url: String, title: String) -> Uuid {
        let id = Uuid::new_v4();
        let tab = TabState {
            id,
            url,
            title,
            content: None,
            ai_context: None,
            last_updated: chrono::Utc::now(),
        };
        
        self.active_tabs.insert(id, tab);
        id
    }

    pub fn get_tab(&self, id: &Uuid) -> Option<&TabState> {
        self.active_tabs.get(id)
    }

    pub fn update_tab_content(&mut self, id: &Uuid, content: String) {
        if let Some(tab) = self.active_tabs.get_mut(id) {
            tab.content = Some(content);
            tab.last_updated = chrono::Utc::now();
        }
    }

    pub fn remove_tab(&mut self, id: &Uuid) {
        self.active_tabs.remove(id);
    }
} 