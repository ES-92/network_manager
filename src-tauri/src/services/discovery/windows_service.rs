use async_trait::async_trait;
use crate::models::service::{Service, ServiceStatus, ServiceType};
use super::traits::ServiceDiscovery;

#[cfg(target_os = "windows")]
use std::process::Command;

pub struct WindowsServiceDiscovery;

impl WindowsServiceDiscovery {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceDiscovery for WindowsServiceDiscovery {
    #[cfg(target_os = "windows")]
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>> {
        // Use PowerShell to get service information
        let output = Command::new("powershell")
            .args(["-Command", "Get-Service | Select-Object Name,Status,DisplayName | ConvertTo-Json"])
            .output()?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let json: serde_json::Value = serde_json::from_str(&stdout)?;

        let services: Vec<Service> = if let Some(arr) = json.as_array() {
            arr.iter()
                .filter_map(|item| {
                    let name = item["Name"].as_str()?.to_string();
                    let display_name = item["DisplayName"].as_str().map(String::from);
                    let status = match item["Status"].as_i64() {
                        Some(4) => ServiceStatus::Running,  // Running
                        Some(1) => ServiceStatus::Stopped,  // Stopped
                        _ => ServiceStatus::Unknown,
                    };

                    Some(Service {
                        id: name.clone(),
                        name: name.clone(),
                        status,
                        service_type: ServiceType::WindowsService,
                        ports: Vec::new(),
                        pid: None,
                        path: None,
                        description: display_name,
                        auto_start: false,
                        cpu_usage: None,
                        memory_bytes: None,
                        memory_percent: None,
                    })
                })
                .collect()
        } else {
            vec![]
        };

        Ok(services)
    }

    #[cfg(not(target_os = "windows"))]
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>> {
        Ok(vec![])
    }

    async fn get_service(&self, id: &str) -> Result<Option<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let services = self.discover().await?;
        Ok(services.into_iter().find(|s| s.id == id))
    }

    fn is_available(&self) -> bool {
        cfg!(target_os = "windows")
    }

    fn provider_name(&self) -> &'static str {
        "Windows Services"
    }
}
