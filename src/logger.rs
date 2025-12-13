use core::fmt;
use std::{env, fs::{File, OpenOptions}, io::Write, sync::{Mutex, MutexGuard}};

use chrono::Local;
use once_cell::sync::Lazy;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

impl LogLevel {
    pub fn at_or_under(&self, level: &LogLevel) -> bool {
        *self <= *level
    }
}

pub trait Log {
    fn get_source(&self) -> String;

    fn log(&self, level: LogLevel, message: &str) {
        Logger::log(self, level, message);
    }
    
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
    log_file_handle: Option<File>,
}

impl Logger {
    pub fn open() {
        let log_file_directory = env::var("LOG_PATH").unwrap_or("./".to_string());
        let log_file_path = format!("{}/log-{}.log", log_file_directory, Local::now().format("%Y%m%d-%H:%M:%S"));

        println!("Logging to file: {}", log_file_path);

        let log_file_handle = match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)
        {
            Ok(file) => file,
            Err(err) => {
                println!("ERROR - Could not open log file: {}", err);
                return;
            }
        };

        if let Some(mut logger) = Self::safe_aquire_logger_lock() {
            logger.log_file_handle = Some(log_file_handle);
        }
    }

    pub fn close() {
        if let Some(mut logger) = Self::safe_aquire_logger_lock() {
            logger.log_file_handle = None;
        }
    }

    fn safe_aquire_logger_lock() -> Option<MutexGuard<'static, Logger>> {
        if let Ok(logger) = MASTER_LOGGER.lock() {
            Some(logger)
        } else {
            println!("WARNING - Error acquiring logger lock");
            None
        }
    }

    fn get_log_level(variable_name: &str) -> LogLevel {
        if let Ok(log_level) = env::var(variable_name) {
            if let Ok(log_level) = log_level.parse::<i32>() {
                return log_level.into();
            }
        }

        println!("WARNING - {} not set, defaulting to DEBUG", variable_name);
        LogLevel::Debug
    }

    fn new() -> Logger {
        Logger {
            console_log_level: Self::get_log_level("CONSOLE_LOG_LEVEL"),
            file_log_level: Self::get_log_level("FILE_LOG_LEVEL"),
            log_file_handle: None,
        }
    }

    fn log_to_console<T: Log + ?Sized>(reporter: &T, level: &LogLevel, message: &str) {
        if let Some(logger) = Self::safe_aquire_logger_lock(){
            if level.at_or_under(&logger.console_log_level) {
                println!("[{} - {}] {}", reporter.get_source(), level, message);
            }
        }
    }

    fn log_to_file<T: Log + ?Sized>(reporter: &T, level: &LogLevel, message: &str) {
        if let Some(mut logger) = Self::safe_aquire_logger_lock() {
            if level.at_or_under(&logger.file_log_level) {
                if let Some(log_file_handle) = &mut logger.log_file_handle {
                    writeln!(log_file_handle, "[{} - {}] {}", reporter.get_source(), level, message).expect("Error writing to log file");
                } else {
                    println!("WARNING - Log file handle not initialized, cannot log to file");
                }
            }
        }
    }

    fn log<T: Log + ?Sized>(reporter: &T, level: LogLevel, message: &str) {
        Logger::log_to_console(reporter, &level, &message);
        Logger::log_to_file(reporter, &level, &message);
    }
}
