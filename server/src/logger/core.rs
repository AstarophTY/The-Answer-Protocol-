use chrono::{Local, SecondsFormat};
use serde_json::json;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use super::level::LogLevel;
use super::config::get_log_level;

struct LogFiles {
    date: String,
    stdout: std::fs::File,
    stderr: std::fs::File,
}

static LOG_FILES: OnceLock<Mutex<Option<LogFiles>>> = OnceLock::new();

fn open_log_files(date: &str) -> io::Result<LogFiles> {
    let dir = PathBuf::from("logs").join(date);
    fs::create_dir_all(&dir)?;

    let stdout = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("stdout.jsonl"))?;
    let stderr = OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("stderr.jsonl"))?;

    Ok(LogFiles {
        date: date.to_string(),
        stdout,
        stderr,
    })
}

fn with_log_files<F>(date: &str, f: F) -> io::Result<()>
where
    F: FnOnce(&mut LogFiles) -> io::Result<()>,
{
    let lock = LOG_FILES.get_or_init(|| Mutex::new(None));
    let mut guard = match lock.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    };

    let needs_open = match guard.as_ref() {
        Some(files) => files.date != date,
        None => true,
    };

    if needs_open {
        *guard = Some(open_log_files(date)?);
    }

    if let Some(files) = guard.as_mut() {
        f(files)?;
    }

    Ok(())
}

pub fn log(level: LogLevel, msg: &str) {
    if get_log_level() > level.priority {
        return ;
    }

    let now = Local::now();
    let timestamp = now.to_rfc3339_opts(SecondsFormat::Millis, true);
    let date = now.format("%Y-%m-%d").to_string();

    let record = json!({
        "timestamp": timestamp,
        "level": level.name,
        "event": "message",
        "message": msg,
    });
    let mut output = record.to_string();
    output.push('\n');

    let terminal_output = format!(
        "[{}] [{}{}\x1b[0m] {}\n",
        now.format("%H:%M:%S"),
        level.color,
        level.name,
        msg
    );

    let _ = with_log_files(&date, |files| {
        if level.is_error {
            files.stderr.write_all(output.as_bytes())
        } else {
            files.stdout.write_all(output.as_bytes())
        }
    });

    let _ = if level.is_error {
        io::stderr().write_all(terminal_output.as_bytes())
    } else {
        io::stdout().write_all(terminal_output.as_bytes())
    };
}