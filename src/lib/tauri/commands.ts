import { invoke } from "@tauri-apps/api/core";
import type {
  Service, PortInfo, Config, AuditEntry, ServiceRecommendation,
  SystemStats, GpuProvider, SecurityScanResult
} from "./types";

// Service commands
export async function discoverServices(): Promise<Service[]> {
  return invoke("discover_services");
}

export async function getServiceDetails(serviceId: string): Promise<Service | null> {
  return invoke("get_service_details", { serviceId });
}

export async function startService(serviceId: string): Promise<void> {
  return invoke("start_service", { serviceId });
}

export async function stopService(serviceId: string): Promise<void> {
  return invoke("stop_service", { serviceId });
}

export async function restartService(serviceId: string): Promise<void> {
  return invoke("restart_service", { serviceId });
}

export async function killProcess(pid: number): Promise<void> {
  return invoke("kill_process", { pid });
}

export async function enableServiceAutostart(
  serviceId: string,
  serviceType: string
): Promise<void> {
  return invoke("enable_service_autostart", { serviceId, serviceType });
}

export async function disableServiceAutostart(
  serviceId: string,
  serviceType: string
): Promise<void> {
  return invoke("disable_service_autostart", { serviceId, serviceType });
}

// Port commands
export async function scanPorts(start: number, end: number): Promise<PortInfo[]> {
  return invoke("scan_ports", { start, end });
}

export async function getPortUsage(): Promise<PortInfo[]> {
  return invoke("get_port_usage");
}

export async function findFreePorts(count: number): Promise<number[]> {
  return invoke("find_free_ports", { count });
}

// Config commands
export async function getConfig(): Promise<Config> {
  return invoke("get_config");
}

export async function updateConfig(config: Config): Promise<void> {
  return invoke("update_config", { config });
}

// Audit commands
export async function getAuditLogs(limit?: number): Promise<AuditEntry[]> {
  return invoke("get_audit_logs", { limit });
}

export async function exportAuditLogs(format: string): Promise<string> {
  return invoke("export_audit_logs", { format });
}

// LLM commands
export async function checkOllamaStatus(): Promise<boolean> {
  return invoke("check_ollama_status");
}

export async function listOllamaModels(): Promise<string[]> {
  return invoke("list_ollama_models");
}

export async function analyzeLogs(logs: string, analysisType: string): Promise<string> {
  return invoke("analyze_logs", { logs, analysisType });
}

export async function setOllamaModel(model: string): Promise<void> {
  return invoke("set_ollama_model", { model });
}

export async function explainProcess(
  processName: string,
  processPath?: string | null,
  description?: string | null
): Promise<string> {
  return invoke("explain_process", {
    processName,
    processPath: processPath ?? null,
    description: description ?? null,
  });
}

export async function getServiceRecommendations(
  services: Service[]
): Promise<ServiceRecommendation[]> {
  const servicesJson = JSON.stringify(
    services.slice(0, 50).map((s) => ({
      id: s.id,
      name: s.name,
      status: s.status,
      type: s.service_type,
      ports: s.ports,
      auto_start: s.auto_start,
    }))
  );
  return invoke("get_service_recommendations", { servicesJson });
}

// Monitor commands
export async function setMonitorInterval(seconds: number): Promise<void> {
  return invoke("set_monitor_interval", { seconds });
}

export async function enableMonitor(enabled: boolean): Promise<void> {
  return invoke("enable_monitor", { enabled });
}

// System stats commands
export async function getSystemStats(): Promise<SystemStats> {
  return invoke("get_system_stats");
}

export async function setGpuProvider(provider: GpuProvider): Promise<void> {
  return invoke("set_gpu_provider", { provider });
}

// Security commands
export async function scanSecurity(): Promise<SecurityScanResult> {
  return invoke("scan_security");
}

export async function getSecurityAnalysis(servicesJson: string): Promise<string> {
  return invoke("get_security_analysis", { servicesJson });
}
