use crate::models::{N8nWorkflow, IntegrationTrigger};
use crate::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tauri::command]
pub async fn setup_n8n_workflow(
    workflow_name: String,
    description: Option<String>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<N8nWorkflow, String> {
    let app_state = state.read().await;
    
    if let Some(integration) = app_state.integrations.get("n8n") {
        match create_n8n_workflow(&workflow_name, description, integration, &app_state.http_client).await {
            Ok(workflow) => Ok(workflow),
            Err(e) => Err(format!("Failed to create n8n workflow: {}", e)),
        }
    } else {
        Err("n8n integration not configured".to_string())
    }
}

#[tauri::command]
pub async fn trigger_integration(
    trigger: IntegrationTrigger,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<serde_json::Value, String> {
    let app_state = state.read().await;
    
    if let Some(integration) = app_state.integrations.get("n8n") {
        match trigger_n8n_workflow(&trigger, integration, &app_state.http_client).await {
            Ok(response) => Ok(response),
            Err(e) => Err(format!("Failed to trigger integration: {}", e)),
        }
    } else {
        Err("n8n integration not configured".to_string())
    }
}

async fn create_n8n_workflow(
    name: &str,
    description: Option<String>,
    integration: &crate::state::Integration,
    client: &reqwest::Client,
) -> Result<N8nWorkflow, Box<dyn std::error::Error>> {
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
                    "path": format!("/webhook/{}", name.to_lowercase().replace(" ", "-")),
                    "options": {}
                },
                "name": "Webhook",
                "type": "n8n-nodes-base.webhook",
                "typeVersion": 1,
                "position": [460, 300],
                "webhookId": uuid::Uuid::new_v4().to_string()
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
        },
        "settings": {
            "saveDataErrorExecution": "all",
            "saveDataSuccessExecution": "all",
            "saveManualExecutions": true,
            "callerPolicy": "workflowsFromSameOwner"
        }
    });
    
    let url = format!("{}/workflows", integration.endpoint);
    let mut request = client.post(&url).json(&workflow_data);
    
    if let Some(token) = &integration.auth_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await?;
    let response_json: serde_json::Value = response.json().await?;
    
    let workflow = N8nWorkflow {
        id: response_json["id"].as_str().unwrap_or("").to_string(),
        name: name.to_string(),
        description,
        webhook_url: Some(format!("{}/webhook/{}", integration.endpoint, name.to_lowercase().replace(" ", "-"))),
        is_active: true,
        triggers: vec!["webhook".to_string()],
        nodes: response_json["nodes"].clone(),
    };
    
    Ok(workflow)
}

async fn trigger_n8n_workflow(
    trigger: &IntegrationTrigger,
    integration: &crate::state::Integration,
    client: &reqwest::Client,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let url = format!("{}/workflows/{}/execute", integration.endpoint, trigger.workflow_id);
    let mut request = client.post(&url).json(&trigger.data);
    
    if let Some(token) = &integration.auth_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await?;
    let response_json: serde_json::Value = response.json().await?;
    
    Ok(response_json)
}

#[tauri::command]
pub async fn get_n8n_workflows(
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<Vec<N8nWorkflow>, String> {
    let app_state = state.read().await;
    
    if let Some(integration) = app_state.integrations.get("n8n") {
        match fetch_n8n_workflows(integration, &app_state.http_client).await {
            Ok(workflows) => Ok(workflows),
            Err(e) => Err(format!("Failed to fetch workflows: {}", e)),
        }
    } else {
        Err("n8n integration not configured".to_string())
    }
}

async fn fetch_n8n_workflows(
    integration: &crate::state::Integration,
    client: &reqwest::Client,
) -> Result<Vec<N8nWorkflow>, Box<dyn std::error::Error>> {
    let url = format!("{}/workflows", integration.endpoint);
    let mut request = client.get(&url);
    
    if let Some(token) = &integration.auth_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await?;
    let response_json: serde_json::Value = response.json().await?;
    
    let mut workflows = Vec::new();
    
    if let Some(data) = response_json["data"].as_array() {
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

#[tauri::command]
pub async fn test_n8n_connection(
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<bool, String> {
    let app_state = state.read().await;
    
    if let Some(integration) = app_state.integrations.get("n8n") {
        match test_n8n_api(integration, &app_state.http_client).await {
            Ok(status) => Ok(status),
            Err(e) => Err(format!("Connection test failed: {}", e)),
        }
    } else {
        Err("n8n integration not configured".to_string())
    }
}

async fn test_n8n_api(
    integration: &crate::state::Integration,
    client: &reqwest::Client,
) -> Result<bool, Box<dyn std::error::Error>> {
    let url = format!("{}/workflows", integration.endpoint);
    let mut request = client.get(&url);
    
    if let Some(token) = &integration.auth_token {
        request = request.header("Authorization", format!("Bearer {}", token));
    }
    
    let response = request.send().await?;
    Ok(response.status().is_success())
} 