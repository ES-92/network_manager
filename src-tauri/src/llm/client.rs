use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_ENDPOINT: &str = "http://localhost:11434";
const DEFAULT_MODEL: &str = "mistral:7b-instruct";
const DEFAULT_TIMEOUT: u64 = 30;

#[derive(Clone)]
pub struct OllamaClient {
    client: Client,
    endpoint: String,
    model: String,
}

#[derive(Serialize)]
struct GenerateRequest {
    model: String,
    prompt: String,
    stream: bool,
}

#[derive(Deserialize)]
struct GenerateResponse {
    response: String,
}

#[derive(Deserialize)]
struct TagsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
}

impl OllamaClient {
    pub fn new() -> Self {
        Self::with_config(DEFAULT_ENDPOINT, DEFAULT_MODEL, DEFAULT_TIMEOUT)
    }

    pub fn with_config(endpoint: &str, model: &str, timeout_secs: u64) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            endpoint: endpoint.to_string(),
            model: model.to_string(),
        }
    }

    /// Check if Ollama is available
    pub async fn is_available(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await
            .is_ok()
    }

    /// List available models
    pub async fn list_models(&self) -> Result<Vec<ModelInfo>, Box<dyn std::error::Error + Send + Sync>> {
        let response: TagsResponse = self.client
            .get(format!("{}/api/tags", self.endpoint))
            .send()
            .await?
            .json()
            .await?;

        Ok(response.models)
    }

    /// Generate a response from the model
    pub async fn generate(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let request = GenerateRequest {
            model: self.model.clone(),
            prompt: prompt.to_string(),
            stream: false,
        };

        let response: GenerateResponse = self.client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.response)
    }

    /// Generate a quick response with a fast model (for process explanations)
    pub async fn generate_fast(&self, prompt: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Use a smaller, faster model for quick explanations
        let fast_model = "llama3.2:1b";

        let request = GenerateRequest {
            model: fast_model.to_string(),
            prompt: prompt.to_string(),
            stream: false,
        };

        // Create a client with shorter timeout for fast responses
        let fast_client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;

        let response: GenerateResponse = fast_client
            .post(format!("{}/api/generate", self.endpoint))
            .json(&request)
            .send()
            .await?
            .json()
            .await?;

        Ok(response.response)
    }

    /// Set the model to use
    pub fn set_model(&mut self, model: &str) {
        self.model = model.to_string();
    }

    /// Get the current model
    pub fn model(&self) -> &str {
        &self.model
    }
}
