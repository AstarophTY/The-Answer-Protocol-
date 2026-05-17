use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Npc {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub dialogue: Vec<String>,
    #[serde(default)]
    pub stats: NpcStats,
}

#[derive(Debug, Deserialize, Default)]
pub struct NpcStats {
    #[serde(default)]
    pub hp: i32,
}
