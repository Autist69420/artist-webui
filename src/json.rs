use std::collections::HashMap;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonData {
    pub packet: PacketTypeStr,
    pub data: Value,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PacketTypeStr {
    #[serde(rename = "turt_connect")]
    TurtleConnnect,
    #[serde(rename = "artist_furnace_update")]
    ArtistFurnaceUpdate,
    #[serde(rename = "artist_inventory_update")]
    ArtistInventoryUpdate,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PacketType {
    #[serde(rename = "turt_connect")]
    TurtleConnect(TurtleConnectionData),
    #[serde(rename = "artist_furnace_update")]
    ArtistFurnaceUpdate(ArtistFurnaceUpdateData),
    #[serde(rename = "artist_inventory_update")]
    ArtistInventoryUpdate(ArtistInventoryUpdateData),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TurtleConnectionData {
    pub name: String,
    pub id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistFurnaceUpdateData {
    pub furnaces: Vec<Furnace>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Furnace {
    pub name: String,
    pub cooking: bool,
    pub info: FurnaceInformation,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FurnaceInformation {
    pub input: Option<String>,
    pub output: Option<String>,
    pub fuel: Option<FurnaceFuelInformation>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FurnaceFuelInformation {
    pub name: String,
    pub count: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ArtistInventoryUpdateData {
    pub used_slots: f32,
    pub full_slots: f32,
    pub total_slots: f32,

    pub slots: Vec<Item>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Item {
    pub hash: String,
    pub sources: HashMap<String, i32>,
    pub count: i64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ItemDetails {
    pub name: String,
    pub tags: HashMap<String, Value>,
    #[serde(rename = "rawName")]
    pub raw_name: String,
    #[serde(rename = "itemGroups")]
    pub item_groups: Vec<Value>,
    #[serde(rename = "maxCount")]
    pub max_count: i64,
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct JsonResponse {
    pub success: bool,
    pub message: String,
}

impl PacketType {
    pub fn get_type(packet_type: PacketTypeStr, json_value: Value) -> PacketType {
        match packet_type {
            PacketTypeStr::TurtleConnnect => {
                let name = to_type::<String>(json_value.get("name").unwrap()).unwrap();
                let id = to_type::<i32>(json_value.get("id").unwrap()).unwrap();

                let connection_data = TurtleConnectionData { name, id };

                PacketType::TurtleConnect(connection_data)
            }
            PacketTypeStr::ArtistFurnaceUpdate => {
                let furnaces =
                    to_type::<Vec<Furnace>>(json_value.get("furnaces").unwrap()).unwrap();

                let furnace_data = ArtistFurnaceUpdateData { furnaces };

                PacketType::ArtistFurnaceUpdate(furnace_data)
            }
            PacketTypeStr::ArtistInventoryUpdate => {
                let total_slots = to_type::<f32>(json_value.get("total_slots").unwrap()).unwrap();
                let used_slots = to_type::<f32>(json_value.get("used_slots").unwrap()).unwrap();
                let full_slots = to_type::<f32>(json_value.get("full_slots").unwrap()).unwrap();

                let slots = to_type::<Vec<Item>>(json_value.get("slots").unwrap()).unwrap();

                let inventory_data = ArtistInventoryUpdateData {
                    used_slots,
                    full_slots,
                    total_slots,
                    slots,
                };

                PacketType::ArtistInventoryUpdate(inventory_data)
            }
        }
    }
}

fn to_type<T>(value: &Value) -> Result<T, ()>
where
    T: DeserializeOwned,
{
    let typp: T = serde_json::from_value(value.clone()).unwrap();

    Ok(typp)
}
