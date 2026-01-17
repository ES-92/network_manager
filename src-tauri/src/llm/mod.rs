// Ollama LLM integration modules

pub mod client;
pub mod analyzer;

pub use client::OllamaClient;
pub use analyzer::{LogAnalyzer, ServiceRecommendation, RecommendationType};
