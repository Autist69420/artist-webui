use log::info;
use serde::Serialize;
use std::sync::{Arc, RwLock};

use actix_web::{get, web, HttpResponse, Responder};

use crate::{
    AppState, ArtistFurnaceInformation, ArtistInventoryInformation, ArtistTurtleInformation,
};

#[derive(Serialize)]
struct ArtistInformation {
    turtle: ArtistTurtleInformation,
    furnace: ArtistFurnaceInformation,
    inventory: ArtistInventoryInformation,
}

#[get("/turtle")]
pub async fn turtle_information(state: web::Data<Arc<RwLock<AppState>>>) -> impl Responder {
    let artist = &state.read().unwrap().artist;

    info!(
        "{} {}",
        artist.turtle_information.name, artist.turtle_information.id
    );

    "waka waka"
}

#[get("/artist")]
pub async fn artist_information(state: web::Data<Arc<RwLock<AppState>>>) -> impl Responder {
    let artist = &state.read().unwrap().artist;

    let turtle = ArtistTurtleInformation {
        name: artist.turtle_information.name.clone(),
        id: artist.turtle_information.id,
    };

    let inventory = ArtistInventoryInformation {
        full_slots: artist.inventory_information.used_slots,
        used_slots: artist.inventory_information.used_slots,
        total_slots: artist.inventory_information.total_slots,
        slots: artist.inventory_information.slots.clone(),
    };

    let furnace = ArtistFurnaceInformation {
        hot_furnaces: artist.furnace_information.hot_furnaces,
        cold_furnaces: artist.furnace_information.cold_furnaces,
    };

    let information = ArtistInformation {
        inventory,
        furnace,
        turtle,
    };

    HttpResponse::Ok().json(information)
}
