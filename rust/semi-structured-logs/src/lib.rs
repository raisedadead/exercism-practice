// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}

/// return a message for the given log level
pub fn log_type(level: LogLevel) -> String {
    match level {
        LogLevel::Info => "[INFO]".to_string(),
        LogLevel::Warning => "[WARNING]".to_string(),
        LogLevel::Error => "[ERROR]".to_string(),
    }
}

/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    format!("{}: {}", log_type(level), message)
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
