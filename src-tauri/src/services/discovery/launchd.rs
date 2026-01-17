use async_trait::async_trait;
use std::process::Command;
use crate::models::service::{Service, ServiceStatus, ServiceType};
use super::traits::ServiceDiscovery;

pub struct LaunchdDiscovery;

impl LaunchdDiscovery {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceDiscovery for LaunchdDiscovery {
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("launchctl")
            .arg("list")
            .output()?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let services: Vec<Service> = stdout
            .lines()
            .skip(1) // Skip header line
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let pid = parts[0].parse::<u32>().ok();
                    let status = if pid.is_some() {
                        ServiceStatus::Running
                    } else {
                        ServiceStatus::Stopped
                    };
                    let name = parts[2].to_string();

                    Some(Service {
                        id: name.clone(),
                        name: name.clone(),
                        status,
                        service_type: ServiceType::Launchd,
                        ports: Vec::new(),
                        pid,
                        path: None,
                        description: None,
                        auto_start: true,
                        cpu_usage: None,
                        memory_bytes: None,
                        memory_percent: None,
                    })
                } else {
                    None
                }
            })
            .collect();

        Ok(services)
    }

    async fn get_service(&self, id: &str) -> Result<Option<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let services = self.discover().await?;
        Ok(services.into_iter().find(|s| s.id == id))
    }

    fn is_available(&self) -> bool {
        cfg!(target_os = "macos")
    }

    fn provider_name(&self) -> &'static str {
        "launchd"
    }
}
