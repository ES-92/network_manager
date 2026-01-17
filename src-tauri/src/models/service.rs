use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Service {
    pub id: String,
    pub name: String,
    pub status: ServiceStatus,
    pub service_type: ServiceType,
    pub ports: Vec<u16>,
    pub pid: Option<u32>,
    pub path: Option<String>,
    pub description: Option<String>,
    pub auto_start: bool,
    /// CPU usage as percentage (0.0 - 100.0)
    pub cpu_usage: Option<f32>,
    /// Memory usage in bytes
    pub memory_bytes: Option<u64>,
    /// Memory usage as percentage of total system memory
    pub memory_percent: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceStatus {
    Running,
    Stopped,
    Error,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ServiceType {
    Docker,
    Systemd,
    Launchd,
    WindowsService,
    Process,
}
