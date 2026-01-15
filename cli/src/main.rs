use clap::{Parser, Subcommand};
use qrcode::render::unicode::Dense1x2;
use qrcode::QrCode;
use serde::{Deserialize, Serialize};
use std::io::{self, Read};
use std::thread;
use std::time::{Duration, Instant};

const APP_NAME: &str = "claude-afk";
const DEFAULT_BACKEND_URL: &str = "https://ccafk.treeleaf.dev";
const POLL_INTERVAL: Duration = Duration::from_secs(2);
const SETUP_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes

#[derive(Debug, Default, Serialize, Deserialize)]
struct Config {
    device_token: Option<String>,
    backend_url: String,
    active: bool,
}

#[derive(Parser)]
#[command(name = "claude-afk", about = "Push notifications for Claude Code")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Set up device pairing by scanning a QR code
    Setup,
    /// Send a notification (reads JSON from stdin)
    Notify,
    /// Show current configuration status
    Status,
    /// Enable notifications
    Activate,
    /// Disable notifications
    Deactivate,
    /// Clear device pairing
    Clear,
}

#[derive(Deserialize)]
struct PairingInitResponse {
    #[serde(rename = "pairingId")]
    pairing_id: String,
    #[serde(rename = "pairingToken")]
    pairing_token: String,
}

#[derive(Deserialize)]
struct PairingStatusResponse {
    complete: bool,
    #[serde(rename = "deviceToken")]
    device_token: Option<String>,
}

#[derive(Deserialize)]
struct NotifyInput {
    message: String,
    title: Option<String>,
}

#[derive(Serialize)]
struct NotifyPayload {
    title: String,
    message: String,
}

fn main() {
    let cli = Cli::parse();

    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), Box<dyn std::error::Error>> {
    match cli.command {
        Commands::Setup => cmd_setup(),
        Commands::Notify => cmd_notify(),
        Commands::Status => cmd_status(),
        Commands::Activate => cmd_activate(),
        Commands::Deactivate => cmd_deactivate(),
        Commands::Clear => cmd_clear(),
    }
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    Ok(confy::load(APP_NAME, None)?)
}

fn save_config(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    Ok(confy::store(APP_NAME, None, config)?)
}

fn get_backend_url() -> String {
    // Priority: env var (for local development) > default production URL
    std::env::var("CLAUDE_AFK_BACKEND_URL")
        .unwrap_or_else(|_| DEFAULT_BACKEND_URL.to_string())
}

fn render_qr(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let code = QrCode::new(url)?;
    let string = code
        .render::<Dense1x2>()
        .dark_color(Dense1x2::Light)
        .light_color(Dense1x2::Dark)
        .build();
    println!("{}", string);
    Ok(())
}

fn cmd_setup() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;

    let backend_url = get_backend_url();
    let backend_url = backend_url.trim_end_matches('/');

    // Initiate pairing
    println!("Initiating pairing with {}...", backend_url);
    let response: PairingInitResponse = ureq::post(&format!("{}/api/pairing/initiate", backend_url))
        .call()?
        .into_json()?;

    let pairing_url = format!("{}/pair/{}", backend_url, response.pairing_token);

    println!("\nScan this QR code with your phone to complete setup:\n");
    render_qr(&pairing_url)?;
    println!("\nOr open this URL: {}", pairing_url);
    println!("\nWaiting for pairing to complete...");

    // Poll for completion
    let start = Instant::now();
    loop {
        if start.elapsed() > SETUP_TIMEOUT {
            return Err("Pairing timed out after 5 minutes".into());
        }

        thread::sleep(POLL_INTERVAL);

        let status: PairingStatusResponse =
            ureq::get(&format!("{}/api/pairing/{}/status", backend_url, response.pairing_id))
                .call()?
                .into_json()?;

        if status.complete {
            if let Some(device_token) = status.device_token {
                config.device_token = Some(device_token);
                config.backend_url = backend_url.to_string();
                config.active = true;
                save_config(&config)?;

                println!("\nPairing successful! Notifications are now enabled.");
                return Ok(());
            } else {
                return Err("Pairing completed but no device token received".into());
            }
        }
    }
}

fn cmd_notify() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    // Silent exit if not configured or not active
    if config.device_token.is_none() || !config.active {
        return Ok(());
    }

    let device_token = config.device_token.unwrap();
    let backend_url = get_backend_url();

    // Read JSON from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let notify_input: NotifyInput = match serde_json::from_str(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Failed to parse input: {}", e);
            return Ok(()); // Silent exit, don't crash Claude Code
        }
    };

    let payload = NotifyPayload {
        title: notify_input.title.unwrap_or_else(|| "Claude Code".to_string()),
        message: notify_input.message,
    };

    // Send notification
    match ureq::post(&format!("{}/api/notify", backend_url))
        .set("Authorization", &format!("Bearer {}", device_token))
        .send_json(&payload)
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Failed to send notification: {}", e);
            // Don't return error, just log to stderr
        }
    }

    Ok(())
}

fn cmd_status() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config()?;

    let backend_url = get_backend_url();
    let device_paired = config.device_token.is_some();
    let notifications_active = config.active;

    println!("Claude AFK Status");
    println!("-----------------");
    println!("Backend URL: {}", backend_url);
    println!(
        "Device paired: {}",
        if device_paired { "yes" } else { "no" }
    );
    println!(
        "Notifications active: {}",
        if notifications_active { "yes" } else { "no" }
    );

    Ok(())
}

fn cmd_activate() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;

    if config.device_token.is_none() {
        return Err("No device paired. Run 'claude-afk setup' first.".into());
    }

    config.active = true;
    save_config(&config)?;

    println!("Notifications activated.");
    Ok(())
}

fn cmd_deactivate() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;
    config.active = false;
    save_config(&config)?;

    println!("Notifications deactivated.");
    Ok(())
}

fn cmd_clear() -> Result<(), Box<dyn std::error::Error>> {
    let mut config = load_config()?;
    config.device_token = None;
    config.active = false;
    save_config(&config)?;

    println!("Device pairing cleared.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // Mutex to ensure env var tests don't run in parallel
    static ENV_MUTEX: Mutex<()> = Mutex::new(());

    // ==================== Config Tests ====================

    #[test]
    fn test_config_default_values() {
        let config = Config::default();
        assert!(config.device_token.is_none());
        assert!(config.backend_url.is_empty());
        assert!(!config.active);
    }

    #[test]
    fn test_config_serialization_roundtrip() {
        let config = Config {
            device_token: Some("test-token-12345".to_string()),
            backend_url: "https://example.com".to_string(),
            active: true,
        };

        let serialized = toml::to_string(&config).unwrap();
        let deserialized: Config = toml::from_str(&serialized).unwrap();

        assert_eq!(deserialized.device_token, config.device_token);
        assert_eq!(deserialized.backend_url, config.backend_url);
        assert_eq!(deserialized.active, config.active);
    }

    #[test]
    fn test_config_deserialize_from_toml() {
        let toml_str = r#"
            device_token = "my-device-token"
            backend_url = "http://localhost:5173"
            active = true
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert_eq!(config.device_token, Some("my-device-token".to_string()));
        assert_eq!(config.backend_url, "http://localhost:5173");
        assert!(config.active);
    }

    #[test]
    fn test_config_deserialize_minimal_toml() {
        // Test with missing optional fields (uses defaults)
        let toml_str = r#"
            backend_url = ""
            active = false
        "#;

        let config: Config = toml::from_str(toml_str).unwrap();
        assert!(config.device_token.is_none());
        assert!(config.backend_url.is_empty());
        assert!(!config.active);
    }

    // ==================== Backend URL Tests ====================

    #[test]
    fn test_get_backend_url_returns_default() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::remove_var("CLAUDE_AFK_BACKEND_URL");

        let result = get_backend_url();
        assert_eq!(result, DEFAULT_BACKEND_URL);
    }

    #[test]
    fn test_get_backend_url_env_var_overrides_default() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("CLAUDE_AFK_BACKEND_URL", "http://localhost:5173");

        let result = get_backend_url();
        assert_eq!(result, "http://localhost:5173");

        std::env::remove_var("CLAUDE_AFK_BACKEND_URL");
    }

    // ==================== NotifyInput Parsing Tests ====================

    #[test]
    fn test_notify_input_parse_full() {
        let json = r#"{"message": "Hello world", "title": "Test Title"}"#;
        let input: NotifyInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.message, "Hello world");
        assert_eq!(input.title, Some("Test Title".to_string()));
    }

    #[test]
    fn test_notify_input_parse_message_only() {
        let json = r#"{"message": "Hello world"}"#;
        let input: NotifyInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.message, "Hello world");
        assert!(input.title.is_none());
    }

    #[test]
    fn test_notify_input_missing_message_fails() {
        let json = r#"{"title": "Test Title"}"#;
        let result: Result<NotifyInput, _> = serde_json::from_str(json);

        assert!(result.is_err());
    }

    #[test]
    fn test_notify_input_empty_message() {
        let json = r#"{"message": ""}"#;
        let input: NotifyInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.message, "");
    }

    #[test]
    fn test_notify_input_with_special_characters() {
        let json = r#"{"message": "Hello \"world\" with\nnewlines", "title": "Test 🚀"}"#;
        let input: NotifyInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.message, "Hello \"world\" with\nnewlines");
        assert_eq!(input.title, Some("Test 🚀".to_string()));
    }

    // ==================== NotifyPayload Tests ====================

    #[test]
    fn test_notify_payload_serialization() {
        let payload = NotifyPayload {
            title: "Test Title".to_string(),
            message: "Test Message".to_string(),
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"title\":\"Test Title\""));
        assert!(json.contains("\"message\":\"Test Message\""));
    }

    #[test]
    fn test_notify_payload_from_input_with_title() {
        let input = NotifyInput {
            message: "Test message".to_string(),
            title: Some("Custom Title".to_string()),
        };

        let payload = NotifyPayload {
            title: input.title.unwrap_or_else(|| "Claude Code".to_string()),
            message: input.message,
        };

        assert_eq!(payload.title, "Custom Title");
        assert_eq!(payload.message, "Test message");
    }

    #[test]
    fn test_notify_payload_from_input_without_title() {
        let input = NotifyInput {
            message: "Test message".to_string(),
            title: None,
        };

        let payload = NotifyPayload {
            title: input.title.unwrap_or_else(|| "Claude Code".to_string()),
            message: input.message,
        };

        assert_eq!(payload.title, "Claude Code");
        assert_eq!(payload.message, "Test message");
    }

    // ==================== API Response Parsing Tests ====================

    #[test]
    fn test_pairing_init_response_parse() {
        let json = r#"{"pairingId": "abc123", "pairingToken": "xyz789"}"#;
        let response: PairingInitResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.pairing_id, "abc123");
        assert_eq!(response.pairing_token, "xyz789");
    }

    #[test]
    fn test_pairing_status_response_incomplete() {
        let json = r#"{"complete": false, "deviceToken": null}"#;
        let response: PairingStatusResponse = serde_json::from_str(json).unwrap();

        assert!(!response.complete);
        assert!(response.device_token.is_none());
    }

    #[test]
    fn test_pairing_status_response_complete() {
        let json = r#"{"complete": true, "deviceToken": "device-token-12345"}"#;
        let response: PairingStatusResponse = serde_json::from_str(json).unwrap();

        assert!(response.complete);
        assert_eq!(response.device_token, Some("device-token-12345".to_string()));
    }

    // ==================== QR Code Generation Tests ====================

    #[test]
    fn test_qr_code_generation() {
        let url = "https://example.com/pair/test-token";
        let code = QrCode::new(url).unwrap();
        let rendered = code
            .render::<Dense1x2>()
            .dark_color(Dense1x2::Light)
            .light_color(Dense1x2::Dark)
            .build();

        // QR code should be non-empty and contain multiple lines
        assert!(!rendered.is_empty());
        assert!(rendered.contains('\n'));
    }

    #[test]
    fn test_qr_code_with_long_url() {
        let url = "https://example.com/pair/very-long-token-that-is-32-chars";
        let result = QrCode::new(url);
        assert!(result.is_ok());
    }

    // ==================== URL Construction Tests ====================

    #[test]
    fn test_pairing_url_construction() {
        let backend_url = "https://example.com";
        let pairing_token = "abc123xyz";
        let pairing_url = format!("{}/pair/{}", backend_url, pairing_token);

        assert_eq!(pairing_url, "https://example.com/pair/abc123xyz");
    }

    #[test]
    fn test_api_endpoint_construction() {
        let backend_url = "https://example.com";

        let initiate_url = format!("{}/api/pairing/initiate", backend_url);
        assert_eq!(initiate_url, "https://example.com/api/pairing/initiate");

        let pairing_id = "session-123";
        let status_url = format!("{}/api/pairing/{}/status", backend_url, pairing_id);
        assert_eq!(status_url, "https://example.com/api/pairing/session-123/status");

        let notify_url = format!("{}/api/notify", backend_url);
        assert_eq!(notify_url, "https://example.com/api/notify");
    }

    // ==================== Config State Logic Tests ====================

    #[test]
    fn test_should_notify_when_configured_and_active() {
        let config = Config {
            device_token: Some("token".to_string()),
            backend_url: "http://example.com".to_string(),
            active: true,
        };

        let should_notify = config.device_token.is_some() && config.active;
        assert!(should_notify);
    }

    #[test]
    fn test_should_not_notify_when_no_token() {
        let config = Config {
            device_token: None,
            backend_url: "http://example.com".to_string(),
            active: true,
        };

        let should_notify = config.device_token.is_some() && config.active;
        assert!(!should_notify);
    }

    #[test]
    fn test_should_not_notify_when_inactive() {
        let config = Config {
            device_token: Some("token".to_string()),
            backend_url: "http://example.com".to_string(),
            active: false,
        };

        let should_notify = config.device_token.is_some() && config.active;
        assert!(!should_notify);
    }


    // ==================== Authorization Header Tests ====================

    #[test]
    fn test_authorization_header_format() {
        let device_token = "my-secret-token";
        let header = format!("Bearer {}", device_token);

        assert_eq!(header, "Bearer my-secret-token");
        assert!(header.starts_with("Bearer "));
    }
}
