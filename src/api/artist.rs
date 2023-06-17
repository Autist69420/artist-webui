use std::sync::{Arc, Mutex};

use axum::{extract::State, response::IntoResponse, Json};

use crate::AppState;

pub async fn artist_inventory_slots(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let artist_inv_information = state.lock().unwrap().artist.inventory.clone();

    Json(artist_inv_information)
}

pub async fn artist_furnace_information(
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    let artist_furnace_information = state.lock().unwrap().artist.furnaces.clone();

    Json(artist_furnace_information)
}
