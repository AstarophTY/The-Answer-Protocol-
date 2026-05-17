use std::collections::HashMap;

use serde::Deserialize;

use super::item::Item;
use super::location::Location;
use super::npc::Npc;

#[derive(Debug, Deserialize)]
pub struct WorldConfig {
    pub world: WorldData,
}

#[derive(Debug, Deserialize, Default)]
pub struct WorldData {
    #[serde(default)]
    pub locations: HashMap<String, Location>,
    #[serde(default)]
    pub items: HashMap<String, Item>,
    #[serde(default)]
    pub npcs: HashMap<String, Npc>,
}
