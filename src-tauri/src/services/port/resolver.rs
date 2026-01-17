use std::process::Command;
use crate::models::port::{PortInfo, Protocol, PortStatus};

pub struct PortResolver;

impl PortResolver {
    pub fn new() -> Self {
        Self
    }

    /// Get all ports currently in use with their associated processes
    #[cfg(target_os = "macos")]
    pub fn get_port_usage(&self) -> Vec<PortInfo> {
        let output = Command::new("lsof")
            .args(["-i", "-P", "-n"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.parse_lsof_output(&stdout)
            }
            _ => vec![],
        }
    }

    #[cfg(target_os = "linux")]
    pub fn get_port_usage(&self) -> Vec<PortInfo> {
        let output = Command::new("ss")
            .args(["-tulnp"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.parse_ss_output(&stdout)
            }
            _ => {
                // Fallback to netstat
                let output = Command::new("netstat")
                    .args(["-tulpn"])
                    .output();
                match output {
                    Ok(output) if output.status.success() => {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        self.parse_netstat_output(&stdout)
                    }
                    _ => vec![],
                }
            }
        }
    }

    #[cfg(target_os = "windows")]
    pub fn get_port_usage(&self) -> Vec<PortInfo> {
        let output = Command::new("netstat")
            .args(["-ano"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                self.parse_netstat_windows_output(&stdout)
            }
            _ => vec![],
        }
    }

    #[cfg(target_os = "macos")]
    fn parse_lsof_output(&self, output: &str) -> Vec<PortInfo> {
        let mut ports = vec![];

        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 9 {
                let process_name = parts[0].to_string();
                let pid = parts[1].parse::<u32>().ok();

                // Parse the name field (e.g., "TCP *:8080 (LISTEN)")
                if let Some(name_part) = parts.get(8) {
                    if let Some(port_str) = name_part.split(':').last() {
                        if let Ok(port) = port_str.trim_end_matches(|c| c == ')' || c == '(').parse::<u16>() {
                            let protocol = if line.contains("TCP") {
                                Protocol::Tcp
                            } else {
                                Protocol::Udp
                            };

                            ports.push(PortInfo {
                                port,
                                protocol,
                                status: PortStatus::Occupied,
                                process_name: Some(process_name.clone()),
                                pid,
                            });
                        }
                    }
                }
            }
        }

        // Deduplicate by port
        ports.sort_by_key(|p| p.port);
        ports.dedup_by_key(|p| p.port);
        ports
    }

    #[cfg(target_os = "linux")]
    fn parse_ss_output(&self, output: &str) -> Vec<PortInfo> {
        let mut ports = vec![];

        for line in output.lines().skip(1) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                // Parse local address (e.g., "0.0.0.0:8080")
                if let Some(addr) = parts.get(4) {
                    if let Some(port_str) = addr.rsplit(':').next() {
                        if let Ok(port) = port_str.parse::<u16>() {
                            let protocol = if line.starts_with("tcp") {
                                Protocol::Tcp
                            } else {
                                Protocol::Udp
                            };

                            let (process_name, pid) = if let Some(process_info) = parts.get(6) {
                                // Parse users:((\"process\",pid=1234,...))
                                let name = process_info
                                    .split('"')
                                    .nth(1)
                                    .map(String::from);
                                let pid = process_info
                                    .split("pid=")
                                    .nth(1)
                                    .and_then(|s| s.split(',').next())
                                    .and_then(|s| s.parse().ok());
                                (name, pid)
                            } else {
                                (None, None)
                            };

                            ports.push(PortInfo {
                                port,
                                protocol,
                                status: PortStatus::Occupied,
                                process_name,
                                pid,
                            });
                        }
                    }
                }
            }
        }

        ports
    }

    #[cfg(target_os = "linux")]
    fn parse_netstat_output(&self, output: &str) -> Vec<PortInfo> {
        let mut ports = vec![];

        for line in output.lines().skip(2) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 {
                if let Some(addr) = parts.get(3) {
                    if let Some(port_str) = addr.rsplit(':').next() {
                        if let Ok(port) = port_str.parse::<u16>() {
                            let protocol = if line.starts_with("tcp") {
                                Protocol::Tcp
                            } else {
                                Protocol::Udp
                            };

                            let (process_name, pid) = if let Some(process_info) = parts.last() {
                                let mut split = process_info.split('/');
                                let pid = split.next().and_then(|s| s.parse().ok());
                                let name = split.next().map(String::from);
                                (name, pid)
                            } else {
                                (None, None)
                            };

                            ports.push(PortInfo {
                                port,
                                protocol,
                                status: PortStatus::Occupied,
                                process_name,
                                pid,
                            });
                        }
                    }
                }
            }
        }

        ports
    }

    #[cfg(target_os = "windows")]
    fn parse_netstat_windows_output(&self, output: &str) -> Vec<PortInfo> {
        let mut ports = vec![];

        for line in output.lines().skip(4) {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 5 {
                let protocol = match parts.get(0) {
                    Some(&"TCP") => Protocol::Tcp,
                    Some(&"UDP") => Protocol::Udp,
                    _ => continue,
                };

                if let Some(addr) = parts.get(1) {
                    if let Some(port_str) = addr.rsplit(':').next() {
                        if let Ok(port) = port_str.parse::<u16>() {
                            let pid = parts.last().and_then(|s| s.parse().ok());

                            ports.push(PortInfo {
                                port,
                                protocol,
                                status: PortStatus::Occupied,
                                process_name: None,
                                pid,
                            });
                        }
                    }
                }
            }
        }

        ports
    }

    /// Find free ports in a range
    pub fn find_free_ports(&self, start: u16, end: u16, count: usize) -> Vec<u16> {
        let occupied: std::collections::HashSet<u16> = self
            .get_port_usage()
            .iter()
            .map(|p| p.port)
            .collect();

        (start..=end)
            .filter(|port| !occupied.contains(port))
            .take(count)
            .collect()
    }
}
