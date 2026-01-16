use std::{io::Read, thread, time::Instant};

use qrcode::{render::unicode::Dense1x2, QrCode};

use crate::{
    config::Config,
    constants::{
        DECISION_POLL_INTERVAL, DECISION_TIMEOUT, DEFAULT_BACKEND_URL, POLL_INTERVAL, SETUP_TIMEOUT,
    },
    logger::Logger,
    models::{
        DecisionStatusResponse, HookOutput, NotifyPayload, NotifyResponse, PairingInitResponse,
        PairingStatusResponse, PreToolUseInput, ToolInfo,
    },
};

pub struct Cmd;

impl Cmd {
    pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        let backend_url = Self::get_backend_url();
        let backend_url = backend_url.trim_end_matches('/');

        // Initiate pairing
        println!("Initiating pairing with {}...", backend_url);
        let response: PairingInitResponse =
            ureq::post(&format!("{}/api/pairing/initiate", backend_url))
                .call()?
                .into_json()?;

        let pairing_url = format!("{}/pair/{}", backend_url, response.pairing_token);

        println!("\nScan this QR code with your phone to complete setup:\n");
        Self::render_qr(&pairing_url)?;
        println!("\nOr open this URL: {}", pairing_url);
        println!("\nWaiting for pairing to complete...");

        // Poll for completion
        let start = Instant::now();
        loop {
            if start.elapsed() > SETUP_TIMEOUT {
                return Err("Pairing timed out after 5 minutes".into());
            }

            thread::sleep(POLL_INTERVAL);

            let status: PairingStatusResponse = ureq::get(&format!(
                "{}/api/pairing/{}/status",
                backend_url, response.pairing_id
            ))
            .call()?
            .into_json()?;

            if status.complete {
                if let Some(device_token) = status.device_token {
                    config.device_token = Some(device_token);
                    config.backend_url = backend_url.to_string();
                    config.active = true;
                    Config::save(&config)?;

                    println!("\nPairing successful! Notifications are now enabled.");
                    return Ok(());
                } else {
                    return Err("Pairing completed but no device token received".into());
                }
            }
        }
    }

    pub fn notify(json_arg: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        // If not configured or not active, fall back to asking user normally
        if config.device_token.is_none() || !config.active {
            let output = HookOutput::ask(None);
            println!("{}", serde_json::to_string(&output)?);
            return Ok(());
        }

        let device_token = config.device_token.unwrap();
        let backend_url = Self::get_backend_url();

        // Use JSON from argument if provided, otherwise read from stdin
        let input = match json_arg {
            Some(json) => json,
            None => {
                let mut buf = String::new();
                std::io::stdin().read_to_string(&mut buf)?;
                buf
            }
        };

        let pre_tool_use: PreToolUseInput = match serde_json::from_str(&input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("Failed to parse PreToolUse input: {}", e);
                // Fall back to normal permission flow
                let output = HookOutput::ask(None);
                println!("{}", serde_json::to_string(&output)?);
                return Ok(());
            }
        };

        // Parse tool-specific information
        let tool_info = ToolInfo::from_pre_tool_use(&pre_tool_use);
        let (title, message) = tool_info.format_for_notification();

        let payload = NotifyPayload {
            title,
            message,
            tool_use_id: pre_tool_use.tool_use_id.clone(),
            session_id: pre_tool_use.session_id.clone(),
        };

        // Send notification to backend and get decision ID
        let notify_response: NotifyResponse =
            match ureq::post(&format!("{}/api/notify", backend_url))
                .set("Authorization", &format!("Bearer {}", device_token))
                .send_json(&payload)
            {
                Ok(resp) => match resp.into_json() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to parse notify response: {}", e);
                        let output = HookOutput::ask(None);
                        println!("{}", serde_json::to_string(&output)?);
                        return Ok(());
                    }
                },
                Err(e) => {
                    eprintln!("Failed to send notification: {}", e);
                    let output = HookOutput::ask(None);
                    println!("{}", serde_json::to_string(&output)?);
                    return Ok(());
                }
            };

        // Poll for decision
        let decision_id = notify_response.decision_id;
        let start = Instant::now();

        loop {
            if start.elapsed() > DECISION_TIMEOUT {
                // Timeout - fall back to ask
                let output = HookOutput::ask(Some("Decision timed out".to_string()));
                println!("{}", serde_json::to_string(&output)?);
                return Ok(());
            }

            thread::sleep(DECISION_POLL_INTERVAL);

            let status_response: DecisionStatusResponse = match ureq::get(&format!(
                "{}/api/decision/{}/status",
                backend_url, decision_id
            ))
            .set("Authorization", &format!("Bearer {}", device_token))
            .call()
            {
                Ok(resp) => match resp.into_json() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("Failed to parse decision status: {}", e);
                        continue;
                    }
                },
                Err(e) => {
                    eprintln!("Failed to poll decision status: {}", e);
                    continue;
                }
            };

            match status_response.status.as_str() {
                "decided" => {
                    match status_response.decision.as_deref() {
                        Some("allow") => {
                            let output = HookOutput::allow(Some("Approved via phone".to_string()));
                            println!("{}", serde_json::to_string(&output)?);
                            return Ok(());
                        }
                        _ => {
                            // Dismissed or unknown decision - fall back to ask
                            let output = HookOutput::ask(Some("Dismissed on phone".to_string()));
                            println!("{}", serde_json::to_string(&output)?);
                            return Ok(());
                        }
                    }
                }
                "expired" => {
                    let output = HookOutput::ask(Some("Decision expired".to_string()));
                    println!("{}", serde_json::to_string(&output)?);
                    return Ok(());
                }
                "pending" => {
                    // Continue polling
                }
                _ => {
                    // Unknown status - continue polling
                }
            }
        }
    }

    pub fn status() -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        let backend_url = Self::get_backend_url();
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

    pub fn activate() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        if config.device_token.is_none() {
            return Err("No device paired. Run 'claude-afk setup' first.".into());
        }

        config.active = true;
        Config::save(&config)?;

        println!("Notifications activated.");
        Ok(())
    }

    pub fn deactivate() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;
        config.active = false;
        Config::save(&config)?;

        println!("Notifications deactivated.");
        Ok(())
    }

    pub fn clear() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;
        config.device_token = None;
        config.active = false;
        Config::save(&config)?;

        println!("Device pairing cleared.");
        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn clear_logs() -> Result<(), Box<dyn std::error::Error>> {
        Logger::clear_logs()?;
        println!("Debug logs cleared.");
        Ok(())
    }

    fn get_backend_url() -> String {
        // Priority: env var (for local development) > default production URL
        std::env::var("CLAUDE_AFK_BACKEND_URL").unwrap_or_else(|_| DEFAULT_BACKEND_URL.to_string())
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
}

#[cfg(test)]
mod tests {
    use crate::cmd::Cmd;
    use crate::config::Config;
    use crate::models::{HookOutput, NotifyPayload, PreToolUseInput, ToolInfo};

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

        let result = Cmd::get_backend_url();
        assert_eq!(result, DEFAULT_BACKEND_URL);
    }

    #[test]
    fn test_get_backend_url_env_var_overrides_default() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("CLAUDE_AFK_BACKEND_URL", "http://localhost:5173");

        let result = Cmd::get_backend_url();
        assert_eq!(result, "http://localhost:5173");

        std::env::remove_var("CLAUDE_AFK_BACKEND_URL");
    }

    // ==================== PreToolUse Input Parsing Tests ====================

    #[test]
    fn test_pre_tool_use_input_parse_bash() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test", "description": "Run tests"},
            "tool_use_id": "tool-456"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "sess-123");
        assert_eq!(input.tool_name, "Bash");
        assert_eq!(input.tool_use_id, "tool-456");
    }

    #[test]
    fn test_pre_tool_use_input_parse_write() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Write",
            "tool_input": {"file_path": "/home/user/file.txt", "content": "Hello world"},
            "tool_use_id": "tool-789"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.tool_name, "Write");
    }

    #[test]
    fn test_pre_tool_use_input_parse_edit() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Edit",
            "tool_input": {"file_path": "/home/user/file.txt", "old_string": "foo", "new_string": "bar"},
            "tool_use_id": "tool-101"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.tool_name, "Edit");
    }

    #[test]
    fn test_pre_tool_use_input_parse_read() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Read",
            "tool_input": {"file_path": "/home/user/file.txt"},
            "tool_use_id": "tool-202"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.tool_name, "Read");
    }

    // ==================== ToolInfo Parsing Tests ====================

    #[test]
    fn test_tool_info_bash_with_description() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test", "description": "Run unit tests"},
            "tool_use_id": "tool-456"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();
        let tool_info = ToolInfo::from_pre_tool_use(&input);

        match tool_info {
            ToolInfo::Bash {
                command,
                description,
            } => {
                assert_eq!(command, "npm test");
                assert_eq!(description, Some("Run unit tests".to_string()));
            }
            _ => panic!("Expected Bash tool info"),
        }
    }

    #[test]
    fn test_tool_info_write() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Write",
            "tool_input": {"file_path": "/home/user/file.txt", "content": "Hello world"},
            "tool_use_id": "tool-789"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();
        let tool_info = ToolInfo::from_pre_tool_use(&input);

        match tool_info {
            ToolInfo::Write {
                file_path,
                content_preview,
            } => {
                assert_eq!(file_path, "/home/user/file.txt");
                assert_eq!(content_preview, "Hello world");
            }
            _ => panic!("Expected Write tool info"),
        }
    }

    #[test]
    fn test_tool_info_edit() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "Edit",
            "tool_input": {"file_path": "/home/user/file.txt", "old_string": "foo", "new_string": "bar"},
            "tool_use_id": "tool-101"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();
        let tool_info = ToolInfo::from_pre_tool_use(&input);

        match tool_info {
            ToolInfo::Edit {
                file_path,
                old_string,
                new_string,
            } => {
                assert_eq!(file_path, "/home/user/file.txt");
                assert_eq!(old_string, "foo");
                assert_eq!(new_string, "bar");
            }
            _ => panic!("Expected Edit tool info"),
        }
    }

    #[test]
    fn test_tool_info_unknown_tool() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PreToolUse",
            "tool_name": "SomeOtherTool",
            "tool_input": {"some_field": "some_value"},
            "tool_use_id": "tool-303"
        }"#;
        let input: PreToolUseInput = serde_json::from_str(json).unwrap();
        let tool_info = ToolInfo::from_pre_tool_use(&input);

        match tool_info {
            ToolInfo::Unknown {
                tool_name,
                raw_input,
            } => {
                assert_eq!(tool_name, "SomeOtherTool");
                assert!(raw_input.contains("some_field"));
            }
            _ => panic!("Expected Unknown tool info"),
        }
    }

    // ==================== Tool Info Notification Formatting Tests ====================

    #[test]
    fn test_bash_notification_format() {
        let tool_info = ToolInfo::Bash {
            command: "npm test".to_string(),
            description: Some("Run unit tests".to_string()),
        };
        let (title, message) = tool_info.format_for_notification();

        assert_eq!(title, "Bash Command");
        assert!(message.contains("Run unit tests"));
        assert!(message.contains("npm test"));
    }

    #[test]
    fn test_write_notification_format() {
        let tool_info = ToolInfo::Write {
            file_path: "/home/user/file.txt".to_string(),
            content_preview: "Hello world".to_string(),
        };
        let (title, message) = tool_info.format_for_notification();

        assert_eq!(title, "Write File");
        assert!(message.contains("/home/user/file.txt"));
        assert!(message.contains("Hello world"));
    }

    // ==================== NotifyPayload Tests ====================

    #[test]
    fn test_notify_payload_serialization() {
        let payload = NotifyPayload {
            title: "Test Title".to_string(),
            message: "Test Message".to_string(),
            tool_use_id: "tool-123".to_string(),
            session_id: "sess-456".to_string(),
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"title\":\"Test Title\""));
        assert!(json.contains("\"message\":\"Test Message\""));
        assert!(json.contains("\"tool_use_id\":\"tool-123\""));
        assert!(json.contains("\"session_id\":\"sess-456\""));
    }

    // ==================== HookOutput Tests ====================

    #[test]
    fn test_hook_output_allow() {
        let output = HookOutput::allow(Some("Auto-approved".to_string()));
        let json = serde_json::to_string(&output).unwrap();

        assert!(json.contains("\"permissionDecision\":\"allow\""));
        assert!(json.contains("\"permissionDecisionReason\":\"Auto-approved\""));
        assert!(json.contains("\"suppressOutput\":true"));
    }

    #[test]
    fn test_hook_output_deny() {
        let output = HookOutput::deny("Security risk".to_string());
        let json = serde_json::to_string(&output).unwrap();

        assert!(json.contains("\"permissionDecision\":\"deny\""));
        assert!(json.contains("\"permissionDecisionReason\":\"Security risk\""));
    }

    #[test]
    fn test_hook_output_ask() {
        let output = HookOutput::ask(None);
        let json = serde_json::to_string(&output).unwrap();

        assert!(json.contains("\"permissionDecision\":\"ask\""));
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
        assert_eq!(
            response.device_token,
            Some("device-token-12345".to_string())
        );
    }

    #[test]
    fn test_notify_response_parse() {
        let json = r#"{"success": true, "decisionId": "decision-abc123"}"#;
        let response: NotifyResponse = serde_json::from_str(json).unwrap();

        assert!(response.success);
        assert_eq!(response.decision_id, "decision-abc123");
    }

    #[test]
    fn test_decision_status_response_pending() {
        let json = r#"{"status": "pending", "decision": null}"#;
        let response: DecisionStatusResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status, "pending");
        assert!(response.decision.is_none());
    }

    #[test]
    fn test_decision_status_response_decided_allow() {
        let json = r#"{"status": "decided", "decision": "allow"}"#;
        let response: DecisionStatusResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status, "decided");
        assert_eq!(response.decision, Some("allow".to_string()));
    }

    #[test]
    fn test_decision_status_response_decided_dismiss() {
        let json = r#"{"status": "decided", "decision": "dismiss"}"#;
        let response: DecisionStatusResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status, "decided");
        assert_eq!(response.decision, Some("dismiss".to_string()));
    }

    #[test]
    fn test_decision_status_response_expired() {
        let json = r#"{"status": "expired", "decision": null}"#;
        let response: DecisionStatusResponse = serde_json::from_str(json).unwrap();

        assert_eq!(response.status, "expired");
        assert!(response.decision.is_none());
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
        assert_eq!(
            status_url,
            "https://example.com/api/pairing/session-123/status"
        );

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
