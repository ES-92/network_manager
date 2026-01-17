use async_trait::async_trait;
use std::process::Command;
use super::traits::ServiceControl;

pub struct LaunchdControl;

impl LaunchdControl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceControl for LaunchdControl {
    async fn start(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("launchctl")
            .args(["start", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to start service: {}", stderr).into());
        }
        Ok(())
    }

    async fn stop(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("launchctl")
            .args(["stop", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to stop service: {}", stderr).into());
        }
        Ok(())
    }

    async fn restart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.stop(service_id).await?;
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        self.start(service_id).await?;
        Ok(())
    }

    async fn kill(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("launchctl")
            .args(["kill", "SIGKILL", service_id])
            .output()?;

        if !output.status.success() {
            // Fallback to regular stop
            self.stop(service_id).await?;
        }
        Ok(())
    }

    async fn enable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get current user ID for user-level services
        let uid_output = Command::new("id").arg("-u").output()?;
        let uid = String::from_utf8_lossy(&uid_output.stdout).trim().to_string();
        let domain_target = format!("gui/{}", uid);

        // Try to enable the service
        let output = Command::new("launchctl")
            .args(["enable", &format!("{}/{}", domain_target, service_id)])
            .output()?;

        if !output.status.success() {
            // Try alternative: load the plist
            let plist_paths = [
                format!("/Library/LaunchAgents/{}.plist", service_id),
                format!("{}/Library/LaunchAgents/{}.plist", std::env::var("HOME").unwrap_or_default(), service_id),
                format!("/Library/LaunchDaemons/{}.plist", service_id),
            ];

            for plist_path in plist_paths {
                if std::path::Path::new(&plist_path).exists() {
                    let load_output = Command::new("launchctl")
                        .args(["load", "-w", &plist_path])
                        .output()?;

                    if load_output.status.success() {
                        return Ok(());
                    }
                }
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to enable autostart: {}", stderr).into());
        }
        Ok(())
    }

    async fn disable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Get current user ID for user-level services
        let uid_output = Command::new("id").arg("-u").output()?;
        let uid = String::from_utf8_lossy(&uid_output.stdout).trim().to_string();
        let domain_target = format!("gui/{}", uid);

        // Try to disable the service
        let output = Command::new("launchctl")
            .args(["disable", &format!("{}/{}", domain_target, service_id)])
            .output()?;

        if !output.status.success() {
            // Try alternative: unload the plist with -w flag
            let plist_paths = [
                format!("/Library/LaunchAgents/{}.plist", service_id),
                format!("{}/Library/LaunchAgents/{}.plist", std::env::var("HOME").unwrap_or_default(), service_id),
                format!("/Library/LaunchDaemons/{}.plist", service_id),
            ];

            for plist_path in plist_paths {
                if std::path::Path::new(&plist_path).exists() {
                    let unload_output = Command::new("launchctl")
                        .args(["unload", "-w", &plist_path])
                        .output()?;

                    if unload_output.status.success() {
                        return Ok(());
                    }
                }
            }

            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to disable autostart: {}", stderr).into());
        }
        Ok(())
    }

    fn can_handle(&self, service_type: &str) -> bool {
        service_type == "launchd"
    }

    fn supports_autostart(&self) -> bool {
        true
    }
}
