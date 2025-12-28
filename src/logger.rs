use core::fmt;
use std::env;
use std::fs::{File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::Mutex;

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

    fn log(&self, level: LogLevel, targets: &[&dyn LogTarget], message: &str) {
        let source = self.get_source();
        
        for target in targets {
            target.log(level, &source, message);
        }
    }
    
    fn log_error(&self, targets: &[&dyn LogTarget], message: &str) {
        self.log(LogLevel::Error, targets, message);
    }

    fn log_warning(&self, targets: &[&dyn LogTarget], message: &str) {
        self.log(LogLevel::Warning, targets, message);
    }

    fn log_info(&self, targets: &[&dyn LogTarget], message: &str) {
        self.log(LogLevel::Info, targets, message);
    }

    fn log_debug(&self, targets: &[&dyn LogTarget], message: &str) {
        self.log(LogLevel::Debug, targets, message);
    }
}

pub trait LogTarget: Send + Sync {
    fn log(&self, level: LogLevel, source: &str, message: &str);
}

pub static CONSOLE_LOGGER: ConsoleLogger = ConsoleLogger::new();
pub static DYN_CONSOLE_LOGGER: &dyn LogTarget = &CONSOLE_LOGGER;
pub static REF_DYN_CONSOLE_LOGGER: &&dyn LogTarget = &DYN_CONSOLE_LOGGER;

pub struct ConsoleLogger {
    console_mutex: Mutex<()>,
}

impl ConsoleLogger {
    pub const fn new() -> ConsoleLogger {
        ConsoleLogger {
            console_mutex: Mutex::new(()),
        }
    }
}

impl LogTarget for ConsoleLogger {
    fn log(&self, level: LogLevel, source: &str, message: &str) {
        if let Ok(_) = self.console_mutex.lock() {
            println!("[{} - {}] {}", source, level, message);
        }
    }
}

static CURRENT_TIME: Lazy<String> = Lazy::new(|| {
    Local::now().format("%Y%m%d-%H:%M:%S").to_string()
});

pub struct FileLogger {
    log_file_handle: Option<Mutex<File>>,
}

impl FileLogger {
    fn new_log_file_handle(file_name: &Path) -> io::Result<File> {
        let log_file_path = Path::new(env::var("LOG_PATH")
            .unwrap_or("./".to_string()).as_str())
            .join(Path::new(&*CURRENT_TIME))
            .join(Path::new(file_name)).with_extension("log");
        
        println!("Logging to file: {}", log_file_path.display());

        OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(log_file_path)
    }

    pub fn new(file_name: &Path) -> FileLogger {
        let log_file_handle = match Self::new_log_file_handle(file_name) {
            Ok(log_file_handle) => Some(Mutex::new(log_file_handle)),
            Err(e) => {
                println!("Failed to open log file: {} - {}", file_name.display(), e);
                None
            }
        };

        FileLogger {
            log_file_handle,
        }
    }
}

impl LogTarget for FileLogger {
    fn log(&self, level: LogLevel, source: &str, message: &str) {
        if let Some(Ok(mut log_file_handle)) = self.log_file_handle.as_ref().map(|f| f.lock()) {
            writeln!(log_file_handle, "[{} - {}] {}", source, level, message);
        }
    }
}