/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Debug => format!("[DEBUG]: {}", message),
        LogLevel::Info => format!("[INFO]: {}", message),
        LogLevel::Warning => format!("[WARNING]: {}", message),
        LogLevel::Error => format!("[ERROR]: {}", message),
    }
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
