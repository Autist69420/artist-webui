use serde::{Deserialize, Serialize};

pub mod api;
pub mod websocket;

#[derive(Debug, Clone)]
pub struct AppState {
    pub templates: tera::Tera,

    pub artist: Artist,
}

#[derive(Deserialize)]
pub enum Packet {
    #[serde(rename = "turtle_connect")]
    TurtleConnect,
    #[serde(rename = "furnace_update")]
    FurnaceUpdate,
    #[serde(rename = "inventory_update")]
    InventoryUpdate,
    #[serde(rename = "inventory_peripherals_update")]
    InventoryPeripheralsUpdate,
}

#[derive(Debug, Clone)]
pub struct Artist {
    pub turtle_information: ArtistTurtleInformation,

    pub furnace_information: ArtistFurnaceInformation,

    pub inventory_information: ArtistInventoryInformation,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArtistTurtleInformation {
    pub name: String,
    pub id: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArtistFurnaceInformation {
    pub cold_furnaces: i32,
    pub hot_furnaces: i32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ArtistInventoryInformation {
    pub used_slots: u32,
    pub full_slots: u32,
    pub total_slots: u32,

    pub slots: serde_json::Value,
}
