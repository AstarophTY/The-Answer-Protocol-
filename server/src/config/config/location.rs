use std::collections::HashMap;

use serde::Deserialize;

use super::direction::Direction;
use super::spawn::Spawn;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Location {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub label: Option<String>,
    #[serde(default)]
    pub exits: HashMap<Direction, String>,
    #[serde(default)]
    pub spawns: Vec<Spawn>,
    #[serde(default)]
    pub items: Vec<String>,
}

impl Location {
    #[allow(dead_code)]
    pub fn remove_item(&mut self, item_name: &str) -> bool {
        if let Some(index) = self.items.iter().position(|item| item == item_name) {
            self.items.remove(index);
            true
        } else {
            false
        }
    }
}