use chrono::Local;
use std::io::{self, Write};

use super::level::LogLevel;
use super::config::get_log_level;

pub fn log(level: LogLevel, msg: &str) {
    if get_log_level() > level.priority {
        return ;
    }

    let time = Local::now().format("%H:%M:%S");

    let output = format!(
        "[{}] [{}{}\x1b[0m] {}\n",
        time,
        level.color,
        level.name,
        msg
    );

    let _ = if level.is_error {
        io::stderr().write_all(output.as_bytes())
    } else {
        io::stdout().write_all(output.as_bytes())
    };
}