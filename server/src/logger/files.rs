use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};

use chrono::Local;
use tracing::{Level, Metadata};
use tracing_subscriber::fmt::MakeWriter;

struct OpenFiles {
    date: String,
    stdout: File,
    stderr: File,
}

fn open_files() -> &'static Mutex<Option<OpenFiles>> {
    static FILES: OnceLock<Mutex<Option<OpenFiles>>> = OnceLock::new();
    FILES.get_or_init(|| Mutex::new(None))
}

fn open_for(date: &str) -> io::Result<OpenFiles> {
    let dir = PathBuf::from("logs").join(date);
    fs::create_dir_all(&dir)?;
    let append = |name: &str| OpenOptions::new().create(true).append(true).open(dir.join(name));
    Ok(OpenFiles {
        date: date.to_string(),
        stdout: append("stdout.jsonl")?,
        stderr: append("stderr.jsonl")?,
    })
}

pub(crate) struct DailyWriter {
    error: bool,
}

impl Write for DailyWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut guard = open_files().lock().unwrap_or_else(|e| e.into_inner());
        let today = Local::now().format("%Y-%m-%d").to_string();
        if guard.as_ref().map_or(true, |f| f.date != today) {
            *guard = Some(open_for(&today)?);
        }
        let files = guard.as_mut().expect("opened just above");
        if self.error {
            files.stderr.write(buf)
        } else {
            files.stdout.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        let mut guard = open_files().lock().unwrap_or_else(|e| e.into_inner());
        if let Some(files) = guard.as_mut() {
            files.stdout.flush()?;
            files.stderr.flush()?;
        }
        Ok(())
    }
}

pub(crate) struct DailyJsonFiles;

impl<'a> MakeWriter<'a> for DailyJsonFiles {
    type Writer = DailyWriter;

    fn make_writer(&'a self) -> Self::Writer {
        DailyWriter { error: false }
    }

    fn make_writer_for(&'a self, meta: &Metadata<'_>) -> Self::Writer {
        DailyWriter { error: *meta.level() == Level::ERROR }
    }
}
