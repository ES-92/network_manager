use async_trait::async_trait;
use super::traits::ServiceControl;

#[cfg(target_os = "windows")]
use std::process::Command;

pub struct WindowsControl;

impl WindowsControl {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ServiceControl for WindowsControl {
    #[cfg(target_os = "windows")]
    async fn start(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("sc")
            .args(["start", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to start service: {}", stderr).into());
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    async fn start(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Windows services not available on this platform".into())
    }

    #[cfg(target_os = "windows")]
    async fn stop(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("sc")
            .args(["stop", service_id])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to stop service: {}", stderr).into());
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    async fn stop(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Windows services not available on this platform".into())
    }

    async fn restart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.stop(service_id).await?;
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        self.start(service_id).await?;
        Ok(())
    }

    async fn kill(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Windows services don't have a kill - use stop
        self.stop(service_id).await
    }

    fn can_handle(&self, service_type: &str) -> bool {
        service_type == "windows_service"
    }

    #[cfg(target_os = "windows")]
    async fn enable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("sc")
            .args(["config", service_id, "start=", "auto"])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to enable autostart: {}", stderr).into());
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    async fn enable_autostart(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Windows services not available on this platform".into())
    }

    #[cfg(target_os = "windows")]
    async fn disable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let output = Command::new("sc")
            .args(["config", service_id, "start=", "demand"])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(format!("Failed to disable autostart: {}", stderr).into());
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    async fn disable_autostart(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Windows services not available on this platform".into())
    }
}
