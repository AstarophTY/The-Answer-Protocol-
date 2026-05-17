use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Spawn {
    pub npc_type: String,
    pub count: u32,
}
