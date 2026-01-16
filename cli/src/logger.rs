#![allow(unused)]

#[cfg(debug_assertions)]
use std::fs::OpenOptions;
#[cfg(debug_assertions)]
use std::io::Write;
#[cfg(debug_assertions)]
use std::path::PathBuf;
#[cfg(debug_assertions)]
use std::sync::OnceLock;
#[cfg(debug_assertions)]
use std::time::SystemTime;

#[cfg(debug_assertions)]
use crate::constants::APP_NAME;

#[cfg(debug_assertions)]
static LOG_FILE_PATH: OnceLock<Option<PathBuf>> = OnceLock::new();

/// Get the path to the log file (same directory as config file)
#[cfg(debug_assertions)]
fn get_log_file_path() -> Option<PathBuf> {
    LOG_FILE_PATH
        .get_or_init(|| {
            confy::get_configuration_file_path(APP_NAME, None)
                .ok()
                .and_then(|config_path| config_path.parent().map(|p| p.to_path_buf()))
                .map(|dir| dir.join("debug.log"))
        })
        .clone()
}

/// Format a timestamp from SystemTime
#[cfg(debug_assertions)]
fn format_timestamp() -> String {
    let now = SystemTime::now();
    let duration = now
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default();
    let secs = duration.as_secs();

    // Simple UTC timestamp formatting without external deps
    let days_since_epoch = secs / 86400;
    let time_of_day = secs % 86400;
    let hours = time_of_day / 3600;
    let minutes = (time_of_day % 3600) / 60;
    let seconds = time_of_day % 60;

    // Calculate date from days since epoch (1970-01-01)
    let (year, month, day) = days_to_date(days_since_epoch);

    format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02}",
        year, month, day, hours, minutes, seconds
    )
}

/// Convert days since Unix epoch to (year, month, day)
#[cfg(debug_assertions)]
fn days_to_date(days: u64) -> (u64, u64, u64) {
    let mut days = days as i64;
    let mut year = 1970i64;

    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let leap = is_leap_year(year);
    let days_in_months: [i64; 12] = if leap {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month = 1i64;
    for &dim in &days_in_months {
        if days < dim {
            break;
        }
        days -= dim;
        month += 1;
    }

    (year as u64, month as u64, (days + 1) as u64)
}

#[cfg(debug_assertions)]
fn is_leap_year(year: i64) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Write a log entry to the file (debug builds only)
#[cfg(debug_assertions)]
fn write_log(level: &str, message: &str) {
    let Some(path) = get_log_file_path() else {
        return;
    };

    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let Ok(mut file) = OpenOptions::new().create(true).append(true).open(&path) else {
        return;
    };

    let timestamp = format_timestamp();
    let _ = writeln!(file, "[{}] {} - {}", timestamp, level, message);
}

/// No-op in release builds
#[cfg(not(debug_assertions))]
fn write_log(_level: &str, _message: &str) {}

pub struct Logger;

impl Logger {
    /// Log a debug message (only in debug builds)
    pub fn debug(message: &str) {
        write_log("DEBUG", message);
    }

    /// Log an info message (only in debug builds)
    pub fn info(message: &str) {
        write_log("INFO", message);
    }

    /// Log a warning message (only in debug builds)
    pub fn warn(message: &str) {
        write_log("WARN", message);
    }

    /// Log an error message (only in debug builds)
    pub fn error(message: &str) {
        write_log("ERROR", message);
    }

    /// Get the path to the log file (debug builds only)
    #[cfg(debug_assertions)]
    pub fn log_file_path() -> Option<PathBuf> {
        confy::get_configuration_file_path(APP_NAME, None)
            .ok()
            .and_then(|config_path| config_path.parent().map(|p| p.to_path_buf()))
            .map(|dir| dir.join("debug.log"))
    }

    /// Clear all logs from the log file (debug builds only)
    #[cfg(debug_assertions)]
    pub fn clear_logs() -> Result<(), std::io::Error> {
        if let Some(path) = Self::log_file_path() {
            if path.exists() {
                std::fs::remove_file(&path)?;
            }
        }
        Ok(())
    }
}

#[cfg(all(test, debug_assertions))]
mod tests {
    use super::*;

    #[test]
    fn test_days_to_date() {
        // 1970-01-01
        assert_eq!(days_to_date(0), (1970, 1, 1));
        // 1970-01-02
        assert_eq!(days_to_date(1), (1970, 1, 2));
        // 2000-01-01 (leap year)
        assert_eq!(days_to_date(10957), (2000, 1, 1));
        // 2024-01-01
        assert_eq!(days_to_date(19723), (2024, 1, 1));
    }

    #[test]
    fn test_is_leap_year() {
        assert!(!is_leap_year(1970));
        assert!(is_leap_year(2000));
        assert!(!is_leap_year(1900));
        assert!(is_leap_year(2024));
    }
}
