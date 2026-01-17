use async_trait::async_trait;
use std::process::Command;
use crate::models::service::{Service, ServiceStatus, ServiceType};
use super::traits::ServiceDiscovery;

pub struct SystemdDiscovery;

impl SystemdDiscovery {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceDiscovery for SystemdDiscovery {
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("systemctl")
            .args(["list-units", "--type=service", "--all", "--no-pager", "--plain"])
            .output()?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let services: Vec<Service> = stdout
            .lines()
            .filter(|line| line.contains(".service"))
            .filter_map(|line| {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 4 {
                    let name = parts[0].trim_end_matches(".service").to_string();
                    let status = match parts[3] {
                        "running" => ServiceStatus::Running,
                        "exited" | "dead" | "inactive" => ServiceStatus::Stopped,
                        "failed" => ServiceStatus::Error,
                        _ => ServiceStatus::Unknown,
                    };

                    Some(Service {
                        id: parts[0].to_string(),
                        name,
                        status,
                        service_type: ServiceType::Systemd,
                        ports: Vec::new(),
                        pid: None,
                        path: None,
                        description: parts.get(4..).map(|p| p.join(" ")),
                        auto_start: parts.get(1).map(|s| *s == "enabled").unwrap_or(false),
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
        cfg!(target_os = "linux") && Command::new("systemctl").arg("--version").output().is_ok()
    }

    fn provider_name(&self) -> &'static str {
        "systemd"
    }
}
