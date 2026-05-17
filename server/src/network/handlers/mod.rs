pub mod chat;
pub mod group;
pub mod inventory;
pub mod session;
pub mod world;

use crate::config;

pub fn resolve_item(query: &str) -> Option<String> {
    let q = query.trim();
    let items = &config::get().world.items;
    items.iter().find_map(|(id, item)| {
        if id.eq_ignore_ascii_case(q) || item.name.eq_ignore_ascii_case(q) {
            Some(id.clone())
        } else {
            None
        }
    })
}
