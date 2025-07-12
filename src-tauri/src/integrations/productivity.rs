use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductivityIntegration {
    pub provider: String,
    pub api_key: Option<String>,
    #[serde(skip)]
    pub client: Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub completed: bool,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTask {
    pub title: String,
    pub description: Option<String>,
    pub due_date: Option<DateTime<Utc>>,
    pub priority: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateNote {
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
}

impl ProductivityIntegration {
    pub fn new(provider: String, api_key: Option<String>) -> Self {
        Self {
            provider,
            api_key,
            client: Client::new(),
        }
    }

    // Task Management
    pub async fn get_tasks(&self) -> Result<Vec<Task>, String> {
        match self.provider.as_str() {
            "notion" => self.get_notion_tasks().await,
            "todoist" => self.get_todoist_tasks().await,
            "trello" => self.get_trello_tasks().await,
            _ => Err("Unsupported productivity provider".to_string()),
        }
    }

    pub async fn create_task(&self, task: CreateTask) -> Result<Task, String> {
        match self.provider.as_str() {
            "notion" => self.create_notion_task(task).await,
            "todoist" => self.create_todoist_task(task).await,
            "trello" => self.create_trello_task(task).await,
            _ => Err("Unsupported productivity provider".to_string()),
        }
    }

    // Note Management
    pub async fn get_notes(&self) -> Result<Vec<Note>, String> {
        match self.provider.as_str() {
            "notion" => self.get_notion_notes().await,
            "obsidian" => self.get_obsidian_notes().await,
            _ => Err("Unsupported note provider".to_string()),
        }
    }

    pub async fn create_note(&self, note: CreateNote) -> Result<Note, String> {
        match self.provider.as_str() {
            "notion" => self.create_notion_note(note).await,
            "obsidian" => self.create_obsidian_note(note).await,
            _ => Err("Unsupported note provider".to_string()),
        }
    }

    // Notion Integration
    async fn get_notion_tasks(&self) -> Result<Vec<Task>, String> {
        let api_key = self.api_key.as_ref().ok_or("Notion API key not set")?;
        let database_id = std::env::var("NOTION_TASKS_DATABASE_ID").map_err(|_| "Notion tasks database ID not set")?;
        
        let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Notion tasks: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut tasks = Vec::new();
        
        if let Some(results) = json["results"].as_array() {
            for result in results {
                let task = Task {
                    id: result["id"].as_str().unwrap_or("").to_string(),
                    title: result["properties"]["Title"]["title"][0]["text"]["content"].as_str().unwrap_or("").to_string(),
                    description: result["properties"]["Description"]["rich_text"][0]["text"]["content"].as_str().map(|s| s.to_string()),
                    completed: result["properties"]["Completed"]["checkbox"].as_bool().unwrap_or(false),
                    due_date: result["properties"]["Due Date"]["date"]["start"].as_str()
                        .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                        .map(|dt| dt.with_timezone(&Utc)),
                    priority: result["properties"]["Priority"]["select"]["name"].as_str().unwrap_or("Medium").to_string(),
                    tags: vec![],
                };
                tasks.push(task);
            }
        }
        
        Ok(tasks)
    }

    async fn create_notion_task(&self, task: CreateTask) -> Result<Task, String> {
        let api_key = self.api_key.as_ref().ok_or("Notion API key not set")?;
        let database_id = std::env::var("NOTION_TASKS_DATABASE_ID").map_err(|_| "Notion tasks database ID not set")?;
        
        let url = "https://api.notion.com/v1/pages";
        
        let task_data = serde_json::json!({
            "parent": {
                "database_id": database_id
            },
            "properties": {
                "Title": {
                    "title": [
                        {
                            "text": {
                                "content": task.title
                            }
                        }
                    ]
                },
                "Description": {
                    "rich_text": [
                        {
                            "text": {
                                                                 "content": task.description.as_deref().unwrap_or_default()
                            }
                        }
                    ]
                },
                "Priority": {
                    "select": {
                        "name": task.priority
                    }
                },
                "Due Date": {
                    "date": task.due_date.map(|dt| serde_json::json!({
                        "start": dt.to_rfc3339()
                    }))
                }
            }
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .json(&task_data)
            .send()
            .await
            .map_err(|e| format!("Failed to create Notion task: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let created_task = Task {
            id: json["id"].as_str().unwrap_or("").to_string(),
            title: task.title,
            description: task.description,
            completed: false,
            due_date: task.due_date,
            priority: task.priority,
            tags: task.tags,
        };
        
        Ok(created_task)
    }

    async fn get_notion_notes(&self) -> Result<Vec<Note>, String> {
        let api_key = self.api_key.as_ref().ok_or("Notion API key not set")?;
        let database_id = std::env::var("NOTION_NOTES_DATABASE_ID").map_err(|_| "Notion notes database ID not set")?;
        
        let url = format!("https://api.notion.com/v1/databases/{}/query", database_id);
        
        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .json(&serde_json::json!({}))
            .send()
            .await
            .map_err(|e| format!("Failed to fetch Notion notes: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let mut notes = Vec::new();
        
        if let Some(results) = json["results"].as_array() {
            for result in results {
                let note = Note {
                    id: result["id"].as_str().unwrap_or("").to_string(),
                    title: result["properties"]["Title"]["title"][0]["text"]["content"].as_str().unwrap_or("").to_string(),
                    content: result["properties"]["Content"]["rich_text"][0]["text"]["content"].as_str().unwrap_or("").to_string(),
                    created_at: DateTime::parse_from_rfc3339(result["created_time"].as_str().unwrap_or(""))
                        .map_err(|e| format!("Invalid created time: {}", e))?.with_timezone(&Utc),
                    updated_at: DateTime::parse_from_rfc3339(result["last_edited_time"].as_str().unwrap_or(""))
                        .map_err(|e| format!("Invalid updated time: {}", e))?.with_timezone(&Utc),
                    tags: vec![],
                };
                notes.push(note);
            }
        }
        
        Ok(notes)
    }

    async fn create_notion_note(&self, note: CreateNote) -> Result<Note, String> {
        let api_key = self.api_key.as_ref().ok_or("Notion API key not set")?;
        let database_id = std::env::var("NOTION_NOTES_DATABASE_ID").map_err(|_| "Notion notes database ID not set")?;
        
        let url = "https://api.notion.com/v1/pages";
        
        let note_data = serde_json::json!({
            "parent": {
                "database_id": database_id
            },
            "properties": {
                "Title": {
                    "title": [
                        {
                            "text": {
                                "content": note.title
                            }
                        }
                    ]
                },
                "Content": {
                    "rich_text": [
                        {
                            "text": {
                                "content": note.content
                            }
                        }
                    ]
                }
            }
        });
        
        let response = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Notion-Version", "2022-06-28")
            .json(&note_data)
            .send()
            .await
            .map_err(|e| format!("Failed to create Notion note: {}", e))?;
        
        let json: Value = response.json().await
            .map_err(|e| format!("Failed to parse response: {}", e))?;
        
        let created_note = Note {
            id: json["id"].as_str().unwrap_or("").to_string(),
            title: note.title,
            content: note.content,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            tags: note.tags,
        };
        
        Ok(created_note)
    }

    // Placeholder implementations for other providers
    async fn get_todoist_tasks(&self) -> Result<Vec<Task>, String> {
        // Implement Todoist API integration
        Ok(vec![])
    }

    async fn create_todoist_task(&self, _task: CreateTask) -> Result<Task, String> {
        // Implement Todoist API integration
        Err("Todoist integration not implemented".to_string())
    }

    async fn get_trello_tasks(&self) -> Result<Vec<Task>, String> {
        // Implement Trello API integration
        Ok(vec![])
    }

    async fn create_trello_task(&self, _task: CreateTask) -> Result<Task, String> {
        // Implement Trello API integration
        Err("Trello integration not implemented".to_string())
    }

    async fn get_obsidian_notes(&self) -> Result<Vec<Note>, String> {
        // Implement Obsidian local file integration
        Ok(vec![])
    }

    async fn create_obsidian_note(&self, _note: CreateNote) -> Result<Note, String> {
        // Implement Obsidian local file integration
        Err("Obsidian integration not implemented".to_string())
    }
}

impl Default for ProductivityIntegration {
    fn default() -> Self {
        Self::new(
            std::env::var("PRODUCTIVITY_PROVIDER").unwrap_or_else(|_| "notion".to_string()),
            std::env::var("PRODUCTIVITY_API_KEY").ok(),
        )
    }
} 