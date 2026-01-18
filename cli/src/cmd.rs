use std::{fs, io::Read, path::PathBuf, thread, time::Instant};

use colored::Colorize;
use nanoid::nanoid;
use qrcode::{render::unicode::Dense1x2, QrCode};

use crate::{
    config::Config,
    constants::{
        DECISION_POLL_INTERVAL, DECISION_TIMEOUT, DEFAULT_API_URL, POLL_INTERVAL, SETUP_TIMEOUT,
    },
    logger::Logger,
    models::{
        DecisionStatusResponse, GenericHookInput, HookOutput, NotificationInput, NotifyPayload,
        NotifyResponse, PairingInitResponse, PairingStatusResponse, PermissionRequestInput,
        SimpleNotifyPayload, ToolInfo,
    },
};

pub struct Cmd;

impl Cmd {
    pub fn pair() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        let backend_url = Self::get_backend_url();
        let backend_url = backend_url.trim_end_matches('/');

        // Initiate pairing
        println!();
        println!("  {} {}", "◆".cyan(), "Claude AFK Pairing".bold());
        println!("  {} {}", "→".dimmed(), backend_url.dimmed());
        println!();

        let response: PairingInitResponse =
            ureq::post(&format!("{}/api/pairing/initiate", backend_url))
                .send_empty()?
                .into_body()
                .read_json()?;

        let pairing_url = format!("{}/pair/{}", backend_url, response.pairing_token);

        println!("  📱 Scan this QR code with your phone:");
        println!();
        Self::render_qr(&pairing_url)?;
        println!();
        println!(
            "  {} {}",
            "Or open:".dimmed(),
            pairing_url.cyan().underline()
        );
        println!();
        println!(
            "  {} Waiting for pairing... {}",
            "◌".yellow(),
            "(press Ctrl+C to cancel)".dimmed()
        );

        // Poll for completion
        let start = Instant::now();
        loop {
            if start.elapsed() > SETUP_TIMEOUT {
                println!();
                println!(
                    "  {} {}",
                    "✗".red(),
                    "Pairing timed out after 5 minutes".red()
                );
                return Err("Pairing timed out".into());
            }

            thread::sleep(POLL_INTERVAL);

            let status: PairingStatusResponse = ureq::get(&format!(
                "{}/api/pairing/{}/status",
                backend_url, response.pairing_id
            ))
            .call()?
            .into_body()
            .read_json()?;

            if status.complete {
                if let Some(device_token) = status.device_token {
                    config.device_token = Some(device_token);
                    config.backend_url = backend_url.to_string();
                    config.active = true;
                    Config::save(&config)?;

                    println!();
                    println!(
                        "  {} {}",
                        "✓".green().bold(),
                        "Pairing successful!".green().bold()
                    );
                    println!(
                        "    {} Notifications are now {}",
                        "→".dimmed(),
                        "enabled".green()
                    );
                    println!();
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
            std::process::exit(0);
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

        // First, determine the hook type
        let generic_input: GenericHookInput = match serde_json::from_str(&input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", &format!("Failed to parse hook input: {}", e));
                std::process::exit(1);
            }
        };

        // Handle based on hook type
        match generic_input.hook_event_name.as_str() {
            "Notification" => Self::handle_notification(&input, &device_token, &backend_url),
            "PermissionRequest" => {
                Self::handle_permission_request(&input, &device_token, &backend_url)
            }
            _ => {
                eprintln!("Unknown hook event: {}", generic_input.hook_event_name);
                std::process::exit(1);
            }
        }
    }

    fn handle_notification(
        input: &str,
        device_token: &str,
        backend_url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let notification: NotificationInput = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("{}", &format!("Failed to parse Notification input: {}", e));
                std::process::exit(1);
            }
        };

        // Only handle idle_prompt notifications
        if notification.notification_type != "idle_prompt" {
            // For other notification types, exit silently
            std::process::exit(0);
        }

        // Use simple notification endpoint - no decision tracking needed
        let payload = SimpleNotifyPayload {
            title: "Claude is waiting".to_string(),
            message: notification.message.clone(),
        };

        // Send notification and exit immediately (no decision polling for notifications)
        match ureq::post(&format!("{}/api/notify/simple", backend_url))
            .header("Authorization", &format!("Bearer {}", device_token))
            .send_json(&payload)
        {
            Ok(_) => {
                Logger::debug("Notification sent successfully");
            }
            Err(e) => {
                eprintln!("{}", &format!("Failed to send notification: {}", e));
            }
        }

        // Exit silently - notifications don't need any output
        std::process::exit(0);
    }

    fn handle_permission_request(
        input: &str,
        device_token: &str,
        backend_url: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let pre_tool_use: PermissionRequestInput = match serde_json::from_str(input) {
            Ok(v) => v,
            Err(e) => {
                eprintln!(
                    "{}",
                    &format!("Failed to parse PermissionRequest input: {}", e)
                );
                std::process::exit(1);
            }
        };

        // Generate or use provided tool_use_id
        let tool_use_id = pre_tool_use
            .tool_use_id
            .clone()
            .unwrap_or_else(|| nanoid!(21));

        // Parse tool-specific information
        let tool_info = ToolInfo::from_pre_tool_use(&pre_tool_use);
        let (title, message) = tool_info.format_for_notification();

        let payload = NotifyPayload {
            title,
            message,
            tool_use_id: tool_use_id.clone(),
            session_id: pre_tool_use.session_id.clone(),
        };

        // Send notification to backend and get decision ID
        let notify_response: NotifyResponse =
            match ureq::post(&format!("{}/api/notify", backend_url))
                .header("Authorization", &format!("Bearer {}", device_token))
                .send_json(&payload)
            {
                Ok(resp) => match resp.into_body().read_json() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", &format!("Failed to parse notify response: {}", e));
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("{}", &format!("Failed to send notification: {}", e));
                    std::process::exit(1);
                }
            };

        // Poll for decision
        let decision_id = notify_response.decision_id;
        let start = Instant::now();

        loop {
            if start.elapsed() > DECISION_TIMEOUT {
                // Timeout - fall back to ask
                eprintln!("Decision timed out");
                std::process::exit(1);
            }

            thread::sleep(DECISION_POLL_INTERVAL);

            let status_response: DecisionStatusResponse = match ureq::get(&format!(
                "{}/api/decision/{}/status",
                backend_url, decision_id
            ))
            .header("Authorization", &format!("Bearer {}", device_token))
            .call()
            {
                Ok(resp) => match resp.into_body().read_json() {
                    Ok(r) => r,
                    Err(e) => {
                        eprintln!("{}", &format!("Failed to parse decision status: {}", e));
                        std::process::exit(1);
                    }
                },
                Err(e) => {
                    eprintln!("{}", &format!("Failed to poll decision status: {}", e));
                    std::process::exit(1);
                }
            };

            match status_response.status.as_str() {
                "decided" => {
                    match status_response.decision.as_deref() {
                        Some("allow") => {
                            let output = HookOutput::allow();
                            println!("{}", serde_json::to_string(&output)?);
                            return Ok(());
                        }
                        Some("deny") => {
                            let output = HookOutput::deny(None);
                            println!("{}", serde_json::to_string(&output)?);
                            return Ok(());
                        }
                        Some("dismiss") => {
                            // Dismissed decision - exit silently
                            std::process::exit(0);
                        }
                        _ => {
                            // Unknown decision - exit silently
                            eprintln!("Unknown decision");
                            std::process::exit(1);
                        }
                    }
                }
                "pending" => {
                    // Continue polling
                    Logger::debug("Decision pending, continuing to poll");
                }
                _ => {
                    // Unknown status - fall back to asking user normally
                    eprintln!("Unknown decision status, falling back to asking user normally");
                    std::process::exit(1);
                }
            }
        }
    }

    pub fn status() -> Result<(), Box<dyn std::error::Error>> {
        let config = Config::load()?;

        let device_paired = config.device_token.is_some();
        let notifications_active = config.active;
        let hooks_installed = Self::hooks_installed();

        println!();
        println!("  {} {}", "◆".cyan(), "Claude AFK Status".bold());
        println!();

        // Device pairing status
        let (pair_icon, pair_status) = if device_paired {
            ("✓".green(), "Paired".green())
        } else {
            ("✗".red(), "Not paired".red())
        };
        println!("  {} Device          {}", pair_icon, pair_status);

        // Notifications status
        let (notif_icon, notif_status) = if notifications_active {
            ("✓".green(), "Active".green())
        } else {
            ("○".yellow(), "Inactive".yellow())
        };
        println!("  {} Notifications   {}", notif_icon, notif_status);

        // Hooks status
        let (hooks_icon, hooks_status) = if hooks_installed {
            ("✓".green(), "Installed".green())
        } else {
            ("○".yellow(), "Not installed".yellow())
        };
        println!("  {} Hooks           {}", hooks_icon, hooks_status);

        // Helpful hints
        if !device_paired {
            println!();
            println!(
                "  {} Run {} to set up notifications",
                "Tip:".dimmed(),
                "claude-afk pair".cyan()
            );
        } else if !hooks_installed {
            println!();
            println!(
                "  {} Run {} to install Claude Code hooks",
                "Tip:".dimmed(),
                "claude-afk install-hook".cyan()
            );
        } else if !notifications_active {
            println!();
            println!(
                "  {} Run {} to enable notifications",
                "Tip:".dimmed(),
                "claude-afk activate".cyan()
            );
        }
        println!();

        Ok(())
    }

    pub fn activate() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        if config.device_token.is_none() {
            println!();
            println!("  {} {}", "✗".red(), "No device paired".red());
            println!(
                "    {} Run {} first",
                "→".dimmed(),
                "claude-afk pair".cyan()
            );
            println!();
            return Err("No device paired".into());
        }

        if config.active {
            println!();
            println!(
                "  {} Notifications are already {}",
                "○".dimmed(),
                "active".green()
            );
            println!();
            return Ok(());
        }

        config.active = true;
        Config::save(&config)?;

        println!();
        println!(
            "  {} Notifications {}",
            "✓".green().bold(),
            "activated".green().bold()
        );
        println!(
            "    {} You'll receive push notifications when Claude needs input",
            "→".dimmed()
        );
        println!();
        Ok(())
    }

    pub fn deactivate() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        if !config.active {
            println!();
            println!(
                "  {} Notifications are already {}",
                "○".dimmed(),
                "inactive".yellow()
            );
            println!();
            return Ok(());
        }

        config.active = false;
        Config::save(&config)?;

        println!();
        println!(
            "  {} Notifications {}",
            "○".yellow(),
            "deactivated".yellow()
        );
        println!(
            "    {} Run {} to re-enable",
            "→".dimmed(),
            "claude-afk activate".cyan()
        );
        println!();
        Ok(())
    }

    pub fn clear() -> Result<(), Box<dyn std::error::Error>> {
        let mut config = Config::load()?;

        if config.device_token.is_none() {
            println!();
            println!(
                "  {} {}",
                "○".dimmed(),
                "No device pairing to clear".dimmed()
            );
            println!();
            return Ok(());
        }

        config.device_token = None;
        config.active = false;
        Config::save(&config)?;

        println!();
        println!("  {} Device pairing {}", "✓".green(), "cleared".white());
        println!(
            "    {} Run {} to pair a new device",
            "→".dimmed(),
            "claude-afk pair".cyan()
        );
        println!();
        Ok(())
    }

    pub fn install_hooks() -> Result<(), Box<dyn std::error::Error>> {
        println!();
        println!("  {} {}", "◆".cyan(), "Installing Claude Code Hooks".bold());
        println!();

        // Get the path to the current executable
        let exe_path = std::env::current_exe()?;
        let exe_path_str = exe_path.to_string_lossy().to_string();

        // Find the Claude Code settings file
        let home = std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE"))?;
        let settings_path = PathBuf::from(&home).join(".claude").join("settings.json");

        // Ensure .claude directory exists
        if let Some(parent) = settings_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Read existing settings or create empty object
        let mut settings: serde_json::Value = if settings_path.exists() {
            let content = fs::read_to_string(&settings_path)?;
            serde_json::from_str(&content).unwrap_or_else(|_| serde_json::json!({}))
        } else {
            serde_json::json!({})
        };

        // Create the PermissionRequest hook structure (wildcard matcher)
        let permission_hook_entry = serde_json::json!({
            "matcher": "*",
            "hooks": [
                {
                    "type": "command",
                    "command": exe_path_str
                }
            ]
        });

        // Create the Notification hook structure (idle_prompt only)
        let notification_hook_entry = serde_json::json!({
            "matcher": "idle_prompt",
            "hooks": [
                {
                    "type": "command",
                    "command": exe_path_str
                }
            ]
        });

        // Get or create the hooks object
        let hooks = settings
            .as_object_mut()
            .ok_or("Settings is not an object")?
            .entry("hooks")
            .or_insert_with(|| serde_json::json!({}));

        let hooks_obj = hooks.as_object_mut().ok_or("Hooks is not an object")?;

        // Helper closure to check if a hook entry contains claude-afk
        let contains_claude_afk = |hook: &serde_json::Value| -> bool {
            if let Some(inner_hooks) = hook.get("hooks").and_then(|h| h.as_array()) {
                inner_hooks.iter().any(|h| {
                    h.get("command")
                        .and_then(|c| c.as_str())
                        .map(|c| c.contains("claude-afk"))
                        .unwrap_or(false)
                })
            } else {
                false
            }
        };

        // Install PermissionRequest hook
        let permission_request = hooks_obj
            .entry("PermissionRequest")
            .or_insert_with(|| serde_json::json!([]));

        let permission_array = permission_request
            .as_array_mut()
            .ok_or("PermissionRequest is not an array")?;

        // Remove any existing claude-afk hooks and add the new one
        permission_array.retain(|hook| !contains_claude_afk(hook));
        permission_array.push(permission_hook_entry);

        // Install Notification hook
        let notification = hooks_obj
            .entry("Notification")
            .or_insert_with(|| serde_json::json!([]));

        let notification_array = notification
            .as_array_mut()
            .ok_or("Notification is not an array")?;

        // Remove any existing claude-afk hooks and add the new one
        notification_array.retain(|hook| !contains_claude_afk(hook));
        notification_array.push(notification_hook_entry);

        // Write the settings back
        let formatted = serde_json::to_string_pretty(&settings)?;
        fs::write(&settings_path, formatted)?;

        println!(
            "  {} Hooks installed to {}",
            "✓".green().bold(),
            "~/.claude/settings.json".cyan()
        );
        println!();
        println!("  {} Binary path:", "→".dimmed());
        println!("    {}", exe_path_str.dimmed());
        println!();
        println!("  {} Installed hooks:", "→".dimmed());
        println!("    • PermissionRequest (all tools)");
        println!("    • Notification (idle_prompt)");
        println!();
        println!(
            "  {} Claude Code will now send push notifications",
            "✓".green()
        );
        println!("    when it needs your input or is waiting.");
        println!();

        println!(
            "  {} Run {} to enable notifications",
            "Tip:".dimmed(),
            "claude-afk activate".cyan()
        );
        println!();

        Ok(())
    }

    #[cfg(debug_assertions)]
    pub fn clear_logs() -> Result<(), Box<dyn std::error::Error>> {
        Logger::clear_logs()?;
        println!();
        println!("  {} Debug logs {}", "✓".green(), "cleared".white());
        println!();
        Ok(())
    }

    fn get_backend_url() -> String {
        // Priority: env var (for local development) > default production URL
        std::env::var("CLAUDE_AFK_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
    }

    fn hooks_installed() -> bool {
        let home = match std::env::var("HOME").or_else(|_| std::env::var("USERPROFILE")) {
            Ok(h) => h,
            Err(_) => return false,
        };

        let settings_path = PathBuf::from(&home).join(".claude").join("settings.json");

        if !settings_path.exists() {
            return false;
        }

        let content = match fs::read_to_string(&settings_path) {
            Ok(c) => c,
            Err(_) => return false,
        };

        let settings: serde_json::Value = match serde_json::from_str(&content) {
            Ok(s) => s,
            Err(_) => return false,
        };

        // Check if hooks object exists with claude-afk entries
        let hooks = match settings.get("hooks") {
            Some(h) => h,
            None => return false,
        };

        // Helper to check if a hook array contains claude-afk
        let has_claude_afk = |hook_array: &serde_json::Value| -> bool {
            hook_array
                .as_array()
                .map(|arr| {
                    arr.iter().any(|hook| {
                        hook.get("hooks")
                            .and_then(|h| h.as_array())
                            .map(|inner| {
                                inner.iter().any(|h| {
                                    h.get("command")
                                        .and_then(|c| c.as_str())
                                        .map(|c| c.contains("claude-afk"))
                                        .unwrap_or(false)
                                })
                            })
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false)
        };

        // Check both PermissionRequest and Notification hooks
        let has_permission = hooks
            .get("PermissionRequest")
            .map(has_claude_afk)
            .unwrap_or(false);

        let has_notification = hooks
            .get("Notification")
            .map(has_claude_afk)
            .unwrap_or(false);

        has_permission && has_notification
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
    use crate::models::{
        GenericHookInput, HookOutput, NotificationInput, NotifyPayload, PermissionRequestInput,
        SimpleNotifyPayload, ToolInfo,
    };

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
        std::env::remove_var("CLAUDE_AFK_API_URL");

        let result = Cmd::get_backend_url();
        assert_eq!(result, DEFAULT_API_URL);
    }

    #[test]
    fn test_get_backend_url_env_var_overrides_default() {
        let _guard = ENV_MUTEX.lock().unwrap();
        std::env::set_var("CLAUDE_AFK_API_URL", "http://localhost:5173");

        let result = Cmd::get_backend_url();
        assert_eq!(result, "http://localhost:5173");

        std::env::remove_var("CLAUDE_AFK_API_URL");
    }

    // ==================== PermissionRequest Input Parsing Tests ====================

    #[test]
    fn test_pre_tool_use_input_parse_bash() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test", "description": "Run tests"},
            "tool_use_id": "tool-456"
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "sess-123");
        assert_eq!(input.tool_name, "Bash");
        assert_eq!(input.tool_use_id, Some("tool-456".to_string()));
    }

    #[test]
    fn test_pre_tool_use_input_parse_write() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Write",
            "tool_input": {"file_path": "/home/user/file.txt", "content": "Hello world"},
            "tool_use_id": "tool-789"
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.tool_name, "Write");
        assert_eq!(input.tool_use_id, Some("tool-789".to_string()));
    }

    #[test]
    fn test_pre_tool_use_input_parse_edit() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Edit",
            "tool_input": {"file_path": "/home/user/file.txt", "old_string": "foo", "new_string": "bar"},
            "tool_use_id": "tool-101"
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.tool_name, "Edit");
        assert_eq!(input.tool_use_id, Some("tool-101".to_string()));
    }

    #[test]
    fn test_pre_tool_use_input_parse_read() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Read",
            "tool_input": {"file_path": "/home/user/file.txt"},
            "tool_use_id": "tool-202"
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.tool_name, "Read");
        assert_eq!(input.tool_use_id, Some("tool-202".to_string()));
    }

    #[test]
    fn test_pre_tool_use_input_parse_missing_tool_use_id() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test"}
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "sess-123");
        assert_eq!(input.tool_name, "Bash");
        assert!(input.tool_use_id.is_none());
    }

    // ==================== ToolInfo Parsing Tests ====================

    #[test]
    fn test_tool_info_bash_with_description() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test", "description": "Run unit tests"}
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();
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
            "hook_event_name": "PermissionRequest",
            "tool_name": "Write",
            "tool_input": {"file_path": "/home/user/file.txt", "content": "Hello world"}
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();
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
            "hook_event_name": "PermissionRequest",
            "tool_name": "Edit",
            "tool_input": {"file_path": "/home/user/file.txt", "old_string": "foo", "new_string": "bar"}
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();
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
            "hook_event_name": "PermissionRequest",
            "tool_name": "SomeOtherTool",
            "tool_input": {"some_field": "some_value"}
        }"#;
        let input: PermissionRequestInput = serde_json::from_str(json).unwrap();
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

        assert_eq!(title, "Run bash command? 🐚");
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

        assert_eq!(title, "Write file? 📝");
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
        let output = HookOutput::allow();
        let json = serde_json::to_string(&output).unwrap();

        assert!(json.contains("\"hookSpecificOutput\""));
        assert!(json.contains("\"hookEventName\":\"PermissionRequest\""));
        assert!(json.contains("\"behavior\":\"allow\""));
        assert!(json.contains("\"suppressOutput\":true"));
    }

    #[test]
    fn test_hook_output_deny() {
        let output = HookOutput::deny(Some("Security risk".to_string()));
        let json = serde_json::to_string(&output).unwrap();

        assert!(json.contains("\"hookSpecificOutput\""));
        assert!(json.contains("\"hookEventName\":\"PermissionRequest\""));
        assert!(json.contains("\"behavior\":\"deny\""));
        assert!(json.contains("\"message\":\"Security risk\""));
        assert!(json.contains("\"interrupt\":true"));
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

    // ==================== GenericHookInput Tests ====================

    #[test]
    fn test_generic_hook_input_parse_permission_request() {
        let json = r#"{
            "session_id": "sess-123",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test"}
        }"#;
        let input: GenericHookInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "sess-123");
        assert_eq!(input.hook_event_name, "PermissionRequest");
    }

    #[test]
    fn test_generic_hook_input_parse_notification() {
        let json = r#"{
            "session_id": "sess-456",
            "hook_event_name": "Notification",
            "message": "Claude is waiting",
            "notification_type": "idle_prompt"
        }"#;
        let input: GenericHookInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "sess-456");
        assert_eq!(input.hook_event_name, "Notification");
    }

    // ==================== NotificationInput Tests ====================

    #[test]
    fn test_notification_input_parse_idle_prompt() {
        let json = r#"{
            "session_id": "abc123",
            "transcript_path": "/Users/test/.claude/projects/test/00893aaf.jsonl",
            "cwd": "/Users/test/project",
            "permission_mode": "default",
            "hook_event_name": "Notification",
            "message": "Claude needs your permission to use Bash",
            "notification_type": "idle_prompt"
        }"#;
        let input: NotificationInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "abc123");
        assert_eq!(input.hook_event_name, "Notification");
        assert_eq!(input.message, "Claude needs your permission to use Bash");
        assert_eq!(input.notification_type, "idle_prompt");
    }

    #[test]
    fn test_notification_input_parse_permission_prompt() {
        let json = r#"{
            "session_id": "xyz789",
            "transcript_path": "/path/to/transcript.jsonl",
            "cwd": "/project",
            "permission_mode": "default",
            "hook_event_name": "Notification",
            "message": "Claude is asking for permission",
            "notification_type": "permission_prompt"
        }"#;
        let input: NotificationInput = serde_json::from_str(json).unwrap();

        assert_eq!(input.session_id, "xyz789");
        assert_eq!(input.notification_type, "permission_prompt");
    }

    // ==================== SimpleNotifyPayload Tests ====================

    #[test]
    fn test_simple_notify_payload_serialization() {
        let payload = SimpleNotifyPayload {
            title: "Claude is waiting".to_string(),
            message: "Claude needs your input".to_string(),
        };

        let json = serde_json::to_string(&payload).unwrap();
        assert!(json.contains("\"title\":\"Claude is waiting\""));
        assert!(json.contains("\"message\":\"Claude needs your input\""));
        // Should NOT contain tool_use_id or session_id
        assert!(!json.contains("tool_use_id"));
        assert!(!json.contains("session_id"));
    }

    #[test]
    fn test_simple_notify_payload_minimal() {
        let payload = SimpleNotifyPayload {
            title: "Test".to_string(),
            message: "Test message".to_string(),
        };

        let json = serde_json::to_string(&payload).unwrap();
        // Verify it's a simple 2-field JSON object
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.as_object().unwrap().len(), 2);
    }

    // ==================== Hook Type Detection Tests ====================

    #[test]
    fn test_hook_type_detection_permission_request() {
        let json = r#"{
            "session_id": "sess-123",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "PermissionRequest",
            "tool_name": "Bash",
            "tool_input": {"command": "npm test"}
        }"#;

        let generic: GenericHookInput = serde_json::from_str(json).unwrap();
        assert_eq!(generic.hook_event_name, "PermissionRequest");

        // Should be able to parse as PermissionRequestInput
        let permission: PermissionRequestInput = serde_json::from_str(json).unwrap();
        assert_eq!(permission.tool_name, "Bash");
    }

    #[test]
    fn test_hook_type_detection_notification() {
        let json = r#"{
            "session_id": "sess-456",
            "transcript_path": "/tmp/transcript.json",
            "cwd": "/home/user/project",
            "permission_mode": "default",
            "hook_event_name": "Notification",
            "message": "Claude is idle",
            "notification_type": "idle_prompt"
        }"#;

        let generic: GenericHookInput = serde_json::from_str(json).unwrap();
        assert_eq!(generic.hook_event_name, "Notification");

        // Should be able to parse as NotificationInput
        let notification: NotificationInput = serde_json::from_str(json).unwrap();
        assert_eq!(notification.notification_type, "idle_prompt");
    }
}
