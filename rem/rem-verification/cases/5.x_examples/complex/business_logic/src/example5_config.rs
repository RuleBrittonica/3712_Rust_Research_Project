#[derive(Clone, Copy)]
pub enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
}

// Manual PartialEq + Eq for LogLevel
impl core::cmp::PartialEq for LogLevel {
    fn eq(&self, other: &Self) -> bool {
        match (*self, *other) {
            (LogLevel::Error, LogLevel::Error) => true,
            (LogLevel::Warn,  LogLevel::Warn)  => true,
            (LogLevel::Info,  LogLevel::Info)  => true,
            (LogLevel::Debug, LogLevel::Debug) => true,
            _ => false,
        }
    }
}

impl core::cmp::Eq for LogLevel {}


pub struct RawConfig {
    // raw values may be invalid (<= 0 or out of range)
    pub max_connections: i32,
    pub timeout_ms: i32,
    pub log_level_code: i32,
}

pub struct AppConfig {
    pub max_connections: u32,
    pub timeout_ms: u32,
    pub log_level: LogLevel,
}

pub fn normalise_config(raw: &RawConfig) -> AppConfig {
    // max_connections: default 100, clamp to [1, 10_000]
    let mut max_conn = raw.max_connections;
    if max_conn <= 0 {
        max_conn = 100;
    }
    if max_conn < 1 {
        max_conn = 1;
    }
    if max_conn > 10_000 {
        max_conn = 10_000;
    }

    // timeout_ms: default 1000 if <= 0
    let mut timeout = raw.timeout_ms;
    if timeout <= 0 {
        timeout = 1_000;
    }

    // log_level: map code, default Info
    let log_level = if raw.log_level_code == 0 {
        LogLevel::Error
    } else if raw.log_level_code == 1 {
        LogLevel::Warn
    } else if raw.log_level_code == 2 {
        LogLevel::Info
    } else if raw.log_level_code == 3 {
        LogLevel::Debug
    } else {
        LogLevel::Info
    };

    AppConfig {
        max_connections: max_conn as u32,
        timeout_ms: timeout as u32,
        log_level,
    }
}

pub fn run_example() {
    let raw = RawConfig {
        max_connections: -5,
        timeout_ms: 0,
        log_level_code: 7,
    };

    let cfg = normalise_config(&raw);
    assert!(cfg.max_connections == 100);
    assert!(cfg.timeout_ms == 1_000);
    assert!(cfg.log_level == LogLevel::Info);
}
