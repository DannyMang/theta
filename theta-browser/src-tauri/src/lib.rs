use tauri::Manager;
use std::sync::Arc;
use tokio::sync::RwLock;

mod ai;
mod browser;
mod database;
mod integrations;
mod models;
mod commands;
mod state;

use state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    dotenv::dotenv().ok();
    env_logger::init();

    let app_state = Arc::new(RwLock::new(AppState::new()));

    tauri::Builder::default()
        .manage(app_state)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .invoke_handler(tauri::generate_handler![
            commands::ai::analyze_content,
            commands::ai::generate_summary,
            commands::ai::chat_with_ai,
            commands::browser::navigate_to_url,
            commands::browser::search_web,
            commands::browser::bookmark_page,
            commands::browser::get_page_content,
            commands::database::save_user_data,
            commands::database::get_user_data,
            commands::integrations::setup_n8n_workflow,
            commands::integrations::trigger_integration,
            commands::integrations::get_n8n_workflows,
            commands::integrations::test_n8n_connection
        ])
        .setup(|app| {
            let app_handle = app.handle().clone();
            tauri::async_runtime::spawn(async move {
                if let Err(e) = initialize_app_state(app_handle).await {
                    log::error!("Failed to initialize app state: {}", e);
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

async fn initialize_app_state(app_handle: tauri::AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Initializing Theta Browser...");
    
    let state = app_handle.state::<Arc<RwLock<AppState>>>();
    let mut app_state = state.write().await;
    
    app_state.initialize_database().await?;
    app_state.initialize_ai_services().await?;
    app_state.initialize_integrations().await?;
    
    log::info!("Theta Browser initialized successfully");
    Ok(())
}
