use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Workspace {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Tab {
    pub id: Uuid,
    pub workspace_id: Uuid,
    pub title: String,
    pub url: String,
    pub favicon_url: Option<String>,
    pub is_active: bool,
    pub position: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Bookmark {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub tags: Vec<String>,
    pub folder: Option<String>,
    pub ai_summary: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct BrowsingSession {
    pub id: Uuid,
    pub user_id: Uuid,
    pub workspace_id: Uuid,
    pub session_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AIConversation {
    pub id: Uuid,
    pub user_id: Uuid,
    pub title: Option<String>,
    pub messages: serde_json::Value,
    pub model: Option<String>,
    pub provider: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ContentAnalysis {
    pub id: Uuid,
    pub user_id: Uuid,
    pub url: String,
    pub title: Option<String>,
    pub content_hash: Option<String>,
    pub analysis_data: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct UserData {
    pub id: Uuid,
    pub key: String,
    pub value: serde_json::Value,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Helper structs for creating records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUser {
    pub username: String,
    pub email: String,
    pub password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWorkspace {
    pub user_id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTab {
    pub workspace_id: Uuid,
    pub title: String,
    pub url: String,
    pub favicon_url: Option<String>,
    pub position: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookmark {
    pub user_id: Uuid,
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
    pub folder: Option<String>,
    pub ai_summary: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserData {
    pub key: String,
    pub value: serde_json::Value,
    pub category: Option<String>,
}

impl CreateUser {
    pub fn new(username: String, email: String, password_hash: String) -> Self {
        Self {
            username,
            email,
            password_hash,
        }
    }
}

impl CreateWorkspace {
    pub fn new(user_id: Uuid, name: String) -> Self {
        Self {
            user_id,
            name,
            description: None,
            color: Some("#6366f1".to_string()),
        }
    }
}

impl CreateTab {
    pub fn new(workspace_id: Uuid, title: String, url: String) -> Self {
        Self {
            workspace_id,
            title,
            url,
            favicon_url: None,
            position: None,
        }
    }
}

impl CreateBookmark {
    pub fn new(user_id: Uuid, title: String, url: String) -> Self {
        Self {
            user_id,
            title,
            url,
            description: None,
            tags: None,
            folder: None,
            ai_summary: None,
        }
    }
} 