use async_trait::async_trait;
use sysinfo::{System, Pid, ProcessesToUpdate};
use super::traits::ServiceControl;

pub struct ProcessControl {
    system: System,
}

impl ProcessControl {
    pub fn new() -> Self {
        Self {
            system: System::new_all(),
        }
    }
}

#[async_trait]
impl ServiceControl for ProcessControl {
    async fn start(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Cannot start a generic process without knowing the command
        Err("Cannot start a process - path information required".into())
    }

    async fn stop(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pid: u32 = service_id.parse()?;
        let pid = Pid::from_u32(pid);

        if let Some(process) = self.system.process(pid) {
            process.kill_with(sysinfo::Signal::Term);
            Ok(())
        } else {
            Err(format!("Process {} not found", service_id).into())
        }
    }

    async fn restart(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Cannot restart a generic process".into())
    }

    async fn kill(&self, service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let pid: u32 = service_id.parse()?;
        let pid = Pid::from_u32(pid);

        if let Some(process) = self.system.process(pid) {
            process.kill_with(sysinfo::Signal::Kill);
            Ok(())
        } else {
            Err(format!("Process {} not found", service_id).into())
        }
    }

    async fn enable_autostart(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Autostart wird für allgemeine Prozesse nicht unterstützt. Verwenden Sie die Systemeinstellungen oder einen Service-Manager.".into())
    }

    async fn disable_autostart(&self, _service_id: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Err("Autostart wird für allgemeine Prozesse nicht unterstützt. Verwenden Sie die Systemeinstellungen oder einen Service-Manager.".into())
    }

    fn can_handle(&self, service_type: &str) -> bool {
        service_type == "process"
    }
}
