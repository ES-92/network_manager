use crate::models::service::Service;
use crate::services::ServiceManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::Mutex;

/// Event types emitted by the service monitor
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type", content = "payload")]
pub enum ServiceEvent {
    /// Initial list of all services
    ServicesDiscovered(Vec<Service>),
    /// A service's status changed
    ServiceStatusChanged {
        service_id: String,
        old_status: String,
        new_status: String,
    },
    /// A new service was detected
    ServiceAdded(Service),
    /// A service was removed
    ServiceRemoved { service_id: String },
    /// Port usage changed for a service
    ServicePortsChanged {
        service_id: String,
        ports: Vec<u16>,
    },
}

/// Configuration for the service monitor
#[derive(Clone)]
pub struct MonitorConfig {
    /// Interval between service checks (default: 5 seconds)
    pub check_interval: Duration,
    /// Whether the monitor is enabled
    pub enabled: bool,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            check_interval: Duration::from_secs(5),
            enabled: true,
        }
    }
}

/// Service monitor that watches for changes and emits events
pub struct ServiceMonitor {
    manager: Arc<Mutex<ServiceManager>>,
    config: MonitorConfig,
    last_state: Arc<Mutex<HashMap<String, Service>>>,
}

impl ServiceMonitor {
    pub fn new(manager: Arc<Mutex<ServiceManager>>) -> Self {
        Self {
            manager,
            config: MonitorConfig::default(),
            last_state: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn with_config(mut self, config: MonitorConfig) -> Self {
        self.config = config;
        self
    }

    /// Start the monitoring loop
    pub async fn start(&self, app_handle: AppHandle) {
        if !self.config.enabled {
            return;
        }

        let manager = Arc::clone(&self.manager);
        let last_state = Arc::clone(&self.last_state);
        let interval = self.config.check_interval;

        tokio::spawn(async move {
            loop {
                // Discover current services
                let services = {
                    let mgr = manager.lock().await;
                    mgr.discover_all().await
                };

                // Build current state map
                let current_state: HashMap<String, Service> = services
                    .iter()
                    .map(|s| (s.id.clone(), s.clone()))
                    .collect();

                // Compare with last state
                let mut state = last_state.lock().await;

                if state.is_empty() {
                    // First run - emit all services
                    let _ = app_handle.emit("service-event", ServiceEvent::ServicesDiscovered(services.clone()));
                } else {
                    // Check for changes
                    for (id, service) in &current_state {
                        if let Some(old_service) = state.get(id) {
                            // Check if status changed
                            let old_status = format!("{:?}", old_service.status);
                            let new_status = format!("{:?}", service.status);
                            if old_status != new_status {
                                let _ = app_handle.emit(
                                    "service-event",
                                    ServiceEvent::ServiceStatusChanged {
                                        service_id: id.clone(),
                                        old_status,
                                        new_status,
                                    },
                                );
                            }

                            // Check if ports changed
                            if old_service.ports != service.ports {
                                let _ = app_handle.emit(
                                    "service-event",
                                    ServiceEvent::ServicePortsChanged {
                                        service_id: id.clone(),
                                        ports: service.ports.clone(),
                                    },
                                );
                            }
                        } else {
                            // New service detected
                            let _ = app_handle.emit(
                                "service-event",
                                ServiceEvent::ServiceAdded(service.clone()),
                            );
                        }
                    }

                    // Check for removed services
                    for id in state.keys() {
                        if !current_state.contains_key(id) {
                            let _ = app_handle.emit(
                                "service-event",
                                ServiceEvent::ServiceRemoved {
                                    service_id: id.clone(),
                                },
                            );
                        }
                    }
                }

                // Update last state
                *state = current_state;

                // Wait for next interval
                tokio::time::sleep(interval).await;
            }
        });
    }
}

/// Commands for controlling the monitor
#[tauri::command]
pub async fn set_monitor_interval(seconds: u64, state: tauri::State<'_, MonitorState>) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.check_interval = Duration::from_secs(seconds);
    Ok(())
}

#[tauri::command]
pub async fn enable_monitor(enabled: bool, state: tauri::State<'_, MonitorState>) -> Result<(), String> {
    let mut config = state.config.lock().await;
    config.enabled = enabled;
    Ok(())
}

/// State for the monitor that can be managed by Tauri
pub struct MonitorState {
    pub config: Mutex<MonitorConfig>,
}

impl Default for MonitorState {
    fn default() -> Self {
        Self {
            config: Mutex::new(MonitorConfig::default()),
        }
    }
}
