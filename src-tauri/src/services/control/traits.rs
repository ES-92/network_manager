use async_trait::async_trait;

/// Common trait for controlling services across different platforms
#[async_trait]
pub trait ServiceControl: Send + Sync {
    /// Start the service
    async fn start(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Stop the service
    async fn stop(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Restart the service
    async fn restart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Kill the service (force stop)
    async fn kill(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Enable autostart for the service
    async fn enable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Disable autostart for the service
    async fn disable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;

    /// Check if this controller can handle the given service
    fn can_handle(&self, service_type: &str) -> bool;

    /// Check if autostart control is supported for this service type
    fn supports_autostart(&self) -> bool {
        false
    }
}
