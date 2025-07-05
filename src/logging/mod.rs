use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum LogLevel {
    INFO,
    WARN,
    ERROR,
    DEBUG,
}

pub struct Logger {
    name: String,
}

impl Logger {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }

    pub fn log(&self, level: LogLevel, msg: &str) {
        let timestamp = Self::current_timestamp();
        println!(
            "[{}] [{}] [{}] {}",
            timestamp,
            Self::level_to_str(&level),
            self.name,
            msg
        );
    }

    pub fn info(&self, msg: &str) {
        self.log(LogLevel::INFO, msg);
    }

    pub fn warn(&self, msg: &str) {
        self.log(LogLevel::WARN, msg);
    }

    pub fn error(&self, msg: &str) {
        self.log(LogLevel::ERROR, msg);
    }

    pub fn debug(&self, msg: &str) {
        self.log(LogLevel::DEBUG, msg);
    }

    fn level_to_str(level: &LogLevel) -> &'static str {
        match level {
            LogLevel::INFO => "INFO",
            LogLevel::WARN => "WARN",
            LogLevel::ERROR => "ERROR",
            LogLevel::DEBUG => "DEBUG",
        }
    }

    fn current_timestamp() -> String {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        format!("{:>10}.{:03}", now.as_secs(), now.subsec_millis())
    }
}
