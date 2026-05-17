use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Item {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub obtainable: bool,
}
