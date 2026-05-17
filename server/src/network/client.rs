use std::sync::Arc;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;
use tokio::sync::RwLock;

use serde_json::json;

use crate::protocol::command::Command;
use crate::protocol::response::Response;
use crate::state::game::{GameState, Player};
use tracing::{debug, info};

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
            Err(e) => e,
        };

        let line_out = response.to_line();
        debug!("[{}] >> {}", addr, line_out.trim_end());

        if writer.write_all(line_out.as_bytes()).await.is_err() {
            break;
        }
    }

    info!("Connection closed: {}", addr);

    let mut state = state.write().await;
    state.players.retain(|_, v| v.addr != addr);
}

async fn dispatch(cmd: Command, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    match cmd {
        Command::Connect { name } => {
            let mut state = state.write().await;
            if state.players.contains_key(&name) {
                Response::error(409, "Name already taken")
            } else {
                let player_name = name.clone();

                state.players.insert(player_name.clone(), Player {
                    name: player_name,
                    addr: addr.to_string(),
                    room: "start".to_string(),
                });

                info!("Player '{}' joined", name);
                Response::ok("connect", json!({ "name": name }))
            }
        }

        Command::Who => {
            let state = state.read().await;
            let names: Vec<&String> = state.players.keys().collect();
            Response::ok("who", json!({ "players": names }))
        }

        Command::Look => Response::ok(
            "look",
            json!({
                "description": "You are in a dark room.",
                "exits": ["north"],
            }),
        ),

        Command::Unknown(raw) => {
            Response::error(404, format!("Unknown command: {}", raw))
        }
    }
}
