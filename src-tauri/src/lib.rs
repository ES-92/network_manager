// Network Manager - Service Management Application
// Cross-platform service discovery, port management, and monitoring

pub mod commands;
pub mod models;
pub mod services;
pub mod llm;

use commands::{
    discover_services, get_service_details, start_service, stop_service, restart_service, kill_process,
    enable_service_autostart, disable_service_autostart,
    scan_ports, get_port_usage, find_free_ports,
    get_config, update_config,
    get_audit_logs, export_audit_logs,
    check_ollama_status, list_ollama_models, analyze_logs, set_ollama_model,
    explain_process, get_service_recommendations,
    get_system_stats, set_gpu_provider, scan_security, get_security_analysis,
};

use services::{MonitorState, set_monitor_interval, enable_monitor};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(MonitorState::default())
        .setup(|_app| {
            // Service manager is initialized lazily in commands
            // Monitor disabled - frontend handles refresh via polling
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Service commands
            discover_services,
            get_service_details,
            start_service,
            stop_service,
            restart_service,
            kill_process,
            enable_service_autostart,
            disable_service_autostart,
            // Port commands
            scan_ports,
            get_port_usage,
            find_free_ports,
            // Config commands
            get_config,
            update_config,
            // Audit commands
            get_audit_logs,
            export_audit_logs,
            // LLM commands
            check_ollama_status,
            list_ollama_models,
            analyze_logs,
            set_ollama_model,
            explain_process,
            get_service_recommendations,
            // Monitor commands
            set_monitor_interval,
            enable_monitor,
            // System stats commands
            get_system_stats,
            set_gpu_provider,
            // Security commands
            scan_security,
            get_security_analysis,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
