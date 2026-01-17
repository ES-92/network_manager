use std::net::{SocketAddr, TcpStream};
use std::time::Duration;
use tokio::sync::Semaphore;
use std::sync::Arc;
use crate::models::port::{PortInfo, Protocol, PortStatus};

pub struct PortScanner {
    timeout: Duration,
    max_concurrent: usize,
}

impl PortScanner {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_millis(200),
            max_concurrent: 100,
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn with_concurrency(mut self, max: usize) -> Self {
        self.max_concurrent = max;
        self
    }

    /// Scan a single port
    pub fn scan_port(&self, host: &str, port: u16) -> bool {
        let addr: Result<SocketAddr, _> = format!("{}:{}", host, port).parse();
        match addr {
            Ok(addr) => TcpStream::connect_timeout(&addr, self.timeout).is_ok(),
            Err(_) => false,
        }
    }

    /// Scan a range of ports
    pub async fn scan_range(&self, host: &str, start: u16, end: u16) -> Vec<PortInfo> {
        let semaphore = Arc::new(Semaphore::new(self.max_concurrent));
        let mut handles = vec![];

        for port in start..=end {
            let permit = semaphore.clone().acquire_owned().await.unwrap();
            let host = host.to_string();
            let timeout = self.timeout;

            let handle = tokio::task::spawn_blocking(move || {
                let addr: Result<SocketAddr, _> = format!("{}:{}", host, port).parse();
                let is_open = match addr {
                    Ok(addr) => TcpStream::connect_timeout(&addr, timeout).is_ok(),
                    Err(_) => false,
                };
                drop(permit);
                (port, is_open)
            });

            handles.push(handle);
        }

        let mut results = vec![];
        for handle in handles {
            if let Ok((port, is_open)) = handle.await {
                if is_open {
                    results.push(PortInfo {
                        port,
                        protocol: Protocol::Tcp,
                        status: PortStatus::Occupied,
                        process_name: None,
                        pid: None,
                    });
                }
            }
        }

        results
    }

    /// Scan common service ports
    pub async fn scan_common_ports(&self, host: &str) -> Vec<PortInfo> {
        let common_ports = vec![
            20, 21, 22, 23, 25, 53, 80, 110, 143, 443, 465, 587, 993, 995,
            3000, 3306, 5432, 5672, 6379, 8000, 8080, 8443, 9000, 27017,
        ];

        let mut results = vec![];
        for &port in &common_ports {
            if self.scan_port(host, port) {
                results.push(PortInfo {
                    port,
                    protocol: Protocol::Tcp,
                    status: PortStatus::Occupied,
                    process_name: None,
                    pid: None,
                });
            }
        }
        results
    }
}
