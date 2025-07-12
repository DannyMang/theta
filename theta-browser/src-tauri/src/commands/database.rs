use crate::models::UserData;
use crate::state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[tauri::command]
pub async fn save_user_data(
    key: String,
    value: serde_json::Value,
    category: Option<String>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<UserData, String> {
    let app_state = state.read().await;
    
    let user_data = UserData {
        id: Uuid::new_v4(),
        key: key.clone(),
        value: value.clone(),
        category: category.clone(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };
    
    if let Some(pool) = &app_state.database {
        match save_data_to_db(&user_data, pool).await {
            Ok(_) => Ok(user_data),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    } else {
        Err("Database not initialized".to_string())
    }
}

#[tauri::command]
pub async fn get_user_data(
    key: String,
    category: Option<String>,
    state: tauri::State<'_, Arc<RwLock<AppState>>>,
) -> Result<Option<UserData>, String> {
    let app_state = state.read().await;
    
    if let Some(pool) = &app_state.database {
        match get_data_from_db(&key, category.as_deref(), pool).await {
            Ok(data) => Ok(data),
            Err(e) => Err(format!("Database error: {}", e)),
        }
    } else {
        Err("Database not initialized".to_string())
    }
}

async fn save_data_to_db(
    user_data: &UserData,
    pool: &sqlx::PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        "INSERT INTO user_data (id, key, value, category, created_at, updated_at) 
         VALUES ($1, $2, $3, $4, $5, $6)
         ON CONFLICT (key, category) 
         DO UPDATE SET value = EXCLUDED.value, updated_at = EXCLUDED.updated_at",
        user_data.id,
        user_data.key,
        user_data.value,
        user_data.category,
        user_data.created_at,
        user_data.updated_at
    )
    .execute(pool)
    .await?;
    
    Ok(())
}

async fn get_data_from_db(
    key: &str,
    category: Option<&str>,
    pool: &sqlx::PgPool,
) -> Result<Option<UserData>, sqlx::Error> {
    let row = sqlx::query_as!(
        UserData,
        "SELECT id, key, value, category, created_at, updated_at
         FROM user_data 
         WHERE key = $1 AND ($2::text IS NULL OR category = $2)
         ORDER BY updated_at DESC
         LIMIT 1",
        key,
        category
    )
    .fetch_optional(pool)
    .await?;
    
    Ok(row)
} 