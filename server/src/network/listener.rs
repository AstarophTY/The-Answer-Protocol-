use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

use crate::state::game::GameState;
use tracing::{error, info};

pub async fn start(addr: &str, state: Arc<RwLock<GameState>>) {
    let listener = TcpListener::bind(addr).await.expect("Failed to bind port");
    info!("Listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((socket, peer_addr)) => {
                info!("New connection from {}", peer_addr);
                let state = Arc::clone(&state);
                tokio::spawn(async move {
                    super::handle::handle(socket, peer_addr.to_string(), state).await;
                });
            }
            Err(e) => {
                error!("Accept error: {}", e);
            }
        }
    }
}
