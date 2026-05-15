pub struct Player {
    pub name: String,
    pub addr: String,
    pub room: String,
}

pub struct GameState {
    pub players: Vec<Player>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            players: Vec::new(),
        }
    }
}
