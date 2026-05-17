mod logger;
mod network;
mod protocol;
mod state;

use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

const LISTEN_ADDR: &str = "127.0.0.1:4000";

#[tokio::main]
async fn main() {
    logger::init();
    info!("Server starting...");

    let game_state = Arc::new(RwLock::new(state::game::GameState::new()));

    network::listener::start(LISTEN_ADDR, game_state).await;
}
