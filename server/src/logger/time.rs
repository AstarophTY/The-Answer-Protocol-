use chrono::{Local, SecondsFormat};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::time::FormatTime;

pub struct ClockTime;
impl FormatTime for ClockTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().format("%H:%M:%S"))
    }
}

pub struct IsoTime;
impl FormatTime for IsoTime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        write!(w, "{}", Local::now().to_rfc3339_opts(SecondsFormat::Millis, true))
    }
}
