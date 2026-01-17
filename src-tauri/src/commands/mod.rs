// Command handlers for Tauri IPC

pub mod service_commands;
pub mod port_commands;
pub mod config_commands;
pub mod audit_commands;
pub mod llm_commands;
pub mod system_commands;

// Re-export commands for easier registration
pub use service_commands::*;
pub use port_commands::*;
pub use config_commands::*;
pub use audit_commands::*;
pub use llm_commands::*;
pub use system_commands::*;
