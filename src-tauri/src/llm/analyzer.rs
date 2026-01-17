use super::client::OllamaClient;
use regex::Regex;
use serde::{Deserialize, Serialize};

pub struct LogAnalyzer {
    client: OllamaClient,
}

#[derive(Clone, Copy)]
pub enum AnalysisType {
    ErrorDetection,
    PatternAnalysis,
    AnomalyDetection,
    PerformanceAnalysis,
    SecurityAnalysis,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceRecommendation {
    pub service_id: String,
    pub service_name: String,
    pub recommendation_type: RecommendationType,
    pub title: String,
    pub description: String,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RecommendationType {
    StopService,
    DisableAutostart,
    ReduceResources,
    SecurityConcern,
    PerformanceImpact,
    Info,
}

impl LogAnalyzer {
    pub fn new(client: OllamaClient) -> Self {
        Self { client }
    }

    /// Sanitize logs by removing sensitive information
    pub fn sanitize_logs(&self, logs: &str) -> String {
        let patterns = vec![
            (r"password\s*[=:]\s*\S+", "password=***"),
            (r"api[_-]?key\s*[=:]\s*\S+", "api_key=***"),
            (r"token\s*[=:]\s*\S+", "token=***"),
            (r"secret\s*[=:]\s*\S+", "secret=***"),
            (r"bearer\s+\S+", "bearer ***"),
            (r"\b[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Z|a-z]{2,}\b", "email@***"),
            (r"\b\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3}\b", "x.x.x.x"),
        ];

        let mut sanitized = logs.to_string();
        for (pattern, replacement) in patterns {
            if let Ok(re) = Regex::new(pattern) {
                sanitized = re.replace_all(&sanitized, replacement).to_string();
            }
        }
        sanitized
    }

    /// Analyze logs with a specific analysis type
    pub async fn analyze(
        &self,
        logs: &str,
        analysis_type: AnalysisType,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Sanitize logs before sending to LLM
        let sanitized_logs = self.sanitize_logs(logs);

        // Truncate if too long
        let max_chars = 8000;
        let logs_to_analyze = if sanitized_logs.len() > max_chars {
            &sanitized_logs[sanitized_logs.len() - max_chars..]
        } else {
            &sanitized_logs
        };

        let prompt = self.build_prompt(logs_to_analyze, analysis_type);
        let response = self.client.generate(&prompt).await?;

        Ok(response)
    }

    /// Build analysis prompt based on type
    fn build_prompt(&self, logs: &str, analysis_type: AnalysisType) -> String {
        let instruction = match analysis_type {
            AnalysisType::ErrorDetection => {
                "Analyze these logs and identify all errors, exceptions, and failures. \
                 For each issue found, explain what went wrong and suggest potential fixes."
            }
            AnalysisType::PatternAnalysis => {
                "Analyze these logs and identify recurring patterns, common operations, \
                 and typical behavior. Highlight any unusual deviations from the norm."
            }
            AnalysisType::AnomalyDetection => {
                "Analyze these logs and identify any anomalies, unusual behavior, \
                 or suspicious activities that deviate from normal operation patterns."
            }
            AnalysisType::PerformanceAnalysis => {
                "Analyze these logs for performance issues. Look for slow operations, \
                 timeouts, resource exhaustion, or bottlenecks. Suggest optimizations."
            }
            AnalysisType::SecurityAnalysis => {
                "Analyze these logs for potential security concerns. Look for failed \
                 authentication attempts, suspicious access patterns, or potential attacks."
            }
        };

        format!(
            "You are a log analysis assistant. Your task is to analyze service logs and provide insights.\n\n\
             IMPORTANT: This is a READ-ONLY analysis. Do not suggest running commands or making changes to services.\n\n\
             {}\n\n\
             LOGS:\n```\n{}\n```\n\n\
             Provide a concise analysis with:\n\
             1. Summary of findings\n\
             2. Key issues or patterns identified\n\
             3. Recommendations (informational only)",
            instruction,
            logs
        )
    }

    /// Check if LLM analysis is available
    pub async fn is_available(&self) -> bool {
        self.client.is_available().await
    }

    /// Explain what a process/service does in natural language
    pub async fn explain_process(
        &self,
        process_name: &str,
        process_path: Option<&str>,
        _description: Option<&str>,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Use a very concise prompt for fast response
        let path_hint = process_path
            .map(|p| {
                // Extract just the app name from path if possible
                if let Some(app) = p.split('/').find(|s| s.ends_with(".app")) {
                    format!(" ({})", app)
                } else {
                    String::new()
                }
            })
            .unwrap_or_default();

        let prompt = format!(
            "Was macht der Prozess '{}{}'? Antworte auf Deutsch in 1-2 Sätzen. Nur Fakten, keine Einleitung.",
            process_name, path_hint
        );

        self.client.generate_fast(&prompt).await
    }

    /// Generate recommendations for services
    pub async fn generate_recommendations(
        &self,
        services_json: &str,
    ) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "You are a system optimization assistant. Analyze the following list of running services and provide recommendations for optimization.\n\n\
             Services (JSON):\n```json\n{}\n```\n\n\
             Focus on identifying:\n\
             1. Services that may be consuming unnecessary resources\n\
             2. Auto-start services that might not be needed\n\
             3. Duplicate or redundant services\n\
             4. Services that could be safely disabled\n\n\
             IMPORTANT: Respond ONLY with valid JSON array. Each recommendation must be an object with these exact fields:\n\
             - service_id: string (the service ID)\n\
             - service_name: string (the service name)\n\
             - recommendation_type: one of \"stop_service\", \"disable_autostart\", \"reduce_resources\", \"performance_impact\", \"info\"\n\
             - title: string (short title, 5-10 words)\n\
             - description: string (explanation in German, 1-2 sentences)\n\
             - action: string or null (suggested action if applicable)\n\n\
             Provide 3-5 recommendations. Be conservative - only suggest stopping services that are truly optional.\n\n\
             Response format example:\n\
             [\n\
               {{\"service_id\": \"123\", \"service_name\": \"ExampleService\", \"recommendation_type\": \"disable_autostart\", \"title\": \"Autostart nicht nötig\", \"description\": \"Dieser Dienst wird selten verwendet.\", \"action\": \"Autostart deaktivieren\"}}\n\
             ]",
            services_json
        );

        self.client.generate(&prompt).await
    }
}
