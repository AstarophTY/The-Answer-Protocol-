use std::sync::Arc;

use serde_json::json;
use tokio::sync::{mpsc, RwLock};
use tracing::info;

use crate::protocol::response::Response;
use crate::state::game::{GameState, GroupLeave};
use crate::state::player::Player;

pub async fn connect(
    name: String,
    addr: &str,
    tx: &mpsc::UnboundedSender<Response>,
    state: Arc<RwLock<GameState>>,
) -> Response {
    let mut state = state.write().await;

    if state.players.contains_key(&name) {
        return Response::error(201, "NAME_IN_USE");
    }

    let player = Player::new(name.clone(), addr.to_string(), tx.clone());
    let room = player.room.clone();
    state.players.insert(name.clone(), player);

    state.broadcast_room(
        &room,
        Some(&name),
        Response::ok(
            "event",
            json!({ "event": "presence_enter", "name": name }),
        ),
    );

    info!(player = %name, "Player joined");
    Response::ok("connect", json!({ "name": name }))
}

pub async fn disconnect(addr: &str, state: Arc<RwLock<GameState>>) {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return,
    };
    let room = state.players[&name].room.clone();

    match state.leave_group(&name) {
        GroupLeave::NotInGroup => {}
        GroupLeave::Left { remaining, .. } => {
            let msg = Response::ok(
                "event",
                json!({ "event": "group_leave", "name": name }),
            );
            for m in &remaining {
                state.send_to(m, msg.clone());
            }
        }
        GroupLeave::Disbanded { members, .. } => {
            let msg = Response::ok(
                "event",
                json!({ "event": "group_disband", "by": name }),
            );
            for m in &members {
                if m != &name {
                    state.send_to(m, msg.clone());
                }
            }
        }
    }

    state.players.remove(&name);

    state.broadcast_room(
        &room,
        None,
        Response::ok(
            "event",
            json!({ "event": "presence_leave", "name": name }),
        ),
    );

    info!(player = %name, "Player disconnected");
}

pub async fn who(state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;
    let names: Vec<&String> = state.players.keys().collect();
    Response::ok(
        "who",
        json!({ "players": names, "count": names.len() }),
    )
}
