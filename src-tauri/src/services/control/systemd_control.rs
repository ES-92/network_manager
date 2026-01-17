use async_trait::async_trait;
use std::process::Command;
use super::traits::ServiceControl;

pub struct SystemdControl;

impl SystemdControl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceControl for SystemdControl {
    async fn start(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("systemctl")
            .args(["start", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to start service: {}", stderr).into());
        }
        Ok(())
    }

    async fn stop(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("systemctl")
            .args(["stop", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to stop service: {}", stderr).into());
        }
        Ok(())
    }

    async fn restart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("systemctl")
            .args(["restart", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to restart service: {}", stderr).into());
        }
        Ok(())
    }

    async fn kill(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("systemctl")
            .args(["kill", "--signal=SIGKILL", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to kill service: {}", stderr).into());
        }
        Ok(())
    }

    fn can_handle(&self, service_type: &str) -> bool {
        service_type == "systemd"
    }
}
