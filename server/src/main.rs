mod logger;

fn main() {
    info!("Server starting...");
    warn!("Test warning");
    error!("Test error");
    debug!("Debug info");
}