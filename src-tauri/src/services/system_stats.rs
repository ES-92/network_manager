use serde::{Deserialize, Serialize};
use std::process::Command;
use sysinfo::{System, CpuRefreshKind, MemoryRefreshKind, RefreshKind};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CpuStats {
    pub usage_percent: f32,
    pub core_count: usize,
    pub per_core_usage: Vec<f32>,
    pub frequency_mhz: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryStats {
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub available_bytes: u64,
    pub usage_percent: f32,
    pub swap_total_bytes: u64,
    pub swap_used_bytes: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpuStats {
    pub name: String,
    pub usage_percent: Option<f32>,
    pub memory_used_bytes: Option<u64>,
    pub memory_total_bytes: Option<u64>,
    pub temperature_celsius: Option<f32>,
    pub power_watts: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemStats {
    pub cpu: CpuStats,
    pub memory: MemoryStats,
    pub gpus: Vec<GpuStats>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum GpuProvider {
    Auto,
    Apple,
    Nvidia,
    Amd,
    None,
}

pub struct SystemMonitor {
    system: System,
    gpu_provider: GpuProvider,
}

impl SystemMonitor {
    pub fn new() -> Self {
        let system = System::new_with_specifics(
            RefreshKind::nothing()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(MemoryRefreshKind::everything())
        );

        // Auto-detect GPU provider
        let gpu_provider = Self::detect_gpu_provider();

        Self { system, gpu_provider }
    }

    pub fn with_gpu_provider(mut self, provider: GpuProvider) -> Self {
        self.gpu_provider = if provider == GpuProvider::Auto {
            Self::detect_gpu_provider()
        } else {
            provider
        };
        self
    }

    fn detect_gpu_provider() -> GpuProvider {
        #[cfg(target_os = "macos")]
        {
            // Check for Apple Silicon
            if cfg!(target_arch = "aarch64") {
                return GpuProvider::Apple;
            }
        }

        // Check for NVIDIA
        if Command::new("nvidia-smi").arg("--version").output().is_ok() {
            return GpuProvider::Nvidia;
        }

        // Check for AMD (Linux)
        #[cfg(target_os = "linux")]
        {
            if std::path::Path::new("/opt/rocm/bin/rocm-smi").exists() {
                return GpuProvider::Amd;
            }
        }

        GpuProvider::None
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu_all();
        self.system.refresh_memory();
    }

    pub fn get_stats(&mut self) -> SystemStats {
        self.refresh();

        // Small delay to get accurate CPU readings
        std::thread::sleep(std::time::Duration::from_millis(100));
        self.system.refresh_cpu_all();

        let cpu = self.get_cpu_stats();
        let memory = self.get_memory_stats();
        let gpus = self.get_gpu_stats();

        SystemStats {
            cpu,
            memory,
            gpus,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
        }
    }

    fn get_cpu_stats(&self) -> CpuStats {
        let cpus = self.system.cpus();
        let per_core_usage: Vec<f32> = cpus.iter().map(|cpu| cpu.cpu_usage()).collect();
        let avg_usage = if per_core_usage.is_empty() {
            0.0
        } else {
            per_core_usage.iter().sum::<f32>() / per_core_usage.len() as f32
        };

        CpuStats {
            usage_percent: avg_usage,
            core_count: cpus.len(),
            per_core_usage,
            frequency_mhz: cpus.first().map(|c| c.frequency()),
        }
    }

    fn get_memory_stats(&self) -> MemoryStats {
        let total = self.system.total_memory();
        let used = self.system.used_memory();
        let available = self.system.available_memory();
        let usage_percent = if total > 0 {
            (used as f64 / total as f64 * 100.0) as f32
        } else {
            0.0
        };

        MemoryStats {
            total_bytes: total,
            used_bytes: used,
            available_bytes: available,
            usage_percent,
            swap_total_bytes: self.system.total_swap(),
            swap_used_bytes: self.system.used_swap(),
        }
    }

    fn get_gpu_stats(&self) -> Vec<GpuStats> {
        match self.gpu_provider {
            GpuProvider::Apple => self.get_apple_gpu_stats(),
            GpuProvider::Nvidia => self.get_nvidia_gpu_stats(),
            GpuProvider::Amd => self.get_amd_gpu_stats(),
            GpuProvider::None | GpuProvider::Auto => vec![],
        }
    }

    #[cfg(target_os = "macos")]
    fn get_apple_gpu_stats(&self) -> Vec<GpuStats> {
        // Use powermetrics or ioreg for Apple Silicon GPU stats
        // This requires sudo for detailed stats, so we provide basic info
        let output = Command::new("system_profiler")
            .args(["SPDisplaysDataType", "-json"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // Parse JSON to get GPU name
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    if let Some(displays) = json.get("SPDisplaysDataType").and_then(|v| v.as_array()) {
                        return displays.iter().filter_map(|gpu| {
                            let name = gpu.get("sppci_model")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Apple GPU")
                                .to_string();

                            Some(GpuStats {
                                name,
                                usage_percent: None, // Requires sudo for powermetrics
                                memory_used_bytes: None,
                                memory_total_bytes: None,
                                temperature_celsius: None,
                                power_watts: None,
                            })
                        }).collect();
                    }
                }
                vec![GpuStats {
                    name: "Apple Silicon GPU".to_string(),
                    usage_percent: None,
                    memory_used_bytes: None,
                    memory_total_bytes: None,
                    temperature_celsius: None,
                    power_watts: None,
                }]
            }
            _ => vec![],
        }
    }

    #[cfg(not(target_os = "macos"))]
    fn get_apple_gpu_stats(&self) -> Vec<GpuStats> {
        vec![]
    }

    fn get_nvidia_gpu_stats(&self) -> Vec<GpuStats> {
        let output = Command::new("nvidia-smi")
            .args([
                "--query-gpu=name,utilization.gpu,memory.used,memory.total,temperature.gpu,power.draw",
                "--format=csv,noheader,nounits"
            ])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                stdout.lines().filter_map(|line| {
                    let parts: Vec<&str> = line.split(',').map(|s| s.trim()).collect();
                    if parts.len() >= 6 {
                        Some(GpuStats {
                            name: parts[0].to_string(),
                            usage_percent: parts[1].parse().ok(),
                            memory_used_bytes: parts[2].parse::<u64>().ok().map(|m| m * 1024 * 1024),
                            memory_total_bytes: parts[3].parse::<u64>().ok().map(|m| m * 1024 * 1024),
                            temperature_celsius: parts[4].parse().ok(),
                            power_watts: parts[5].parse().ok(),
                        })
                    } else {
                        None
                    }
                }).collect()
            }
            _ => vec![],
        }
    }

    fn get_amd_gpu_stats(&self) -> Vec<GpuStats> {
        // Try rocm-smi for AMD GPUs
        let output = Command::new("rocm-smi")
            .args(["--showuse", "--showmeminfo", "vram", "--showtemp", "--json"])
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                    // Parse AMD GPU info from JSON
                    let mut gpus = vec![];
                    if let Some(cards) = json.as_object() {
                        for (name, info) in cards {
                            if name.starts_with("card") {
                                let usage = info.get("GPU use (%)")
                                    .and_then(|v| v.as_str())
                                    .and_then(|s| s.parse().ok());
                                let temp = info.get("Temperature (Sensor edge) (C)")
                                    .and_then(|v| v.as_str())
                                    .and_then(|s| s.parse().ok());

                                gpus.push(GpuStats {
                                    name: format!("AMD GPU {}", name),
                                    usage_percent: usage,
                                    memory_used_bytes: None,
                                    memory_total_bytes: None,
                                    temperature_celsius: temp,
                                    power_watts: None,
                                });
                            }
                        }
                    }
                    return gpus;
                }
                vec![]
            }
            _ => vec![],
        }
    }
}

impl Default for SystemMonitor {
    fn default() -> Self {
        Self::new()
    }
}
