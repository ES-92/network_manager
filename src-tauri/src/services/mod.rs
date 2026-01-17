// Core service logic modules

pub mod discovery;
pub mod control;
pub mod port;
pub mod security;
pub mod manager;
pub mod monitor;
pub mod system_stats;
pub mod security_scanner;

pub use manager::ServiceManager;
pub use monitor::{ServiceMonitor, MonitorState, set_monitor_interval, enable_monitor};
pub use system_stats::{SystemMonitor, SystemStats, GpuProvider};
pub use security_scanner::{SecurityScanner, SecurityScanResult, SecurityIssue};
