use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};
use std::path::PathBuf;
use chrono::Utc;
use crate::models::audit::{AuditEntry, EventType};

pub struct AuditLogger {
    log_path: PathBuf,
}

impl AuditLogger {
    pub fn new() -> Self {
        let log_dir = dirs::data_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("network_manager")
            .join("logs");

        std::fs::create_dir_all(&log_dir).ok();

        Self {
            log_path: log_dir.join("audit.jsonl"),
        }
    }

    pub fn with_path(path: PathBuf) -> Self {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).ok();
        }
        Self { log_path: path }
    }

    /// Log an audit event
    pub fn log(&self, entry: &AuditEntry) -> Result<(), Box<dyn std::error::Error>> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string(entry)?;
        writeln!(writer, "{}", json)?;
        writer.flush()?;

        Ok(())
    }

    /// Log a service control event
    pub fn log_service_event(
        &self,
        event_type: EventType,
        service_id: &str,
        operation: &str,
        success: bool,
        error: Option<&str>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut entry = AuditEntry::new(event_type, operation.to_string());
        entry.service_id = Some(service_id.to_string());
        entry.success = success;
        entry.error_message = error.map(String::from);

        self.log(&entry)
    }

    /// Get recent audit entries
    pub fn get_entries(&self, limit: usize) -> Result<Vec<AuditEntry>, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&self.log_path)?;
        let entries: Vec<AuditEntry> = content
            .lines()
            .rev()
            .take(limit)
            .filter_map(|line| serde_json::from_str(line).ok())
            .collect();

        Ok(entries)
    }

    /// Get the log file path
    pub fn log_path(&self) -> &PathBuf {
        &self.log_path
    }
}
