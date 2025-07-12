use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8nIntegration {
    pub endpoint: String,
    pub api_key: Option<String>,
    pub webhook_url: Option<String>,
    #[serde(skip)]
    pub client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8nWorkflow {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub webhook_url: Option<String>,
    pub is_active: bool,
    pub triggers: Vec<String>,
    pub nodes: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct N8nExecution {
    pub id: String,
    pub workflow_id: String,
    pub status: String,
    pub data: Value,
    pub started_at: String,
    pub finished_at: Option<String>,
}

impl N8nIntegration {
    pub fn new(endpoint: String, api_key: Option<String>) -> Self {
        Self {
            endpoint,
            api_key,
            webhook_url: None,
            client: Client::new(),
        }
    }

    pub async fn test_connection(&self) -> Result<bool, String> {
        let url = format!("{}/workflows", self.endpoint);
        
        let mut request = self.client.get(&url);
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        match request.send().await {
            Ok(response) => Ok(response.status().is_success()),
            Err(e) => Err(format!("Connection test failed: {}", e)),
        }
    }

    pub async fn get_workflows(&self) -> Result<Vec<N8nWorkflow>, String> {
        let url = format!("{}/workflows", self.endpoint);
        
        let mut request = self.client.get(&url);
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| format!("Failed to fetch workflows: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut workflows = Vec::new();
        
        if let Some(data) = json["data"].as_array() {
            for item in data {
                let workflow = N8nWorkflow {
                    id: item["id"].as_str().unwrap_or("").to_string(),
                    name: item["name"].as_str().unwrap_or("").to_string(),
                    description: item["description"].as_str().map(|s| s.to_string()),
                    webhook_url: None,
                    is_active: item["active"].as_bool().unwrap_or(false),
                    triggers: vec![],
                    nodes: item["nodes"].clone(),
                };
                workflows.push(workflow);
            }
        }
        
        Ok(workflows)
    }

    pub async fn create_workflow(&self, name: &str, description: Option<&str>) -> Result<N8nWorkflow, String> {
        let url = format!("{}/workflows", self.endpoint);
        
        let workflow_data = serde_json::json!({
            "name": name,
            "active": true,
            "nodes": [
                {
                    "parameters": {},
                    "name": "Start",
                    "type": "n8n-nodes-base.start",
                    "typeVersion": 1,
                    "position": [240, 300]
                },
                {
                    "parameters": {
                        "httpMethod": "POST",
                        "path": format!("/{}", name.to_lowercase().replace(" ", "-")),
                        "options": {}
                    },
                    "name": "Webhook",
                    "type": "n8n-nodes-base.webhook",
                    "typeVersion": 1,
                    "position": [460, 300],
                    "webhookId": Uuid::new_v4().to_string()
                }
            ],
            "connections": {
                "Start": {
                    "main": [
                        [
                            {
                                "node": "Webhook",
                                "type": "main",
                                "index": 0
                            }
                        ]
                    ]
                }
            }
        });
        
        let mut request = self.client.post(&url).json(&workflow_data);
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| format!("Failed to create workflow: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let workflow = N8nWorkflow {
            id: json["id"].as_str().unwrap_or("").to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            webhook_url: Some(format!("{}/webhook/{}", self.endpoint, name.to_lowercase().replace(" ", "-"))),
            is_active: true,
            triggers: vec!["webhook".to_string()],
            nodes: json["nodes"].clone(),
        };
        
        Ok(workflow)
    }

    pub async fn execute_workflow(&self, workflow_id: &str, data: Value) -> Result<N8nExecution, String> {
        let url = format!("{}/workflows/{}/execute", self.endpoint, workflow_id);
        
        let mut request = self.client.post(&url).json(&data);
        
        if let Some(api_key) = &self.api_key {
            request = request.header("Authorization", format!("Bearer {}", api_key));
        }
        
        let response = request.send().await
            .map_err(|e| format!("Failed to execute workflow: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let execution = N8nExecution {
            id: json["id"].as_str().unwrap_or("").to_string(),
            workflow_id: workflow_id.to_string(),
            status: json["status"].as_str().unwrap_or("unknown").to_string(),
            data: json["data"].clone(),
            started_at: json["startedAt"].as_str().unwrap_or("").to_string(),
            finished_at: json["finishedAt"].as_str().map(|s| s.to_string()),
        };
        
        Ok(execution)
    }

    pub async fn trigger_webhook(&self, webhook_url: &str, data: Value) -> Result<Value, String> {
        let response = self.client
            .post(webhook_url)
            .json(&data)
            .send()
            .await
            .map_err(|e| format!("Failed to trigger webhook: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        Ok(json)
    }
}

impl Default for N8nIntegration {
    fn default() -> Self {
        Self::new(
            std::env::var("N8N_ENDPOINT").unwrap_or_else(|_| "http://localhost:5678".to_string()),
            std::env::var("N8N_API_KEY").ok(),
        )
    }
} 