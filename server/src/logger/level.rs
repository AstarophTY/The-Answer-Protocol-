pub struct LogLevel {
    pub name: &'static str,
    pub color: &'static str,
    pub priority: u8,
    pub is_error: bool,
}

pub const INFO: LogLevel = LogLevel {
    name: "INFO",
    color: "\x1b[32m",
    priority: 1,
    is_error: false,
};

pub const WARNING: LogLevel = LogLevel {
    name: "WARNING",
    color: "\x1b[33m",
    priority: 2,
    is_error: false,
};

pub const ERROR: LogLevel = LogLevel {
    name: "ERROR",
    color: "\x1b[31m",
    priority: 3,
    is_error: true,
};

pub const DEBUG: LogLevel = LogLevel {
    name: "DEBUG",
    color: "\x1b[36m",
    priority: 0,
    is_error: false,
};