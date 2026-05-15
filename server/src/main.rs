mod logger;
mod network;
mod protocol;
mod state;

use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::main]
async fn main() {
    info!("Server starting...");

    let game_state = Arc::new(RwLock::new(state::game::GameState::new()));

    network::listener::start("127.0.0.1:4000", game_state).await;
}
