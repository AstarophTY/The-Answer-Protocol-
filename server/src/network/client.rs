use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

use crate::protocol::command::Command;
use crate::state::game::{GameState, Player};
use crate::{debug, info};

pub async fn handle(socket: TcpStream, addr: String, state: Arc<RwLock<GameState>>) {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    while let Ok(Some(line)) = lines.next_line().await {
        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        debug!("[{}] << {}", addr, line);

        let response = match Command::parse(&line) {
            Ok(cmd) => dispatch(cmd, &addr, Arc::clone(&state)).await,
            Err(e) => format!("ERR {}\n", e),
        };

        debug!("[{}] >> {}", addr, response.trim_end());

        if writer.write_all(response.as_bytes()).await.is_err() {
            break;
        }
    }

    info!("Connection closed: {}", addr);

    let mut state = state.write().await;
    state.players.retain(|p| p.addr != addr);
}

async fn dispatch(cmd: Command, addr: &str, state: Arc<RwLock<GameState>>) -> String {
    match cmd {
        Command::Connect { name } => {
            let mut state = state.write().await;
            if state.players.iter().any(|p| p.name == name) {
                "ERR Name already taken\n".to_string()
            } else {
                state.players.push(Player {
                    name: name.clone(),
                    addr: addr.to_string(),
                    room: "start".to_string(),
                });
                info!("Player '{}' joined", name);
                format!("OK CONNECT {}\n", name)
            }
        }

        Command::Who => {
            let state = state.read().await;
            let names: Vec<&str> = state.players.iter().map(|p| p.name.as_str()).collect();
            if names.is_empty() {
                "WHO (none)\n".to_string()
            } else {
                format!("WHO {}\n", names.join(","))
            }
        }

        Command::Look => {
            "LOOK You are in a dark room. Exits: north\n".to_string()
        }

        Command::Unknown(raw) => {
            format!("ERR Unknown command: {}\n", raw)
        }
    }
}
