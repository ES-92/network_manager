use serde::{Deserialize, Serialize};
use crate::models::service::Service;
use crate::services::port::resolver::PortResolver;
use std::collections::HashSet;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SecuritySeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SecurityCategory {
    UnencryptedConnection,
    PublicExposure,
    DefaultCredentials,
    OutdatedSoftware,
    MissingAuthentication,
    InsecureConfiguration,
    PrivilegeEscalation,
    DataLeakage,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityIssue {
    pub id: String,
    pub service_id: Option<String>,
    pub service_name: Option<String>,
    pub category: SecurityCategory,
    pub severity: SecuritySeverity,
    pub title: String,
    pub description: String,
    pub recommendation: String,
    pub port: Option<u16>,
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityScanResult {
    pub issues: Vec<SecurityIssue>,
    pub scan_timestamp: u64,
    pub services_scanned: usize,
    pub ports_scanned: usize,
    pub critical_count: usize,
    pub high_count: usize,
    pub medium_count: usize,
    pub low_count: usize,
}

// Known insecure ports and their issues
const INSECURE_PORTS: &[(u16, &str, &str)] = &[
    (21, "FTP", "FTP überträgt Daten unverschlüsselt, inkl. Passwörter"),
    (23, "Telnet", "Telnet ist unverschlüsselt, verwende SSH stattdessen"),
    (25, "SMTP", "SMTP ohne TLS überträgt E-Mails unverschlüsselt"),
    (69, "TFTP", "TFTP hat keine Authentifizierung"),
    (80, "HTTP", "HTTP ist unverschlüsselt, verwende HTTPS"),
    (110, "POP3", "POP3 ohne TLS überträgt E-Mails unverschlüsselt"),
    (143, "IMAP", "IMAP ohne TLS überträgt E-Mails unverschlüsselt"),
    (161, "SNMP", "SNMP v1/v2 hat schwache Authentifizierung"),
    (389, "LDAP", "LDAP ohne TLS überträgt Verzeichnisdaten unverschlüsselt"),
    (445, "SMB", "SMB kann für Angriffe missbraucht werden"),
    (512, "rexec", "Remote Execution ohne starke Authentifizierung"),
    (513, "rlogin", "Remote Login ist unsicher, verwende SSH"),
    (514, "rsh", "Remote Shell ist unsicher, verwende SSH"),
    (1433, "MSSQL", "Datenbank sollte nicht öffentlich erreichbar sein"),
    (1521, "Oracle", "Datenbank sollte nicht öffentlich erreichbar sein"),
    (3306, "MySQL", "Datenbank sollte nicht öffentlich erreichbar sein"),
    (5432, "PostgreSQL", "Datenbank sollte nicht öffentlich erreichbar sein"),
    (6379, "Redis", "Redis hat oft keine Authentifizierung"),
    (11211, "Memcached", "Memcached hat keine Authentifizierung"),
    (27017, "MongoDB", "MongoDB sollte nicht öffentlich erreichbar sein"),
];

// Ports that indicate services listening on all interfaces
const DATABASE_PORTS: &[u16] = &[1433, 1521, 3306, 5432, 6379, 11211, 27017, 5984, 9200, 9300];

pub struct SecurityScanner {
    port_resolver: PortResolver,
}

impl SecurityScanner {
    pub fn new() -> Self {
        Self {
            port_resolver: PortResolver::new(),
        }
    }

    pub fn scan(&self, services: &[Service]) -> SecurityScanResult {
        let mut issues = Vec::new();
        let port_usage = self.port_resolver.get_port_usage();
        let open_ports: HashSet<u16> = port_usage.iter().map(|p| p.port).collect();

        // Check for insecure ports
        for &(port, name, description) in INSECURE_PORTS {
            if open_ports.contains(&port) {
                let service = services.iter().find(|s| s.ports.contains(&port));
                let severity = self.get_port_severity(port);

                issues.push(SecurityIssue {
                    id: format!("port-{}", port),
                    service_id: service.map(|s| s.id.clone()),
                    service_name: service.map(|s| s.name.clone()),
                    category: SecurityCategory::UnencryptedConnection,
                    severity,
                    title: format!("{} Port {} ist offen", name, port),
                    description: description.to_string(),
                    recommendation: self.get_port_recommendation(port),
                    port: Some(port),
                    details: None,
                });
            }
        }

        // Check for databases exposed on all interfaces
        for port_info in &port_usage {
            if DATABASE_PORTS.contains(&port_info.port) {
                // Check if listening on 0.0.0.0 or ::
                let is_public = self.is_port_public(port_info.port);
                if is_public {
                    let service = services.iter().find(|s| s.ports.contains(&port_info.port));
                    issues.push(SecurityIssue {
                        id: format!("public-db-{}", port_info.port),
                        service_id: service.map(|s| s.id.clone()),
                        service_name: port_info.process_name.clone(),
                        category: SecurityCategory::PublicExposure,
                        severity: SecuritySeverity::Critical,
                        title: format!("Datenbank auf Port {} ist öffentlich erreichbar", port_info.port),
                        description: "Datenbanken sollten nicht von außen erreichbar sein".to_string(),
                        recommendation: "Binde die Datenbank an localhost (127.0.0.1) oder verwende eine Firewall".to_string(),
                        port: Some(port_info.port),
                        details: port_info.process_name.clone(),
                    });
                }
            }
        }

        // Check services for common security issues
        for service in services {
            self.check_service_security(service, &mut issues);
        }

        // Check for services running as root (on Unix)
        #[cfg(unix)]
        self.check_root_services(services, &mut issues);

        // Count by severity
        let critical_count = issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::Critical)).count();
        let high_count = issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::High)).count();
        let medium_count = issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::Medium)).count();
        let low_count = issues.iter().filter(|i| matches!(i.severity, SecuritySeverity::Low)).count();

        SecurityScanResult {
            issues,
            scan_timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            services_scanned: services.len(),
            ports_scanned: open_ports.len(),
            critical_count,
            high_count,
            medium_count,
            low_count,
        }
    }

    fn get_port_severity(&self, port: u16) -> SecuritySeverity {
        match port {
            23 | 512 | 513 | 514 => SecuritySeverity::Critical, // Telnet, r-services
            21 | 69 => SecuritySeverity::High, // FTP, TFTP
            80 | 110 | 143 | 389 => SecuritySeverity::Medium, // Unencrypted but common
            25 | 161 | 445 => SecuritySeverity::Medium,
            _ => SecuritySeverity::Low,
        }
    }

    fn get_port_recommendation(&self, port: u16) -> String {
        match port {
            21 => "Verwende SFTP (Port 22) statt FTP".to_string(),
            23 => "Verwende SSH (Port 22) statt Telnet".to_string(),
            25 => "Aktiviere STARTTLS oder verwende Port 587 mit TLS".to_string(),
            80 => "Aktiviere HTTPS und leite HTTP auf HTTPS um".to_string(),
            110 => "Verwende POP3S (Port 995) mit TLS".to_string(),
            143 => "Verwende IMAPS (Port 993) mit TLS".to_string(),
            389 => "Verwende LDAPS (Port 636) mit TLS".to_string(),
            445 => "Beschränke SMB-Zugriff auf lokales Netzwerk".to_string(),
            _ if DATABASE_PORTS.contains(&port) => {
                "Binde Datenbank an localhost und verwende SSH-Tunnel für Remote-Zugriff".to_string()
            }
            _ => "Prüfe ob dieser Port wirklich öffentlich erreichbar sein muss".to_string(),
        }
    }

    fn is_port_public(&self, port: u16) -> bool {
        // Check lsof/netstat output for binding address
        #[cfg(target_os = "macos")]
        {
            let output = std::process::Command::new("lsof")
                .args(["-i", &format!(":{}", port), "-P", "-n"])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // If it shows *:port or 0.0.0.0:port, it's public
                return stdout.contains(&format!("*:{}", port))
                    || stdout.contains(&format!("0.0.0.0:{}", port))
                    || stdout.contains(&format!("[::]:{}", port));
            }
        }

        #[cfg(target_os = "linux")]
        {
            let output = std::process::Command::new("ss")
                .args(["-tlnp", &format!("sport = :{}", port)])
                .output();

            if let Ok(output) = output {
                let stdout = String::from_utf8_lossy(&output.stdout);
                return stdout.contains("0.0.0.0") || stdout.contains("[::]") || stdout.contains("*:");
            }
        }

        false
    }

    fn check_service_security(&self, service: &Service, issues: &mut Vec<SecurityIssue>) {
        let name_lower = service.name.to_lowercase();

        // Check for known vulnerable services
        if name_lower.contains("redis") && service.ports.contains(&6379) {
            issues.push(SecurityIssue {
                id: format!("redis-auth-{}", service.id),
                service_id: Some(service.id.clone()),
                service_name: Some(service.name.clone()),
                category: SecurityCategory::MissingAuthentication,
                severity: SecuritySeverity::High,
                title: "Redis möglicherweise ohne Authentifizierung".to_string(),
                description: "Redis hat standardmäßig keine Passwort-Authentifizierung".to_string(),
                recommendation: "Setze ein Passwort mit 'requirepass' in redis.conf".to_string(),
                port: Some(6379),
                details: None,
            });
        }

        if name_lower.contains("mongodb") && service.ports.contains(&27017) {
            issues.push(SecurityIssue {
                id: format!("mongo-auth-{}", service.id),
                service_id: Some(service.id.clone()),
                service_name: Some(service.name.clone()),
                category: SecurityCategory::MissingAuthentication,
                severity: SecuritySeverity::High,
                title: "MongoDB möglicherweise ohne Authentifizierung".to_string(),
                description: "MongoDB hat standardmäßig keine Authentifizierung aktiviert".to_string(),
                recommendation: "Aktiviere Authentifizierung mit --auth Flag".to_string(),
                port: Some(27017),
                details: None,
            });
        }

        if name_lower.contains("elasticsearch") {
            issues.push(SecurityIssue {
                id: format!("elastic-auth-{}", service.id),
                service_id: Some(service.id.clone()),
                service_name: Some(service.name.clone()),
                category: SecurityCategory::MissingAuthentication,
                severity: SecuritySeverity::Medium,
                title: "Elasticsearch Security prüfen".to_string(),
                description: "Elasticsearch X-Pack Security sollte aktiviert sein".to_string(),
                recommendation: "Aktiviere X-Pack Security für Authentifizierung und TLS".to_string(),
                port: service.ports.first().copied(),
                details: None,
            });
        }

        // Check Docker containers for privileged mode or host network
        if service.service_type == crate::models::service::ServiceType::Docker {
            // Note: Would need Docker API to check these details
            // For now, just flag Docker services for review
        }
    }

    #[cfg(unix)]
    fn check_root_services(&self, services: &[Service], issues: &mut Vec<SecurityIssue>) {
        for service in services {
            if let Some(pid) = service.pid {
                // Check if process is running as root
                let output = std::process::Command::new("ps")
                    .args(["-o", "user=", "-p", &pid.to_string()])
                    .output();

                if let Ok(output) = output {
                    let user = String::from_utf8_lossy(&output.stdout).trim().to_string();
                    if user == "root" && !self.is_system_service(&service.name) {
                        issues.push(SecurityIssue {
                            id: format!("root-{}", service.id),
                            service_id: Some(service.id.clone()),
                            service_name: Some(service.name.clone()),
                            category: SecurityCategory::PrivilegeEscalation,
                            severity: SecuritySeverity::Medium,
                            title: format!("{} läuft als root", service.name),
                            description: "Services sollten mit minimalen Rechten laufen".to_string(),
                            recommendation: "Erstelle einen dedizierten Benutzer für diesen Service".to_string(),
                            port: service.ports.first().copied(),
                            details: Some(format!("PID: {}", pid)),
                        });
                    }
                }
            }
        }
    }

    fn is_system_service(&self, name: &str) -> bool {
        let system_prefixes = ["com.apple.", "systemd", "launchd", "kernel", "init"];
        system_prefixes.iter().any(|prefix| name.to_lowercase().starts_with(prefix))
    }
}

impl Default for SecurityScanner {
    fn default() -> Self {
        Self::new()
    }
}
