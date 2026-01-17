use async_trait::async_trait;
use sysinfo::{System, ProcessesToUpdate};
use crate::models::service::{Service, ServiceStatus, ServiceType};
use super::traits::ServiceDiscovery;

pub struct ProcessDiscovery {
    system: System,
    total_memory: u64,
}

impl ProcessDiscovery {
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_memory();
        let total_memory = system.total_memory();
        Self {
            system,
            total_memory,
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_processes(ProcessesToUpdate::All, true);
        self.system.refresh_memory();
    }
}

#[async_trait]
impl ServiceDiscovery for ProcessDiscovery {
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let total_mem = self.total_memory as f32;

        let services: Vec<Service> = self.system
            .processes()
            .iter()
            .map(|(pid, process)| {
                let memory_bytes = process.memory();
                let memory_percent = if total_mem > 0.0 {
                    Some((memory_bytes as f32 / total_mem) * 100.0)
                } else {
                    None
                };

                Service {
                    id: pid.as_u32().to_string(),
                    name: process.name().to_string_lossy().to_string(),
                    status: if process.status().to_string() == "Run" {
                        ServiceStatus::Running
                    } else {
                        ServiceStatus::Stopped
                    },
                    service_type: ServiceType::Process,
                    ports: Vec::new(), // Will be populated by port scanner
                    pid: Some(pid.as_u32()),
                    path: process.exe().map(|p| p.to_string_lossy().to_string()),
                    description: Some(process.cmd().iter().map(|s| s.to_string_lossy().to_string()).collect::<Vec<_>>().join(" ")),
                    auto_start: false,
                    cpu_usage: Some(process.cpu_usage()),
                    memory_bytes: Some(memory_bytes),
                    memory_percent,
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
        true
    }

    fn provider_name(&self) -> &'static str {
        "Process"
    }
}
