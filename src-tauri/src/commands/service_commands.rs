use crate::models::service::Service;
use crate::services::ServiceManager;
use crate::services::control::{docker_control::DockerControl, process_control::ProcessControl};

#[cfg(target_os = "macos")]
use crate::services::control::launchd_control::LaunchdControl;

#[cfg(target_os = "linux")]
use crate::services::control::systemd_control::SystemdControl;

#[cfg(target_os = "windows")]
use crate::services::control::windows_control::WindowsControl;

use crate::services::control::traits::ServiceControl;
use std::sync::OnceLock;
use tokio::sync::Mutex;

// Global service manager instance
static SERVICE_MANAGER: OnceLock<Mutex<ServiceManager>> = OnceLock::new();

fn get_manager() -> &'static Mutex<ServiceManager> {
    SERVICE_MANAGER.get_or_init(|| Mutex::new(ServiceManager::new()))
}

#[tauri::command]
pub async fn discover_services() -> Result<Vec<Service>, String> {
    let manager = get_manager().lock().await;
    Ok(manager.discover_all().await)
}

#[tauri::command]
pub async fn get_service_details(service_id: String) -> Result<Option<Service>, String> {
    let manager = get_manager().lock().await;
    Ok(manager.get_service(&service_id).await)
}

#[tauri::command]
pub async fn start_service(service_id: String) -> Result<(), String> {
    let manager = get_manager().lock().await;

    if let Some(service) = manager.get_service(&service_id).await {
        let result = match service.service_type {
            crate::models::service::ServiceType::Docker => {
                DockerControl::new().start(&service_id).await
            }
            #[cfg(target_os = "macos")]
            crate::models::service::ServiceType::Launchd => {
                LaunchdControl::new().start(&service_id).await
            }
            #[cfg(target_os = "linux")]
            crate::models::service::ServiceType::Systemd => {
                SystemdControl::new().start(&service_id).await
            }
            #[cfg(target_os = "windows")]
            crate::models::service::ServiceType::WindowsService => {
                WindowsControl::new().start(&service_id).await
            }
            _ => Err("Cannot start this type of service".into()),
        };

        result.map_err(|e| e.to_string())
    } else {
        Err(format!("Service {} not found", service_id))
    }
}

#[tauri::command]
pub async fn stop_service(service_id: String) -> Result<(), String> {
    let manager = get_manager().lock().await;

    if let Some(service) = manager.get_service(&service_id).await {
        let result = match service.service_type {
            crate::models::service::ServiceType::Docker => {
                DockerControl::new().stop(&service_id).await
            }
            #[cfg(target_os = "macos")]
            crate::models::service::ServiceType::Launchd => {
                LaunchdControl::new().stop(&service_id).await
            }
            #[cfg(target_os = "linux")]
            crate::models::service::ServiceType::Systemd => {
                SystemdControl::new().stop(&service_id).await
            }
            #[cfg(target_os = "windows")]
            crate::models::service::ServiceType::WindowsService => {
                WindowsControl::new().stop(&service_id).await
            }
            crate::models::service::ServiceType::Process => {
                ProcessControl::new().stop(&service_id).await
            }
            #[allow(unreachable_patterns)]
            _ => Err("Cannot stop this type of service".into()),
        };

        result.map_err(|e| e.to_string())
    } else {
        Err(format!("Service {} not found", service_id))
    }
}

#[tauri::command]
pub async fn restart_service(service_id: String) -> Result<(), String> {
    let manager = get_manager().lock().await;

    if let Some(service) = manager.get_service(&service_id).await {
        let result = match service.service_type {
            crate::models::service::ServiceType::Docker => {
                DockerControl::new().restart(&service_id).await
            }
            #[cfg(target_os = "macos")]
            crate::models::service::ServiceType::Launchd => {
                LaunchdControl::new().restart(&service_id).await
            }
            #[cfg(target_os = "linux")]
            crate::models::service::ServiceType::Systemd => {
                SystemdControl::new().restart(&service_id).await
            }
            #[cfg(target_os = "windows")]
            crate::models::service::ServiceType::WindowsService => {
                WindowsControl::new().restart(&service_id).await
            }
            _ => Err("Cannot restart this type of service".into()),
        };

        result.map_err(|e| e.to_string())
    } else {
        Err(format!("Service {} not found", service_id))
    }
}

#[tauri::command]
pub async fn kill_process(pid: u32) -> Result<(), String> {
    let control = ProcessControl::new();
    control.kill(&pid.to_string()).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn enable_service_autostart(service_id: String, service_type: String) -> Result<(), String> {
    let result = match service_type.as_str() {
        #[cfg(target_os = "macos")]
        "launchd" => {
            LaunchdControl::new().enable_autostart(&service_id).await
        }
        #[cfg(target_os = "linux")]
        "systemd" => {
            SystemdControl::new().enable_autostart(&service_id).await
        }
        #[cfg(target_os = "windows")]
        "windows_service" => {
            WindowsControl::new().enable_autostart(&service_id).await
        }
        "docker" => {
            DockerControl::new().enable_autostart(&service_id).await
        }
        _ => Err("Autostart wird für diesen Service-Typ nicht unterstützt".into()),
    };

    result.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn disable_service_autostart(service_id: String, service_type: String) -> Result<(), String> {
    let result = match service_type.as_str() {
        #[cfg(target_os = "macos")]
        "launchd" => {
            LaunchdControl::new().disable_autostart(&service_id).await
        }
        #[cfg(target_os = "linux")]
        "systemd" => {
            SystemdControl::new().disable_autostart(&service_id).await
        }
        #[cfg(target_os = "windows")]
        "windows_service" => {
            WindowsControl::new().disable_autostart(&service_id).await
        }
        "docker" => {
            DockerControl::new().disable_autostart(&service_id).await
        }
        _ => Err("Autostart wird für diesen Service-Typ nicht unterstützt".into()),
    };

    result.map_err(|e| e.to_string())
}
