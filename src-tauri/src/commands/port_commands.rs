use crate::models::port::PortInfo;
use crate::services::port::{PortScanner, PortResolver};

#[tauri::command]
pub async fn scan_ports(start: u16, end: u16) -> Result<Vec<PortInfo>, String> {
    let scanner = PortScanner::new();
    let ports = scanner.scan_range("127.0.0.1", start, end).await;
    Ok(ports)
}

#[tauri::command]
pub async fn get_port_usage() -> Result<Vec<PortInfo>, String> {
    let resolver = PortResolver::new();
    Ok(resolver.get_port_usage())
}

#[tauri::command]
pub async fn find_free_ports(count: u16) -> Result<Vec<u16>, String> {
    let resolver = PortResolver::new();
    Ok(resolver.find_free_ports(1024, 65535, count as usize))
}
