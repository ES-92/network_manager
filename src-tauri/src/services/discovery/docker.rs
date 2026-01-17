use async_trait::async_trait;
use bollard::Docker;
#[allow(deprecated)]
use bollard::container::ListContainersOptions;
use crate::models::service::{Service, ServiceStatus, ServiceType};
use super::traits::ServiceDiscovery;

pub struct DockerDiscovery {
    docker: Option<Docker>,
}

impl DockerDiscovery {
    pub fn new() -> Self {
        let docker = Docker::connect_with_local_defaults().ok();
        Self { docker }
    }
}

#[async_trait]
impl ServiceDiscovery for DockerDiscovery {
    #[allow(deprecated)]
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let docker = match &self.docker {
            Some(d) => d,
            None => return Ok(vec![]),
        };

        let options = Some(ListContainersOptions::<String> {
            all: true,
            ..Default::default()
        });

        let containers = docker.list_containers(options).await?;

        let mut services = Vec::new();

        for container in containers {
            let container_id = container.id.clone().unwrap_or_default();
            let name = container.names
                .and_then(|names| names.first().cloned())
                .map(|n| n.trim_start_matches('/').to_string())
                .unwrap_or_else(|| "unknown".to_string());

            // Convert state enum to our status
            let status = match container.state {
                Some(state) => {
                    let state_str = format!("{:?}", state).to_lowercase();
                    if state_str.contains("running") {
                        ServiceStatus::Running
                    } else if state_str.contains("exited") || state_str.contains("dead") {
                        ServiceStatus::Stopped
                    } else {
                        ServiceStatus::Unknown
                    }
                }
                None => ServiceStatus::Unknown,
            };

            // Extract all port mappings
            let ports: Vec<u16> = container.ports
                .map(|ports| {
                    ports.iter()
                        .filter_map(|p| p.public_port)
                        .collect()
                })
                .unwrap_or_default();

            // Get restart policy from container inspection
            let auto_start = if !container_id.is_empty() {
                match docker.inspect_container(&container_id, None::<bollard::container::InspectContainerOptions>).await {
                    Ok(info) => {
                        info.host_config
                            .and_then(|hc| hc.restart_policy)
                            .and_then(|rp| rp.name)
                            .map(|name| {
                                let name_str = format!("{:?}", name).to_lowercase();
                                name_str.contains("always") || name_str.contains("unless")
                            })
                            .unwrap_or(false)
                    }
                    Err(_) => false,
                }
            } else {
                false
            };

            services.push(Service {
                id: container_id,
                name,
                status,
                service_type: ServiceType::Docker,
                ports,
                pid: None,
                path: container.image,
                description: container.status,
                auto_start,
                cpu_usage: None,
                memory_bytes: None,
                memory_percent: None,
            });
        }

        Ok(services)
    }

    async fn get_service(&self, id: &str) -> Result<Option<Service>, Box<dyn std::error::Error + Send + Sync>> {
        let services = self.discover().await?;
        Ok(services.into_iter().find(|s| s.id == id))
    }

    fn is_available(&self) -> bool {
        self.docker.is_some()
    }

    fn provider_name(&self) -> &'static str {
        "Docker"
    }
}
