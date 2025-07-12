use super::models::{AIProvider, AIModel};
use std::collections::HashMap;

pub struct ProviderManager {
    providers: HashMap<String, AIProvider>,
}

impl ProviderManager {
    pub fn new() -> Self {
        let mut manager = Self {
            providers: HashMap::new(),
        };
        
        // Initialize default providers
        manager.init_default_providers();
        manager
    }

    fn init_default_providers(&mut self) {
        // OpenAI
        let mut openai = AIProvider::new(
            "openai".to_string(),
            "https://api.openai.com/v1".to_string(),
            std::env::var("OPENAI_API_KEY").ok(),
        );
        openai.add_model("gpt-4".to_string());
        openai.add_model("gpt-3.5-turbo".to_string());
        self.providers.insert("openai".to_string(), openai);

        // Anthropic
        let mut anthropic = AIProvider::new(
            "anthropic".to_string(),
            "https://api.anthropic.com/v1".to_string(),
            std::env::var("ANTHROPIC_API_KEY").ok(),
        );
        anthropic.add_model("claude-3-sonnet-20240229".to_string());
        anthropic.add_model("claude-3-haiku-20240307".to_string());
        self.providers.insert("anthropic".to_string(), anthropic);
    }

    pub fn get_provider(&self, name: &str) -> Option<&AIProvider> {
        self.providers.get(name)
    }

    pub fn add_provider(&mut self, name: String, provider: AIProvider) {
        self.providers.insert(name, provider);
    }

    pub fn list_providers(&self) -> Vec<&String> {
        self.providers.keys().collect()
    }

    pub fn get_available_models(&self, provider_name: &str) -> Vec<&String> {
        if let Some(provider) = self.providers.get(provider_name) {
            provider.models.iter().collect()
        } else {
            vec![]
        }
    }
} 