pub mod api;
pub mod json;
pub mod websocket;

use json::{Furnace, Item};
use serde::Serialize;

#[derive(Clone)]
pub struct AppState {
    pub turtle: TurtleInformation,
    pub artist: ArtistInformation,

    pub ready: ReadyState,
}

#[derive(Clone)]
pub enum ReadyState {
    Ready,
    NotConnected,
}

#[derive(Clone)]
pub struct ArtistInformation {
    pub furnaces: Vec<Furnace>,
    pub inventory: ArtistInventoryInformation,
}

#[derive(Clone, Serialize)]
pub struct TurtleInformation {
    pub name: String,
    pub id: i32,
}

#[derive(Clone, Serialize)]
pub struct ArtistInventoryInformation {
    pub used_slots: f32,
    pub full_slots: f32,
    pub total_slots: f32,

    pub slots: Vec<Item>,
}

impl Default for ArtistInformation {
    fn default() -> Self {
        Self {
            furnaces: Default::default(),
            inventory: Default::default(),
        }
    }
}

impl Default for TurtleInformation {
    fn default() -> Self {
        Self {
            name: String::from("No name"),
            id: -1,
        }
    }
}

impl Default for ArtistInventoryInformation {
    fn default() -> Self {
        Self {
            used_slots: Default::default(),
            full_slots: Default::default(),
            total_slots: Default::default(),
            slots: Default::default(),
        }
    }
}
