use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use serde_json::json;

use crate::protocol::command::Command;
use crate::protocol::response::Response;
use crate::state::game::GameState;
use crate::state::player::Player;
use tracing::info;

pub async fn dispatch(
    cmd: Command,
    addr: &str,
    tx: &mpsc::UnboundedSender<Response>,
    state: Arc<RwLock<GameState>>,
) -> Response {
    match cmd {
        Command::Connect { name } => {
            let mut state = state.write().await;
            if state.players.contains_key(&name) {
                Response::error(409, "Name already taken")
            } else {
                state.players.insert(name.clone(), Player {
                    name: name.clone(),
                    addr: addr.to_string(),
                    room: "start".to_string(),
                    tx: tx.clone(),
                });

                state.broadcast_room(
                    "start",
                    Some(&name),
                    Response::ok("event", json!({
                        "event": "player_joined",
                        "name": name,
                    })),
                );

                info!("Player '{}' joined", name);
                Response::ok("connect", json!({ "name": name }))
            }
        }

        Command::Chat { text } => {
            let state = state.read().await;
            let speaker = state.players.values().find(|p| p.addr == addr);
            match speaker {
                Some(p) => {
                    let room = p.room.clone();
                    let from = p.name.clone();
                    state.broadcast_room(
                        &room,
                        Some(&from),
                        Response::ok("event", json!({
                            "event": "said",
                            "from": from,
                            "text": text,
                        })),
                    );
                    Response::ok("chat", json!({ "text": text }))
                }
                None => Response::error(403, "Connect first"),
            }
        }

        Command::Who => {
            let state = state.read().await;
            let names: Vec<&String> = state.players.keys().collect();
            Response::ok("who", json!({ "players": names }))
        }

        Command::Look => Response::ok("look", json!({
            "description": "You are in a dark room.",
            "exits": ["north"],
        })),

        Command::Group { action: _ } => Response::ok("createGroup", json!({
            "description": "Try to create group.",
        })),

        Command::Unknown(raw) => Response::error(404, format!("Unknown command: {}", raw)),
    }
}