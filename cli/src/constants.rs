use std::time::Duration;

pub const APP_NAME: &str = "claude-afk";
pub const DEFAULT_BACKEND_URL: &str = "https://ccafk.treeleaf.dev";
pub const POLL_INTERVAL: Duration = Duration::from_secs(2);
pub const SETUP_TIMEOUT: Duration = Duration::from_secs(300); // 5 minutes
pub const DECISION_TIMEOUT: Duration = Duration::from_secs(120); // 2 minutes
pub const DECISION_POLL_INTERVAL: Duration = Duration::from_secs(2);
