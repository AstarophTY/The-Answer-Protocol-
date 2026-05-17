use std::collections::HashMap;

use crate::protocol::response::Response;

pub struct GameState {
    pub players: HashMap<String, super::player::Player>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
        }
    }

    pub fn send_to(&self, name: &str, msg: Response) {
        if let Some(p) = self.players.get(name) {
            let _ = p.tx.send(msg);
        }
    }

    pub fn broadcast_room(&self, room: &str, except: Option<&str>, msg: Response) {
        for p in self.players.values() {
            if p.room == room && Some(p.name.as_str()) != except {
                let _ = p.tx.send(msg.clone());
            }
        }
    }
}
