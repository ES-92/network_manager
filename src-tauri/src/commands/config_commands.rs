use crate::models::config::Config;
use std::sync::OnceLock;
use tokio::sync::RwLock;

// Global config instance
static CONFIG: OnceLock<RwLock<Config>> = OnceLock::new();

fn get_config_store() -> &'static RwLock<Config> {
    CONFIG.get_or_init(|| RwLock::new(Config::default()))
}

#[tauri::command]
pub async fn get_config() -> Result<Config, String> {
    let config = get_config_store().read().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn update_config(config: Config) -> Result<(), String> {
    let mut current = get_config_store().write().await;
    *current = config;
    Ok(())
}
