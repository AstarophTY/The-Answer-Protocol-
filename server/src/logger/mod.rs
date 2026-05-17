mod files;
mod time;

use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use files::DailyJsonFiles;
use time::{ClockTime, IsoTime};

pub fn init() {
    dotenvy::dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let console = fmt::layer().with_target(false).with_timer(ClockTime);

    let json_files = fmt::layer()
        .json()
        .flatten_event(true)
        .with_target(false)
        .with_timer(IsoTime)
        .with_writer(DailyJsonFiles);

    tracing_subscriber::registry()
        .with(filter)
        .with(console)
        .with(json_files)
        .init();
}
