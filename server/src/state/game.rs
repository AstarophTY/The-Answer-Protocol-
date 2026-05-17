use std::collections::HashMap;

pub struct Player {
    pub name: String,
    pub addr: String,
    pub room: String,
}

pub struct GameState {
    pub players: HashMap<String, Player>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: HashMap::new(),
        }
    }
}
