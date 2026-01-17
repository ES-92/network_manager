use async_trait::async_trait;
use bollard::Docker;
#[allow(deprecated)]
use bollard::container::{StartContainerOptions, StopContainerOptions, RestartContainerOptions, KillContainerOptions, UpdateContainerOptions};
use bollard::models::RestartPolicy;
use super::traits::ServiceControl;

pub struct DockerControl {
    docker: Option<Docker>,
}

impl DockerControl {
    pub fn new() -> Self {
        let docker = Docker::connect_with_local_defaults().ok();
        Self { docker }
    }
}

#[async_trait]
impl ServiceControl for DockerControl {
    #[allow(deprecated)]
    async fn start(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let docker = self.docker.as_ref().ok_or("Docker not available")?;
        docker.start_container(service_id, None::<StartContainerOptions<String>>).await?;
        Ok(())
    }

    #[allow(deprecated)]
    async fn stop(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let docker = self.docker.as_ref().ok_or("Docker not available")?;
        docker.stop_container(service_id, Some(StopContainerOptions { t: 10 })).await?;
        Ok(())
    }

    #[allow(deprecated)]
    async fn restart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let docker = self.docker.as_ref().ok_or("Docker not available")?;
        docker.restart_container(service_id, Some(RestartContainerOptions { t: 10 })).await?;
        Ok(())
    }

    #[allow(deprecated)]
    async fn kill(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let docker = self.docker.as_ref().ok_or("Docker not available")?;
        docker.kill_container(service_id, Some(KillContainerOptions { signal: "SIGKILL" })).await?;
        Ok(())
    }

    async fn enable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let docker = self.docker.as_ref().ok_or("Docker not available")?;
        let config = UpdateContainerOptions::<String> {
            restart_policy: Some(RestartPolicy {
                name: Some(bollard::models::RestartPolicyNameEnum::ALWAYS),
                maximum_retry_count: None,
            }),
            ..Default::default()
        };
        docker.update_container(service_id, config).await?;
        Ok(())
    }

    async fn disable_autostart(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let docker = self.docker.as_ref().ok_or("Docker not available")?;
        let config = UpdateContainerOptions::<String> {
            restart_policy: Some(RestartPolicy {
                name: Some(bollard::models::RestartPolicyNameEnum::NO),
                maximum_retry_count: None,
            }),
            ..Default::default()
        };
        docker.update_container(service_id, config).await?;
        Ok(())
    }

    fn can_handle(&self, service_type: &str) -> bool {
        service_type == "docker"
    }
}
