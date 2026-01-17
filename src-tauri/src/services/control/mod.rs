// Service control modules

pub mod traits;
pub mod docker_control;
pub mod process_control;

#[cfg(target_os = "macos")]
pub mod launchd_control;

#[cfg(target_os = "linux")]
pub mod systemd_control;

#[cfg(target_os = "windows")]
pub mod windows_control;

pub use traits::ServiceControl;
