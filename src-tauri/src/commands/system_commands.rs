use crate::services::system_stats::{SystemMonitor, SystemStats, GpuProvider};
use crate::services::security_scanner::{SecurityScanner, SecurityScanResult};
use crate::services::ServiceManager;
use std::sync::OnceLock;
use tokio::sync::Mutex;

static SYSTEM_MONITOR: OnceLock<Mutex<SystemMonitor>> = OnceLock::new();
static SECURITY_SCANNER: OnceLock<SecurityScanner> = OnceLock::new();
static SERVICE_MANAGER: OnceLock<Mutex<ServiceManager>> = OnceLock::new();

fn get_system_monitor() -> &'static Mutex<SystemMonitor> {
    SYSTEM_MONITOR.get_or_init(|| Mutex::new(SystemMonitor::new()))
}

fn get_security_scanner() -> &'static SecurityScanner {
    SECURITY_SCANNER.get_or_init(SecurityScanner::new)
}

fn get_service_manager() -> &'static Mutex<ServiceManager> {
    SERVICE_MANAGER.get_or_init(|| Mutex::new(ServiceManager::new()))
}

#[tauri::command]
pub async fn get_system_stats() -> Result<SystemStats, String> {
    let mut monitor = get_system_monitor().lock().await;
    Ok(monitor.get_stats())
}

#[tauri::command]
pub async fn set_gpu_provider(provider: String) -> Result<(), String> {
    let gpu_provider = match provider.to_lowercase().as_str() {
        "auto" => GpuProvider::Auto,
        "apple" => GpuProvider::Apple,
        "nvidia" => GpuProvider::Nvidia,
        "amd" => GpuProvider::Amd,
        "none" => GpuProvider::None,
        _ => return Err(format!("Unknown GPU provider: {}", provider)),
    };

    let mut monitor = get_system_monitor().lock().await;
    *monitor = SystemMonitor::new().with_gpu_provider(gpu_provider);
    Ok(())
}

#[tauri::command]
pub async fn scan_security() -> Result<SecurityScanResult, String> {
    let scanner = get_security_scanner();
    let manager = get_service_manager().lock().await;
    let services = manager.discover_all().await;
    Ok(scanner.scan(&services))
}

#[tauri::command]
pub async fn get_security_analysis(services_json: String) -> Result<String, String> {
    // Use LLM for security analysis if available
    let client = crate::llm::client::OllamaClient::new();

    if !client.is_available().await {
        return Err("Ollama ist nicht verfügbar. Starte Ollama für KI-Sicherheitsanalyse.".into());
    }

    let prompt = format!(
        r#"Analysiere diese Services auf Sicherheitsprobleme. Antworte auf Deutsch.
Prüfe auf:
- Unverschlüsselte Verbindungen
- Öffentlich erreichbare Datenbanken
- Fehlende Authentifizierung
- Bekannte Schwachstellen

Services:
{}

Gib eine kurze Zusammenfassung der wichtigsten Sicherheitsrisiken und Empfehlungen."#,
        services_json
    );

    client.generate(&prompt).await.map_err(|e| e.to_string())
}
