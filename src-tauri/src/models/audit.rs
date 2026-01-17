use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub event_type: EventType,
    pub user: String,
    pub operation: String,
    pub service_id: Option<String>,
    pub success: bool,
    pub error_message: Option<String>,
    pub details: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    ServiceStart,
    ServiceStop,
    ServiceRestart,
    ProcessKill,
    ConfigChange,
    PrivilegeEscalation,
    LlmAnalysis,
    PortScan,
}

impl AuditEntry {
    pub fn new(event_type: EventType, operation: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            event_type,
            user: whoami::username(),
            operation,
            service_id: None,
            success: true,
            error_message: None,
            details: serde_json::json!({}),
        }
    }
}
