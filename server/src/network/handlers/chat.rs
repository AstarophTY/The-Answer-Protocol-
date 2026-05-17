use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;

use crate::protocol::command::ChatScope;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub async fn chat(
    scope: ChatScope,
    text: String,
    addr: &str,
    state: Arc<RwLock<GameState>>,
) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    let (scope_label, event) = match &scope {
        ChatScope::Global => ("global", "chat_global"),
        ChatScope::Room => ("room", "chat_room"),
        ChatScope::Group => ("group", "chat_group"),
    };

    let msg = Response::ok(
        "event",
        json!({
            "event": event,
            "scope": scope_label,
            "from": name,
            "text": text,
        }),
    );

    match scope {
        ChatScope::Global => {
            state.broadcast_all(Some(&name), msg);
        }
        ChatScope::Room => {
            let room = state.players[&name].room.clone();
            state.broadcast_room(&room, Some(&name), msg);
        }
        ChatScope::Group => match state.players[&name].group {
            Some(gid) => state.broadcast_group(gid, Some(&name), msg),
            None => return Response::error(401, "NOT_IN_GROUP"),
        },
    }

    Response::ok(
        "chat",
        json!({ "scope": scope_label, "text": text }),
    )
}
