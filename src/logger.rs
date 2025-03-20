use core::fmt;
use std::{env, sync::Mutex, io::Write};

use once_cell::sync::Lazy;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    None = 0,
    Brief = 1,
    Detailed = 2,
    Trace = 3,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            0 => LogLevel::None,
            1 => LogLevel::Brief,
            2 => LogLevel::Detailed,
            3 => LogLevel::Trace,
            _ => LogLevel::None,
        }
    }
}

pub enum LogSeverity {
    Error,
    Warning,
    Info,
}

impl fmt::Display for LogSeverity {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogSeverity::Error => write!(f, "ERROR"),
            LogSeverity::Warning => write!(f, "WARNING"),
            LogSeverity::Info => write!(f, "INFO"),
        }
    }
}

impl LogLevel {
    pub fn at_or_under(&self, level: &LogLevel) -> bool {
        *self <= *level
    }
} 

static MASTER_LOGGER: Lazy<Mutex<MasterLogger>> = Lazy::new(|| Mutex::new(MasterLogger::new())); 

pub struct MasterLogger {
    console_log_level: LogLevel,
    file_log_level: LogLevel,
    log_file_handle: Option<std::fs::File>,
}

impl MasterLogger {
    pub fn open() {
        let log_file_directory = env::var("LOG_PATH").unwrap_or("./".to_string());
        let log_file_path = format!("{}/log-{}.log", log_file_directory, chrono::Local::now().format("%Y%m%d-%H:%M:%S"));

        println!("Logging to file: {}", log_file_path);

        let log_file_handle = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)
            .expect("Error opening log file");

        MASTER_LOGGER.lock().unwrap().log_file_handle = Some(log_file_handle);
    }

    pub fn close() {
        MASTER_LOGGER.lock().unwrap().log_file_handle = None;
    }

    fn get_log_level(variable_name: &str) -> LogLevel {
        env::var(variable_name).map_or_else(|_| LogLevel::None, |log_level| {
            log_level.parse::<i32>().map(|log_level| log_level.into()).unwrap_or(LogLevel::None)
        })
    }

    fn new() -> MasterLogger {
        MasterLogger {
            console_log_level: Self::get_log_level("CONSOLE_LOG_LEVEL"),
            file_log_level: Self::get_log_level("FILE_LOG_LEVEL"),
            log_file_handle: None,
        }
    }

    fn log_to_console(source: &str, level: &LogLevel, severity: &LogSeverity, message: &str) {
        let logger = MASTER_LOGGER.lock().unwrap();

        if level.at_or_under(&logger.console_log_level) {
            println!("[{} - {}] {}", source, severity, message);
        }
    }

    fn log_to_file(source: &str, level: &LogLevel, severity: &LogSeverity, message: &str) {
        let mut logger = MASTER_LOGGER.lock().unwrap();

        if level.at_or_under(&logger.file_log_level) {
            let log_file_handle = logger.log_file_handle.as_mut().expect("Log file handle not initialized");

            writeln!(log_file_handle, "[{} - {}] {}", source, severity, message).expect("Error writing to log file");
        }
    }
}

pub struct Logger {
    source: String
}

impl Logger {
    pub fn new(source: &str) -> Logger {
        Logger {
            source: source.to_string(),
        }
    }

    pub fn log_brief(&self, severity: LogSeverity, message: &str) {
        self.log(LogLevel::Brief, severity, message);
    }

    pub fn log_brief_info(&self, message: &str) {
        self.log_brief(LogSeverity::Info, message);
    }

    pub fn log_brief_warning(&self, message: &str) {
        self.log_brief(LogSeverity::Warning, message);
    }

    pub fn log_brief_error(&self, message: &str) {
        self.log_brief(LogSeverity::Error, message);
    }

    pub fn log_detailed(&self, severity: LogSeverity, message: &str) {
        self.log(LogLevel::Detailed, severity, message);
    }

    pub fn log_detailed_info(&self, message: &str) {
        self.log_detailed(LogSeverity::Info, message);
    }

    pub fn log_detailed_warning(&self, message: &str) {
        self.log_detailed(LogSeverity::Warning, message);
    }

    pub fn log_detailed_error(&self, message: &str) {
        self.log_detailed(LogSeverity::Error, message);
    }

    pub fn log_trace(&self, severity: LogSeverity, message: &str) {
        self.log(LogLevel::Trace, severity, message);
    }

    pub fn log_trace_info(&self, message: &str) {
        self.log_trace(LogSeverity::Info, message);
    }

    pub fn log_trace_warning(&self, message: &str) {
        self.log_trace(LogSeverity::Warning, message);
    }

    pub fn log_trace_error(&self, message: &str) {
        self.log_trace(LogSeverity::Error, message);
    }

    pub fn log(&self, level: LogLevel, severity: LogSeverity, message: &str) {
        MasterLogger::log_to_console(&self.source, &level, &severity, message);
        MasterLogger::log_to_file(&self.source, &level, &severity, message);
    }
}