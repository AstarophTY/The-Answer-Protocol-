use tokio::sync::mpsc::UnboundedSender;

use crate::protocol::response::Response;
use crate::state::group::GroupId;

pub struct Player {
    pub name: String,
    pub addr: String,
    pub room: String,
    pub tx: UnboundedSender<Response>,
    pub inventory: Vec<String>,
    pub group: Option<GroupId>,
}

impl Player {
    pub fn new(name: String, addr: String, tx: UnboundedSender<Response>) -> Self {
        Player {
            name,
            addr,
            room: "start".to_string(),
            tx,
            inventory: Vec::new(),
            group: None,
        }
    }

    pub fn take_from_inventory(&mut self, item_id: &str) -> bool {
        if let Some(idx) = self.inventory.iter().position(|i| i == item_id) {
            self.inventory.remove(idx);
            true
        } else {
            false
        }
    }
}
