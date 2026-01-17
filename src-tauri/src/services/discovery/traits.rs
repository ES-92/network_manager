use crate::models::service::Service;
use async_trait::async_trait;

/// Common trait for service discovery across different platforms and service types
#[async_trait]
pub trait ServiceDiscovery: Send + Sync {
    /// Discover all services of this type
    async fn discover(&self) -> Result<Vec<Service>, Box<dyn std::error::Error + Send + Sync>>;

    /// Get a specific service by ID
    async fn get_service(&self, id: &str) -> Result<Option<Service>, Box<dyn std::error::Error + Send + Sync>>;

    /// Check if this discovery method is available on the current system
    fn is_available(&self) -> bool;

    /// Get the name of this discovery provider
    fn provider_name(&self) -> &'static str;
}
