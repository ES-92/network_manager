// Service discovery modules

pub mod traits;
pub mod docker;
pub mod process;

#[cfg(target_os = "macos")]
pub mod launchd;

#[cfg(target_os = "linux")]
pub mod systemd;

#[cfg(target_os = "windows")]
pub mod windows_service;

pub use traits::ServiceDiscovery;
