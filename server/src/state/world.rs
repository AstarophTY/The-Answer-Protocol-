use std::collections::HashMap;

use crate::config;

pub struct WorldState {
    room_items: HashMap<String, Vec<String>>,
}

impl WorldState {
    pub fn from_config() -> Self {
        let cfg = config::get();
        let mut room_items = HashMap::new();
        for (room_id, loc) in &cfg.world.locations {
            room_items.insert(room_id.clone(), loc.items.clone());
        }
        WorldState { room_items }
    }

    pub fn items_in(&self, room: &str) -> &[String] {
        self.room_items.get(room).map(|v| v.as_slice()).unwrap_or(&[])
    }

    pub fn remove_item(&mut self, room: &str, item_id: &str) -> bool {
        if let Some(items) = self.room_items.get_mut(room) {
            if let Some(idx) = items.iter().position(|i| i == item_id) {
                items.remove(idx);
                return true;
            }
        }
        false
    }

    pub fn add_item(&mut self, room: &str, item_id: String) {
        self.room_items.entry(room.to_string()).or_default().push(item_id);
    }
}
