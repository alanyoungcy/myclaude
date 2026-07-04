use std::sync::Mutex;
use tauri::Manager;

mod config;
mod llm;
mod storage;
mod commands;
mod tavily;
mod skills;
mod llm_wrapper;
mod deep_research;

use config::AppConfig;
use storage::Database;
use skills::SkillLoader;

pub struct AppState {
    pub config: Mutex<AppConfig>,
    pub db: Mutex<Database>,
    pub skill_loader: SkillLoader,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            println!("=== MyClaude Starting ===");
            
            // Initialize configuration
            println!("Initializing configuration...");
            let config = AppConfig::load().unwrap_or_else(|e| {
                eprintln!("Failed to load config: {}, using defaults", e);
                AppConfig::default()
            });
            println!("Configuration initialized");
            
            // Initialize database
            println!("Initializing database...");
            let db = Database::new().expect("Failed to initialize database");
            println!("Database initialized");

            // Initialize skill loader
            println!("Initializing skill loader...");
            let app_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            let skills_dir = app_dir.join("skills");
            std::fs::create_dir_all(&skills_dir).ok();
            let skill_loader = SkillLoader::new(skills_dir);
            println!("Skill loader initialized");

            // Set up app state
            app.manage(AppState {
                config: Mutex::new(config),
                db: Mutex::new(db),
                skill_loader,
            });
            
            println!("=== MyClaude Ready ===");
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_config,
            commands::update_config,
            commands::get_models,
            commands::send_message,
            commands::get_conversations,
            commands::get_conversation,
            commands::create_conversation,
            commands::delete_conversation,
            commands::get_system_prompts,
            commands::save_system_prompt,
            commands::delete_system_prompt,
            commands::web_search,
            commands::get_skills,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
