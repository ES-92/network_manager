use crate::llm::{OllamaClient, LogAnalyzer, ServiceRecommendation};
use crate::llm::analyzer::AnalysisType;
use std::sync::OnceLock;
use tokio::sync::RwLock;

static OLLAMA_CLIENT: OnceLock<RwLock<OllamaClient>> = OnceLock::new();

fn get_client() -> &'static RwLock<OllamaClient> {
    OLLAMA_CLIENT.get_or_init(|| RwLock::new(OllamaClient::new()))
}

/// Common process explanations cache for known processes - comprehensive list
fn get_known_process_explanation(name: &str) -> Option<String> {
    let name_lower = name.to_lowercase();

    // ===== Adobe =====
    if name_lower.contains("adobe") {
        if name_lower.contains("arm") || name_lower.contains("armdchelper") {
            return Some("Adobe ARM (Application Resource Manager) - Verwaltet automatische Updates für Adobe-Programme. Sicher, aber optional wenn Sie keine Adobe-Produkte nutzen.".to_string());
        }
        if name_lower.contains("cef") || name_lower.contains("cefhelper") {
            return Some("Adobe CEF Helper - Chromium-basierter Prozess für Web-Inhalte in Adobe Creative Cloud. Sicher, optional bei Nichtnutzung.".to_string());
        }
        if name_lower.contains("creative") || name_lower.contains("cc") {
            return Some("Adobe Creative Cloud - Verwaltungs-App für Adobe-Produkte wie Photoshop, Illustrator etc.".to_string());
        }
        if name_lower.contains("ipc") {
            return Some("Adobe IPC Broker - Kommunikationsprozess zwischen Adobe-Anwendungen. Sicher.".to_string());
        }
        return Some("Adobe-Prozess - Gehört zur Adobe-Software-Suite. In der Regel sicher.".to_string());
    }

    // ===== Apple macOS System =====
    if name_lower.contains("windowserver") {
        return Some("WindowServer - Essentieller macOS-Prozess für die grafische Oberfläche. NIEMALS beenden!".to_string());
    }
    if name_lower.contains("kernel_task") || name_lower == "kernel" {
        return Some("kernel_task - Kern des macOS-Betriebssystems. Verwaltet Hardware und Ressourcen. Essentiell.".to_string());
    }
    if name_lower.contains("spotlight") || name_lower.contains("mds") || name_lower.contains("mdworker") {
        return Some("Spotlight - macOS Suchindizierung. Durchsucht und indiziert Dateien für schnelle Suche. Sicher.".to_string());
    }
    if name_lower.contains("launchd") {
        return Some("launchd - Zentraler macOS-Prozessmanager. Startet und verwaltet alle Dienste. Essentiell.".to_string());
    }
    if name_lower.contains("loginwindow") {
        return Some("loginwindow - macOS Anmeldeprozess und Benutzersitzungsverwaltung. Essentiell.".to_string());
    }
    if name_lower.contains("finder") {
        return Some("Finder - macOS Dateimanager und Desktop-Verwaltung. Standard-App von Apple.".to_string());
    }
    if name_lower.contains("dock") && !name_lower.contains("docker") {
        return Some("Dock - macOS Anwendungsleiste am unteren Bildschirmrand. System-App von Apple.".to_string());
    }
    if name_lower.contains("systemuiserver") {
        return Some("SystemUIServer - Verwaltet die macOS-Menüleiste und Systemsymbole. Essentiell.".to_string());
    }
    if name_lower.contains("coreaudio") {
        return Some("CoreAudio - macOS Audiosystem. Verwaltet alle Audioein- und -ausgaben. Essentiell.".to_string());
    }
    if name_lower.contains("airplay") {
        return Some("AirPlay - Apple-Dienst für drahtloses Streaming zu Apple TV und kompatiblen Geräten.".to_string());
    }
    if name_lower.contains("bluetooth") {
        return Some("Bluetooth-Dienst - Verwaltet Bluetooth-Verbindungen zu Geräten wie Kopfhörern, Tastaturen etc.".to_string());
    }
    if name_lower.contains("wifi") || name_lower.contains("wlan") {
        return Some("WLAN/WiFi-Dienst - Verwaltet drahtlose Netzwerkverbindungen. Essentiell für Internet.".to_string());
    }
    if name_lower.contains("cfprefsd") {
        return Some("cfprefsd - macOS Einstellungs-Daemon. Verwaltet App-Einstellungen und Preferences. Essentiell.".to_string());
    }
    if name_lower.contains("distnoted") {
        return Some("distnoted - Distributed Notification Server. Verwaltet System-Benachrichtigungen zwischen Apps.".to_string());
    }
    if name_lower.contains("notificationcenter") || name_lower.contains("usernoted") {
        return Some("Notification Center - macOS Benachrichtigungszentrale für App-Mitteilungen.".to_string());
    }
    if name_lower.contains("coreservices") {
        return Some("CoreServices - Zentrale macOS-Systemdienste. Verschiedene Hintergrundprozesse.".to_string());
    }
    if name_lower.contains("imagent") || name_lower.contains("imessage") {
        return Some("iMessage-Dienst - Apple Nachrichtendienst für iMessage und SMS-Weiterleitung.".to_string());
    }
    if name_lower.contains("facetime") {
        return Some("FaceTime - Apple Video- und Audioanrufdienst.".to_string());
    }
    if name_lower.contains("icloud") || name_lower.contains("bird") {
        return Some("iCloud-Dienst - Synchronisiert Dateien, Fotos und Daten mit Apple iCloud.".to_string());
    }
    if name_lower.contains("photoanalysis") || name_lower.contains("photolibrary") {
        return Some("Fotos-Analyse - Analysiert Bilder für Gesichtserkennung und intelligente Alben.".to_string());
    }
    if name_lower.contains("backupd") || name_lower.contains("timemachine") {
        return Some("Time Machine - macOS Backup-System. Erstellt automatische Sicherungen.".to_string());
    }
    if name_lower.contains("softwareupdate") {
        return Some("Software Update - macOS Aktualisierungsdienst für System- und App-Updates.".to_string());
    }
    if name_lower.contains("siri") || name_lower.contains("assistant") {
        return Some("Siri - Apple Sprachassistent. Verarbeitet Sprachbefehle und -anfragen.".to_string());
    }
    if name_lower.contains("securityd") || name_lower.contains("trustd") {
        return Some("Sicherheitsdienst - Verwaltet Zertifikate, Schlüsselbund und Sicherheitsrichtlinien. Essentiell.".to_string());
    }
    if name_lower.contains("opendirectory") || name_lower.contains("dscacheutil") {
        return Some("Directory Service - Verwaltet Benutzer, Gruppen und Netzwerkverzeichnisse.".to_string());
    }

    // ===== Browsers =====
    if name_lower.contains("chrome") {
        if name_lower.contains("helper") {
            return Some("Chrome Helper - Unterprozess von Google Chrome für Tabs, Erweiterungen und Plugins. Isoliert für Sicherheit.".to_string());
        }
        return Some("Google Chrome - Webbrowser von Google. Verwendet mehrere Prozesse für Stabilität.".to_string());
    }
    if name_lower.contains("firefox") {
        return Some("Mozilla Firefox - Open-Source Webbrowser. Verwendet Multiprozess-Architektur.".to_string());
    }
    if name_lower.contains("safari") {
        if name_lower.contains("networking") {
            return Some("Safari Networking - Netzwerk-Prozess für Safari-Webbrowser.".to_string());
        }
        return Some("Safari - Apple Webbrowser. Standard-Browser auf macOS und iOS.".to_string());
    }
    if name_lower.contains("edge") {
        return Some("Microsoft Edge - Chromium-basierter Webbrowser von Microsoft.".to_string());
    }
    if name_lower.contains("brave") {
        return Some("Brave Browser - Datenschutzorientierter Webbrowser mit integriertem Werbeblocker.".to_string());
    }
    if name_lower.contains("opera") {
        return Some("Opera - Webbrowser mit integriertem VPN und Werbeblocker.".to_string());
    }
    if name_lower.contains("webkit") {
        return Some("WebKit - Browser-Engine für Safari und andere Apps. Rendert Webseiten.".to_string());
    }

    // ===== Development Tools =====
    if name_lower.contains("docker") {
        return Some("Docker - Container-Virtualisierung. Führt Anwendungen in isolierten Containern aus.".to_string());
    }
    if name_lower.contains("node") && !name_lower.contains("notification") {
        return Some("Node.js - JavaScript-Laufzeitumgebung für Webentwicklung und Server.".to_string());
    }
    if name_lower.contains("code") && (name_lower.contains("helper") || name_lower.contains("visual")) {
        return Some("Visual Studio Code - Microsoft Code-Editor. Helper-Prozesse für Erweiterungen.".to_string());
    }
    if name_lower.contains("xcode") {
        return Some("Xcode - Apple Entwicklungsumgebung für macOS, iOS und andere Apple-Plattformen.".to_string());
    }
    if name_lower.contains("simulator") {
        return Some("iOS Simulator - Emuliert iPhone/iPad für App-Entwicklung und Tests.".to_string());
    }
    if name_lower.contains("git") {
        return Some("Git - Versionskontrollsystem für Softwareentwicklung.".to_string());
    }
    if name_lower.contains("npm") {
        return Some("npm - Node Package Manager. Verwaltet JavaScript-Pakete und Abhängigkeiten.".to_string());
    }
    if name_lower.contains("yarn") {
        return Some("Yarn - Alternativer JavaScript-Paketmanager, oft schneller als npm.".to_string());
    }
    if name_lower.contains("python") {
        return Some("Python - Programmiersprache. Weit verbreitet für Scripting, KI und Webentwicklung.".to_string());
    }
    if name_lower.contains("ruby") {
        return Some("Ruby - Programmiersprache. Bekannt für Ruby on Rails Webframework.".to_string());
    }
    if name_lower.contains("java") && !name_lower.contains("javascript") {
        return Some("Java - Programmiersprache und Laufzeitumgebung. Weit verbreitet in Unternehmen.".to_string());
    }
    if name_lower.contains("rust") {
        return Some("Rust - Systemprogrammiersprache. Bekannt für Sicherheit und Performance.".to_string());
    }
    if name_lower.contains("cargo") {
        return Some("Cargo - Rust Paketmanager und Build-System.".to_string());
    }
    if name_lower.contains("go") && name_lower.len() < 10 {
        return Some("Go/Golang - Programmiersprache von Google. Bekannt für Einfachheit und Performance.".to_string());
    }
    if name_lower.contains("postgres") || name_lower.contains("psql") {
        return Some("PostgreSQL - Leistungsstarke Open-Source Datenbank.".to_string());
    }
    if name_lower.contains("mysql") {
        return Some("MySQL - Populäre relationale Datenbank.".to_string());
    }
    if name_lower.contains("redis") {
        return Some("Redis - In-Memory Datenbank für Caching und Nachrichtenwarteschlangen.".to_string());
    }
    if name_lower.contains("mongo") {
        return Some("MongoDB - NoSQL-Dokumentendatenbank.".to_string());
    }
    if name_lower.contains("ollama") {
        return Some("Ollama - Lokale KI/LLM-Laufzeitumgebung. Führt Sprachmodelle auf Ihrem Computer aus.".to_string());
    }
    if name_lower.contains("tauri") {
        return Some("Tauri - Framework für Desktop-Anwendungen mit Webtechnologien.".to_string());
    }
    if name_lower.contains("electron") {
        return Some("Electron - Framework für Desktop-Apps (z.B. VS Code, Slack, Discord).".to_string());
    }
    if name_lower.contains("jetbrains") || name_lower.contains("intellij") || name_lower.contains("pycharm") || name_lower.contains("webstorm") {
        return Some("JetBrains IDE - Professionelle Entwicklungsumgebung für verschiedene Programmiersprachen.".to_string());
    }

    // ===== Communication =====
    if name_lower.contains("slack") {
        return Some("Slack - Team-Kommunikationsplattform für Unternehmen.".to_string());
    }
    if name_lower.contains("discord") {
        return Some("Discord - Voice-, Video- und Text-Chat-Plattform.".to_string());
    }
    if name_lower.contains("zoom") {
        return Some("Zoom - Videokonferenz-Software für Meetings und Webinare.".to_string());
    }
    if name_lower.contains("teams") {
        return Some("Microsoft Teams - Kommunikationsplattform für Unternehmen.".to_string());
    }
    if name_lower.contains("telegram") {
        return Some("Telegram - Cloud-basierter Messenger mit Fokus auf Geschwindigkeit und Sicherheit.".to_string());
    }
    if name_lower.contains("whatsapp") {
        return Some("WhatsApp - Messenger von Meta für Text, Sprach- und Videoanrufe.".to_string());
    }
    if name_lower.contains("signal") {
        return Some("Signal - Sicherer Messenger mit Ende-zu-Ende-Verschlüsselung.".to_string());
    }
    if name_lower.contains("skype") {
        return Some("Skype - Video- und Sprachanrufdienst von Microsoft.".to_string());
    }

    // ===== Productivity =====
    if name_lower.contains("spotify") {
        return Some("Spotify - Musik-Streaming-Dienst.".to_string());
    }
    if name_lower.contains("dropbox") {
        return Some("Dropbox - Cloud-Speicher und Dateisynchronisation.".to_string());
    }
    if name_lower.contains("onedrive") {
        return Some("OneDrive - Microsoft Cloud-Speicher, integriert in Windows und Office.".to_string());
    }
    if name_lower.contains("notion") {
        return Some("Notion - All-in-One Workspace für Notizen, Dokumente und Projektmanagement.".to_string());
    }
    if name_lower.contains("obsidian") {
        return Some("Obsidian - Wissensmanagement-App mit Markdown-Notizen und Verknüpfungen.".to_string());
    }
    if name_lower.contains("1password") || name_lower.contains("onepassword") {
        return Some("1Password - Passwort-Manager für sichere Speicherung von Zugangsdaten.".to_string());
    }
    if name_lower.contains("bitwarden") {
        return Some("Bitwarden - Open-Source Passwort-Manager.".to_string());
    }
    if name_lower.contains("lastpass") {
        return Some("LastPass - Cloud-basierter Passwort-Manager.".to_string());
    }

    // ===== Security & VPN =====
    if name_lower.contains("vpn") {
        return Some("VPN-Client - Stellt sichere, verschlüsselte Netzwerkverbindungen her.".to_string());
    }
    if name_lower.contains("wireguard") {
        return Some("WireGuard - Modernes, schnelles VPN-Protokoll.".to_string());
    }
    if name_lower.contains("openvpn") {
        return Some("OpenVPN - Open-Source VPN-Lösung.".to_string());
    }
    if name_lower.contains("antivir") || name_lower.contains("avast") || name_lower.contains("norton") || name_lower.contains("kaspersky") || name_lower.contains("malware") {
        return Some("Antivirus/Sicherheitssoftware - Schützt vor Malware und Bedrohungen.".to_string());
    }
    if name_lower.contains("littlesnitch") {
        return Some("Little Snitch - macOS Firewall zur Kontrolle ausgehender Verbindungen.".to_string());
    }

    // ===== Media =====
    if name_lower.contains("vlc") {
        return Some("VLC - Open-Source Mediaplayer für fast alle Audio- und Videoformate.".to_string());
    }
    if name_lower.contains("quicktime") {
        return Some("QuickTime - Apple Mediaplayer und Framework.".to_string());
    }
    if name_lower.contains("handbrake") {
        return Some("HandBrake - Open-Source Videokonverter.".to_string());
    }
    if name_lower.contains("obs") {
        return Some("OBS Studio - Open-Source Software für Streaming und Aufnahme.".to_string());
    }

    // ===== System Utilities =====
    if name_lower.contains("alfred") {
        return Some("Alfred - Produktivitäts-App für macOS mit Spotlight-Alternative und Workflows.".to_string());
    }
    if name_lower.contains("raycast") {
        return Some("Raycast - Produktivitäts-Tool und Launcher für macOS.".to_string());
    }
    if name_lower.contains("rectangle") || name_lower.contains("magnet") {
        return Some("Fenster-Manager - Organisiert Fenster auf dem Desktop mit Tastenkombinationen.".to_string());
    }
    if name_lower.contains("bartender") {
        return Some("Bartender - Organisiert und versteckt Menüleistensymbole auf macOS.".to_string());
    }
    if name_lower.contains("cleanmymac") || name_lower.contains("ccleaner") {
        return Some("System-Cleaner - Bereinigt temporäre Dateien und Cache.".to_string());
    }

    // ===== Generic patterns =====
    if name_lower.contains("helper") {
        return Some("Helper-Prozess - Unterprozess einer Anwendung für spezielle Aufgaben.".to_string());
    }
    if name_lower.contains("agent") {
        return Some("Agent-Prozess - Hintergrundprozess einer Anwendung oder des Systems.".to_string());
    }
    if name_lower.contains("daemon") || name_lower.ends_with("d") && name_lower.len() < 15 {
        return Some("Daemon - Hintergrunddienst, der ohne Benutzerinteraktion läuft.".to_string());
    }
    if name_lower.contains("service") {
        return Some("Systemdienst - Hintergrundprozess für bestimmte Funktionen.".to_string());
    }
    if name_lower.contains("updater") || name_lower.contains("update") {
        return Some("Update-Dienst - Prüft und installiert Software-Aktualisierungen.".to_string());
    }
    if name_lower.starts_with("com.apple.") {
        return Some("Apple-Systemdienst - Interner macOS-Prozess. In der Regel essentiell.".to_string());
    }

    None
}

#[tauri::command]
pub async fn check_ollama_status() -> Result<bool, String> {
    let client = get_client().read().await;
    Ok(client.is_available().await)
}

#[tauri::command]
pub async fn list_ollama_models() -> Result<Vec<String>, String> {
    let client = get_client().read().await;
    let models = client.list_models().await.map_err(|e| e.to_string())?;
    Ok(models.into_iter().map(|m| m.name).collect())
}

#[tauri::command]
pub async fn analyze_logs(logs: String, analysis_type: String) -> Result<String, String> {
    let client = get_client().read().await;
    let analyzer = LogAnalyzer::new(client.clone());

    let analysis = match analysis_type.as_str() {
        "errors" => AnalysisType::ErrorDetection,
        "patterns" => AnalysisType::PatternAnalysis,
        "anomalies" => AnalysisType::AnomalyDetection,
        "performance" => AnalysisType::PerformanceAnalysis,
        "security" => AnalysisType::SecurityAnalysis,
        _ => return Err(format!("Unknown analysis type: {}", analysis_type)),
    };

    analyzer.analyze(&logs, analysis).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_ollama_model(model: String) -> Result<(), String> {
    let mut client = get_client().write().await;
    client.set_model(&model);
    Ok(())
}

#[tauri::command]
pub async fn explain_process(
    process_name: String,
    process_path: Option<String>,
    description: Option<String>,
) -> Result<String, String> {
    // First check if we have a cached explanation for known processes
    if let Some(explanation) = get_known_process_explanation(&process_name) {
        return Ok(explanation);
    }

    // Fall back to LLM for unknown processes
    let client = get_client().read().await;
    if !client.is_available().await {
        return Err("Ollama ist nicht verfügbar. Bitte starten Sie Ollama, um Prozess-Erklärungen zu erhalten.".to_string());
    }

    let analyzer = LogAnalyzer::new(client.clone());
    analyzer
        .explain_process(
            &process_name,
            process_path.as_deref(),
            description.as_deref(),
        )
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_service_recommendations(services_json: String) -> Result<Vec<ServiceRecommendation>, String> {
    let client = get_client().read().await;

    if !client.is_available().await {
        // Return mock recommendations when Ollama is not available
        return Ok(get_default_recommendations());
    }

    let analyzer = LogAnalyzer::new(client.clone());
    let response = analyzer
        .generate_recommendations(&services_json)
        .await
        .map_err(|e| e.to_string())?;

    // Try to parse the JSON response
    // First, try to extract JSON from the response (LLM might add extra text)
    let json_str = extract_json_array(&response).unwrap_or(&response);

    match serde_json::from_str::<Vec<ServiceRecommendation>>(json_str) {
        Ok(recommendations) => Ok(recommendations),
        Err(_) => {
            // If parsing fails, return default recommendations
            Ok(get_default_recommendations())
        }
    }
}

fn extract_json_array(text: &str) -> Option<&str> {
    let start = text.find('[')?;
    let end = text.rfind(']')?;
    if start < end {
        Some(&text[start..=end])
    } else {
        None
    }
}

fn get_default_recommendations() -> Vec<ServiceRecommendation> {
    vec![
        ServiceRecommendation {
            service_id: "tip".to_string(),
            service_name: "Ollama".to_string(),
            recommendation_type: crate::llm::RecommendationType::Info,
            title: "KI-Empfehlungen aktivieren".to_string(),
            description: "Starten Sie Ollama für intelligente, personalisierte Empfehlungen basierend auf Ihren laufenden Services.".to_string(),
            action: Some("ollama serve".to_string()),
        },
    ]
}
