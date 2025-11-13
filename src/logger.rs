use core::fmt;
use std::{env, sync::Mutex, io::Write};

use once_cell::sync::Lazy;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Error = 0,
    Warning = 1,
    Info = 2,
    Debug = 3,
}

impl From<i32> for LogLevel {
    fn from(value: i32) -> Self {
        match value {
            0 => LogLevel::Error,
            1 => LogLevel::Warning,
            2 => LogLevel::Info,
            3 => LogLevel::Debug,
            _ => LogLevel::Debug,
        }
    }
}

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Warning => write!(f, "WARNING"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Debug => write!(f, "DEBUG"),
        }
    }
}

pub trait LogSource {
    fn get_source(&self) -> String;
}

impl LogLevel {
    pub fn at_or_under(&self, level: &LogLevel) -> bool {
        *self <= *level
    }
}

pub trait Log: LogSource + Sized {
    fn log_error(&self, message: &str) {
        Logger::log(self, LogLevel::Error, message);
    }

    fn log_warning(&self, message: &str) {
        Logger::log(self, LogLevel::Warning, message);
    }

    fn log_info(&self, message: &str) {
        Logger::log(self, LogLevel::Info, message);
    }

    fn log_debug(&self, message: &str) {
        Logger::log(self, LogLevel::Debug, message);
    }
}

static MASTER_LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new())); 

pub struct Logger {
    console_log_level: LogLevel,
    file_log_level: LogLevel,
    log_file_handle: Option<std::fs::File>,
}

impl Logger {
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
        env::var(variable_name).map_or_else(|_| LogLevel::Debug, |log_level| {
            log_level.parse::<i32>().map(|log_level| log_level.into()).unwrap_or(LogLevel::Debug)
        })
    }

    fn new() -> Logger {
        Logger {
            console_log_level: Self::get_log_level("CONSOLE_LOG_LEVEL"),
            file_log_level: Self::get_log_level("FILE_LOG_LEVEL"),
            log_file_handle: None,
        }
    }

    fn log_to_console(reporter: &impl LogSource, level: &LogLevel, message: &str) {
        let logger = MASTER_LOGGER.lock().unwrap();

        if level.at_or_under(&logger.console_log_level) {
            println!("[{} - {}] {}", reporter.get_source(), level, message);
        }
    }

    fn log_to_file(reporter: &impl LogSource, level: &LogLevel, message: &str) {
        let mut logger = MASTER_LOGGER.lock().unwrap();

        if level.at_or_under(&logger.file_log_level) {
            let log_file_handle = logger.log_file_handle.as_mut().expect("Log file handle not initialized");

            writeln!(log_file_handle, "[{} - {}] {}", reporter.get_source(), level, message).expect("Error writing to log file");
        }
    }

    pub fn log(reporter: &impl LogSource, level: LogLevel, message: &str) {
        Logger::log_to_console(reporter, &level, &message);
        Logger::log_to_file(reporter, &level, &message);
    }

    pub fn log_error(reporter: &impl LogSource, message: &str) {
        Logger::log(reporter, LogLevel::Error, message);
    }

    pub fn log_warning(reporter: &impl LogSource, message: &str) {
        Logger::log(reporter, LogLevel::Warning, message);
    }

    pub fn log_info(reporter: &impl LogSource, message: &str) {
        Logger::log(reporter, LogLevel::Info, message);
    }

    pub fn log_debug(reporter: &impl LogSource, message: &str) {
        Logger::log(reporter, LogLevel::Debug, message);
    }
}
