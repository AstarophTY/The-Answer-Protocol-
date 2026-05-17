use std::sync::Arc;

use tokio::sync::{mpsc, RwLock};

use crate::network::handlers::{chat, group, inventory, session, world};
use crate::protocol::command::Command;
use crate::protocol::response::Response;
use crate::state::game::GameState;

pub async fn dispatch(
    cmd: Command,
    addr: &str,
    tx: &mpsc::UnboundedSender<Response>,
    state: Arc<RwLock<GameState>>,
) -> Response {
    match cmd {
        Command::Connect { name } => session::connect(name, addr, tx, state).await,
        Command::Who => session::who(state).await,
        Command::Look => world::look(addr, state).await,
        Command::Chat { scope, text } => chat::chat(scope, text, addr, state).await,
        Command::Take { item } => inventory::take(item, addr, state).await,
        Command::Drop { item } => inventory::drop_item(item, addr, state).await,
        Command::Inventory => inventory::inventory(addr, state).await,
        Command::Group(action) => group::group(action, addr, state).await,
        Command::Unknown(raw) => {
            Response::error(404, format!("Unknown command: {}", raw))
        }
    }
}
