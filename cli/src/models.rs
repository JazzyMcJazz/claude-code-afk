use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Deserialize)]
pub struct PairingInitResponse {
    #[serde(rename = "pairingId")]
    pub pairing_id: String,
    #[serde(rename = "pairingToken")]
    pub pairing_token: String,
}

#[derive(Deserialize)]
pub struct PairingStatusResponse {
    pub complete: bool,
    #[serde(rename = "deviceToken")]
    pub device_token: Option<String>,
}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct NotifyResponse {
    pub success: bool,
    #[serde(rename = "decisionId")]
    pub decision_id: String,
}

#[derive(Deserialize)]
pub struct DecisionStatusResponse {
    pub status: String,
    pub decision: Option<String>,
}

// ==================== PreToolUse Input Structures ====================

/// Main input structure for PreToolUse hook
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PreToolUseInput {
    pub session_id: String,
    pub transcript_path: String,
    pub cwd: String,
    pub permission_mode: String,
    pub hook_event_name: String,
    pub tool_name: String,
    pub tool_input: Value,
    pub tool_use_id: String,
}

/// Bash tool input
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct BashToolInput {
    pub command: String,
    pub description: Option<String>,
    pub timeout: Option<u64>,
    pub run_in_background: Option<bool>,
}

/// Write tool input
#[derive(Debug, Deserialize)]
pub struct WriteToolInput {
    pub file_path: String,
    pub content: String,
}

/// Edit tool input
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct EditToolInput {
    pub file_path: String,
    pub old_string: String,
    pub new_string: String,
    pub replace_all: Option<bool>,
}

/// Read tool input
#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ReadToolInput {
    pub file_path: String,
    pub offset: Option<u64>,
    pub limit: Option<u64>,
}

/// Represents the parsed tool information for display
#[derive(Debug)]
pub enum ToolInfo {
    Bash {
        command: String,
        description: Option<String>,
    },
    Write {
        file_path: String,
        content_preview: String,
    },
    Edit {
        file_path: String,
        old_string: String,
        new_string: String,
    },
    Read {
        file_path: String,
    },
    Unknown {
        tool_name: String,
        raw_input: String,
    },
}

impl ToolInfo {
    /// Parse tool_input based on tool_name
    pub fn from_pre_tool_use(input: &PreToolUseInput) -> Self {
        match input.tool_name.as_str() {
            "Bash" => {
                if let Ok(bash) = serde_json::from_value::<BashToolInput>(input.tool_input.clone())
                {
                    ToolInfo::Bash {
                        command: bash.command,
                        description: bash.description,
                    }
                } else {
                    ToolInfo::Unknown {
                        tool_name: input.tool_name.clone(),
                        raw_input: input.tool_input.to_string(),
                    }
                }
            }
            "Write" => {
                if let Ok(write) =
                    serde_json::from_value::<WriteToolInput>(input.tool_input.clone())
                {
                    let content_preview = if write.content.len() > 100 {
                        format!("{}...", &write.content[..100])
                    } else {
                        write.content
                    };
                    ToolInfo::Write {
                        file_path: write.file_path,
                        content_preview,
                    }
                } else {
                    ToolInfo::Unknown {
                        tool_name: input.tool_name.clone(),
                        raw_input: input.tool_input.to_string(),
                    }
                }
            }
            "Edit" => {
                if let Ok(edit) = serde_json::from_value::<EditToolInput>(input.tool_input.clone())
                {
                    ToolInfo::Edit {
                        file_path: edit.file_path,
                        old_string: edit.old_string,
                        new_string: edit.new_string,
                    }
                } else {
                    ToolInfo::Unknown {
                        tool_name: input.tool_name.clone(),
                        raw_input: input.tool_input.to_string(),
                    }
                }
            }
            "Read" => {
                if let Ok(read) = serde_json::from_value::<ReadToolInput>(input.tool_input.clone())
                {
                    ToolInfo::Read {
                        file_path: read.file_path,
                    }
                } else {
                    ToolInfo::Unknown {
                        tool_name: input.tool_name.clone(),
                        raw_input: input.tool_input.to_string(),
                    }
                }
            }
            _ => ToolInfo::Unknown {
                tool_name: input.tool_name.clone(),
                raw_input: input.tool_input.to_string(),
            },
        }
    }

    /// Format the tool info for display in a notification
    pub fn format_for_notification(&self) -> (String, String) {
        match self {
            ToolInfo::Bash {
                command,
                description,
            } => {
                let title = "Bash Command".to_string();
                let message = if let Some(desc) = description {
                    format!("{}\n\n{}", desc, command)
                } else {
                    command.clone()
                };
                (title, message)
            }
            ToolInfo::Write {
                file_path,
                content_preview,
            } => {
                let title = "Write File".to_string();
                let message = format!("{}\n\n{}", file_path, content_preview);
                (title, message)
            }
            ToolInfo::Edit {
                file_path,
                old_string,
                new_string,
            } => {
                let title = "Edit File".to_string();
                let old_preview = if old_string.len() > 50 {
                    format!("{}...", &old_string[..50])
                } else {
                    old_string.clone()
                };
                let new_preview = if new_string.len() > 50 {
                    format!("{}...", &new_string[..50])
                } else {
                    new_string.clone()
                };
                let message = format!("{}\n\n- {}\n+ {}", file_path, old_preview, new_preview);
                (title, message)
            }
            ToolInfo::Read { file_path } => {
                let title = "Read File".to_string();
                let message = file_path.clone();
                (title, message)
            }
            ToolInfo::Unknown {
                tool_name,
                raw_input,
            } => {
                let title = format!("Tool: {}", tool_name);
                let message = if raw_input.len() > 200 {
                    format!("{}...", &raw_input[..200])
                } else {
                    raw_input.clone()
                };
                (title, message)
            }
        }
    }
}

// ==================== Hook Output Structures ====================

/// Output structure for PreToolUse hook response
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct HookOutput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hook_specific_output: Option<PreToolUseOutput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suppress_output: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreToolUseOutput {
    pub hook_event_name: String,
    pub permission_decision: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_decision_reason: Option<String>,
}

impl HookOutput {
    pub fn allow(reason: Option<String>) -> Self {
        HookOutput {
            hook_specific_output: Some(PreToolUseOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "allow".to_string(),
                permission_decision_reason: reason,
            }),
            suppress_output: Some(true),
        }
    }

    #[allow(dead_code)]
    pub fn deny(reason: String) -> Self {
        HookOutput {
            hook_specific_output: Some(PreToolUseOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "deny".to_string(),
                permission_decision_reason: Some(reason),
            }),
            suppress_output: None,
        }
    }

    pub fn ask(reason: Option<String>) -> Self {
        HookOutput {
            hook_specific_output: Some(PreToolUseOutput {
                hook_event_name: "PreToolUse".to_string(),
                permission_decision: "ask".to_string(),
                permission_decision_reason: reason,
            }),
            suppress_output: None,
        }
    }
}

// ==================== Notify Payload ====================

#[derive(Debug, Serialize)]
pub struct NotifyPayload {
    pub title: String,
    pub message: String,
    pub tool_use_id: String,
    pub session_id: String,
}
