use crate::models::service::Service;
use crate::services::discovery::{docker::DockerDiscovery, process::ProcessDiscovery, ServiceDiscovery};

#[cfg(target_os = "macos")]
use crate::services::discovery::launchd::LaunchdDiscovery;

#[cfg(target_os = "linux")]
use crate::services::discovery::systemd::SystemdDiscovery;

#[cfg(target_os = "windows")]
use crate::services::discovery::windows_service::WindowsServiceDiscovery;

use crate::services::port::resolver::PortResolver;

/// Main service manager that orchestrates all discovery modules
pub struct ServiceManager {
    docker: DockerDiscovery,
    process: ProcessDiscovery,
    #[cfg(target_os = "macos")]
    launchd: LaunchdDiscovery,
    #[cfg(target_os = "linux")]
    systemd: SystemdDiscovery,
    #[cfg(target_os = "windows")]
    windows: WindowsServiceDiscovery,
    port_resolver: PortResolver,
}

impl ServiceManager {
    pub fn new() -> Self {
        Self {
            docker: DockerDiscovery::new(),
            process: ProcessDiscovery::new(),
            #[cfg(target_os = "macos")]
            launchd: LaunchdDiscovery::new(),
            #[cfg(target_os = "linux")]
            systemd: SystemdDiscovery::new(),
            #[cfg(target_os = "windows")]
            windows: WindowsServiceDiscovery::new(),
            port_resolver: PortResolver::new(),
        }
    }

    /// Discover all services from all available providers
    pub async fn discover_all(&self) -> Vec<Service> {
        let mut all_services = Vec::new();

        // Get port usage for enriching service data
        let port_usage = self.port_resolver.get_port_usage();

        // Docker containers
        if self.docker.is_available() {
            if let Ok(services) = self.docker.discover().await {
                all_services.extend(services);
            }
        }

        // Platform-specific services
        #[cfg(target_os = "macos")]
        {
            if let Ok(services) = self.launchd.discover().await {
                // Include launchd services (limit to 100 for performance)
                // Prioritize running services
                let mut sorted_services: Vec<Service> = services;
                sorted_services.sort_by(|a, b| {
                    let a_running = matches!(a.status, crate::models::service::ServiceStatus::Running);
                    let b_running = matches!(b.status, crate::models::service::ServiceStatus::Running);
                    b_running.cmp(&a_running)
                });
                let filtered: Vec<Service> = sorted_services
                    .into_iter()
                    .take(100)
                    .collect();
                all_services.extend(filtered);
            }
        }

        #[cfg(target_os = "linux")]
        {
            if let Ok(services) = self.systemd.discover().await {
                all_services.extend(services);
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Ok(services) = self.windows.discover().await {
                all_services.extend(services);
            }
        }

        // Enrich services with port information
        for service in &mut all_services {
            if let Some(pid) = service.pid {
                let service_ports: Vec<u16> = port_usage
                    .iter()
                    .filter(|p| p.pid == Some(pid))
                    .map(|p| p.port)
                    .collect();
                if !service_ports.is_empty() {
                    service.ports = service_ports;
                }
            }
        }

        // Add processes with ports that aren't already covered
        let service_pids: std::collections::HashSet<u32> = all_services
            .iter()
            .filter_map(|s| s.pid)
            .collect();

        // Group ports by PID for processes not already in the list
        let mut pid_ports: std::collections::HashMap<u32, Vec<u16>> = std::collections::HashMap::new();
        for port_info in &port_usage {
            if let Some(pid) = port_info.pid {
                if !service_pids.contains(&pid) {
                    pid_ports.entry(pid).or_default().push(port_info.port);
                }
            }
        }

        // Limit process services to max 50 (sorted by most ports)
        let mut pid_ports_vec: Vec<_> = pid_ports.into_iter().collect();
        pid_ports_vec.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

        for (pid, ports) in pid_ports_vec.into_iter().take(50) {
            let process_name = port_usage
                .iter()
                .find(|p| p.pid == Some(pid))
                .and_then(|p| p.process_name.clone())
                .unwrap_or_else(|| format!("Process {}", pid));

            let port_desc = if ports.len() == 1 {
                format!("Port {}", ports[0])
            } else {
                format!("Ports: {}", ports.iter().take(5).map(|p| p.to_string()).collect::<Vec<_>>().join(", "))
            };

            all_services.push(Service {
                id: format!("process-{}", pid),
                name: process_name,
                status: crate::models::service::ServiceStatus::Running,
                service_type: crate::models::service::ServiceType::Process,
                ports: ports.into_iter().take(10).collect(), // Limit ports per service
                pid: Some(pid),
                path: None,
                description: Some(port_desc),
                auto_start: false,
                cpu_usage: None,
                memory_bytes: None,
                memory_percent: None,
            });
        }

        // Deduplicate by ID first (keep first occurrence)
        let mut seen_ids = std::collections::HashSet::new();
        all_services.retain(|s| seen_ids.insert(s.id.clone()));

        // Sort: running services first, then by name
        all_services.sort_by(|a, b| {
            let a_running = matches!(a.status, crate::models::service::ServiceStatus::Running);
            let b_running = matches!(b.status, crate::models::service::ServiceStatus::Running);
            match b_running.cmp(&a_running) {
                std::cmp::Ordering::Equal => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                other => other,
            }
        });

        // Limit total services for performance (max 150)
        all_services.truncate(150);

        all_services
    }

    /// Get a specific service by ID
    pub async fn get_service(&self, id: &str) -> Option<Service> {
        let services = self.discover_all().await;
        services.into_iter().find(|s| s.id == id)
    }
}

impl Default for ServiceManager {
    fn default() -> Self {
        Self::new()
    }
}
