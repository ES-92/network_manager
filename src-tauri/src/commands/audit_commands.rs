use crate::services::security::AuditLogger;
use std::sync::OnceLock;

static AUDIT_LOGGER: OnceLock<AuditLogger> = OnceLock::new();

fn get_logger() -> &'static AuditLogger {
    AUDIT_LOGGER.get_or_init(AuditLogger::new)
}

#[tauri::command]
pub async fn get_audit_logs(limit: Option<u32>) -> Result<Vec<serde_json::Value>, String> {
    let logger = get_logger();
    let entries = logger
        .get_entries(limit.unwrap_or(100) as usize)
        .map_err(|e| e.to_string())?;

    let values: Vec<serde_json::Value> = entries
        .iter()
        .filter_map(|e| serde_json::to_value(e).ok())
        .collect();

    Ok(values)
}

#[tauri::command]
pub async fn export_audit_logs(format: String) -> Result<String, String> {
    let logger = get_logger();
    let entries = logger
        .get_entries(10000)
        .map_err(|e| e.to_string())?;

    match format.as_str() {
        "json" => serde_json::to_string_pretty(&entries).map_err(|e| e.to_string()),
        "csv" => {
            let mut csv = String::from("timestamp,event_type,user,operation,service_id,success,error_message\n");
            for entry in entries {
                csv.push_str(&format!(
                    "{},{:?},{},{},{},{},{}\n",
                    entry.timestamp,
                    entry.event_type,
                    entry.user,
                    entry.operation,
                    entry.service_id.unwrap_or_default(),
                    entry.success,
                    entry.error_message.unwrap_or_default()
                ));
            }
            Ok(csv)
        }
        _ => Err(format!("Unsupported format: {}", format)),
    }
}
