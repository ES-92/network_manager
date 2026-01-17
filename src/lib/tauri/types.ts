// Service types
export type ServiceStatus = "running" | "stopped" | "error" | "unknown";
export type ServiceType = "docker" | "systemd" | "launchd" | "windows_service" | "process";

export interface Service {
  id: string;
  name: string;
  status: ServiceStatus;
  service_type: ServiceType;
  ports: number[];
  pid: number | null;
  path: string | null;
  description: string | null;
  auto_start: boolean;
  /** CPU usage as percentage (0.0 - 100.0) */
  cpu_usage: number | null;
  /** Memory usage in bytes */
  memory_bytes: number | null;
  /** Memory usage as percentage of total system memory */
  memory_percent: number | null;
}

// Port types
export type Protocol = "tcp" | "udp";
export type PortStatus = "occupied" | "free";

export interface PortInfo {
  port: number;
  protocol: Protocol;
  status: PortStatus;
  process_name: string | null;
  pid: number | null;
}

// Audit types
export type EventType =
  | "service_start"
  | "service_stop"
  | "service_restart"
  | "process_kill"
  | "config_change"
  | "privilege_escalation"
  | "llm_analysis"
  | "port_scan";

export interface AuditEntry {
  id: string;
  timestamp: string;
  event_type: EventType;
  user: string;
  operation: string;
  service_id: string | null;
  success: boolean;
  error_message: string | null;
  details: Record<string, unknown>;
}

// Recommendation types
export type RecommendationType =
  | "stop_service"
  | "disable_autostart"
  | "reduce_resources"
  | "security_concern"
  | "performance_impact"
  | "info";

export interface ServiceRecommendation {
  service_id: string;
  service_name: string;
  recommendation_type: RecommendationType;
  title: string;
  description: string;
  action: string | null;
}

// System Stats types
export interface CpuStats {
  usage_percent: number;
  core_count: number;
  per_core_usage: number[];
  frequency_mhz: number | null;
}

export interface MemoryStats {
  total_bytes: number;
  used_bytes: number;
  available_bytes: number;
  usage_percent: number;
  swap_total_bytes: number;
  swap_used_bytes: number;
}

export interface GpuStats {
  name: string;
  usage_percent: number | null;
  memory_used_bytes: number | null;
  memory_total_bytes: number | null;
  temperature_celsius: number | null;
  power_watts: number | null;
}

export interface SystemStats {
  cpu: CpuStats;
  memory: MemoryStats;
  gpus: GpuStats[];
  timestamp: number;
}

export type GpuProvider = "auto" | "apple" | "nvidia" | "amd" | "none";

// Security types
export type SecuritySeverity = "critical" | "high" | "medium" | "low" | "info";

export type SecurityCategory =
  | "unencrypted_connection"
  | "public_exposure"
  | "default_credentials"
  | "outdated_software"
  | "missing_authentication"
  | "insecure_configuration"
  | "privilege_escalation"
  | "data_leakage";

export interface SecurityIssue {
  id: string;
  service_id: string | null;
  service_name: string | null;
  category: SecurityCategory;
  severity: SecuritySeverity;
  title: string;
  description: string;
  recommendation: string;
  port: number | null;
  details: string | null;
}

export interface SecurityScanResult {
  issues: SecurityIssue[];
  scan_timestamp: number;
  services_scanned: number;
  ports_scanned: number;
  critical_count: number;
  high_count: number;
  medium_count: number;
  low_count: number;
}

// Config types
export type ThemeMode = "system" | "light" | "dark";

export interface Config {
  theme: {
    mode: ThemeMode;
  };
  refresh_interval_ms: number;
  ollama: {
    enabled: boolean;
    endpoint: string;
    model: string;
    timeout_seconds: number;
  };
  security: {
    audit_logging: boolean;
    require_confirmation_for_kill: boolean;
    privilege_cache_ttl_minutes: number;
  };
}
