use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    pub theme: Theme,
    pub refresh_interval_ms: u32,
    pub ollama: OllamaConfig,
    pub security: SecurityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Theme {
    pub mode: ThemeMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "lowercase")]
pub enum ThemeMode {
    #[default]
    System,
    Light,
    Dark,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OllamaConfig {
    pub enabled: bool,
    pub endpoint: String,
    pub model: String,
    pub timeout_seconds: u32,
}

impl Default for OllamaConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            endpoint: "http://localhost:11434".to_string(),
            model: "mistral:7b-instruct".to_string(),
            timeout_seconds: 30,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub audit_logging: bool,
    pub require_confirmation_for_kill: bool,
    pub privilege_cache_ttl_minutes: u32,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            audit_logging: true,
            require_confirmation_for_kill: true,
            privilege_cache_ttl_minutes: 15,
        }
    }
}
