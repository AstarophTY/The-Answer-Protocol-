use std::sync::Arc;

use serde_json::json;
use tokio::sync::RwLock;
use tracing::info;

use crate::config;
use crate::network::handlers::resolve_item;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub async fn take(query: String, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room = state.players[&name].room.clone();

    let item_id = match resolve_item(&query) {
        Some(id) => id,
        None => return Response::error(404, "ITEM_NOT_FOUND"),
    };

    let present = state
        .world
        .items_in(&room)
        .iter()
        .any(|i| i == &item_id);
    if !present {
        return Response::error(404, "ITEM_NOT_FOUND");
    }

    let obtainable = config::get()
        .world
        .items
        .get(&item_id)
        .map(|i| i.obtainable)
        .unwrap_or(false);
    if !obtainable {
        return Response::error(404, "ITEM_NOT_FOUND");
    }

    state.world.remove_item(&room, &item_id);
    if let Some(p) = state.players.get_mut(&name) {
        p.inventory.push(item_id.clone());
    }

    state.broadcast_room(
        &room,
        Some(&name),
        Response::ok(
            "event",
            json!({ "event": "item_taken", "by": name, "item": item_id }),
        ),
    );

    info!(player = %name, item = %item_id, "Item taken");
    Response::ok("take", json!({ "taken": item_id }))
}

pub async fn drop_item(query: String, addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let mut state = state.write().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };
    let room = state.players[&name].room.clone();

    let item_id = match resolve_item(&query) {
        Some(id) => id,
        None => return Response::error(404, "ITEM_NOT_IN_INVENTORY"),
    };

    let had = state
        .players
        .get_mut(&name)
        .map(|p| p.take_from_inventory(&item_id))
        .unwrap_or(false);
    if !had {
        return Response::error(404, "ITEM_NOT_IN_INVENTORY");
    }

    state.world.add_item(&room, item_id.clone());

    state.broadcast_room(
        &room,
        Some(&name),
        Response::ok(
            "event",
            json!({ "event": "item_dropped", "by": name, "item": item_id }),
        ),
    );

    info!(player = %name, item = %item_id, "Item dropped");
    Response::ok("drop", json!({ "dropped": item_id }))
}

pub async fn inventory(addr: &str, state: Arc<RwLock<GameState>>) -> Response {
    let state = state.read().await;

    let name = match state.name_of(addr) {
        Some(n) => n,
        None => return Response::error(403, "Connect first"),
    };

    let items = state.players[&name].inventory.clone();
    Response::ok("inventory", json!({ "items": items }))
}
