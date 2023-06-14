use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    response::IntoResponse,
};
use futures::{stream::StreamExt, SinkExt};
use std::sync::{Arc, Mutex};

use crate::{
    json::{JsonResponse, PacketType},
    AppState,
};

use crate::json::{JsonData, PacketTypeStr};

pub async fn websocket_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<Mutex<AppState>>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| websocket(socket, state))
}

pub async fn websocket(stream: WebSocket, state: Arc<Mutex<AppState>>) {
    // By splitting, we can send and receive at the same time.
    let (mut sender, mut receiver) = stream.split();

    while let Some(Ok(message)) = receiver.next().await {
        if let Message::Text(message) = message {
            let json_data: JsonData = serde_json::from_str(&message).unwrap();

            match json_data.packet {
                PacketTypeStr::TurtleConnnect => {
                    let packet_type = PacketType::get_type(json_data.packet, json_data.data);

                    if let PacketType::TurtleConnect(data) = packet_type {
                        tracing::info!("{} {}", data.name, data.id);

                        // TODO: Check if the id is already set or not.
                        state.lock().unwrap().turtle.name = data.name;
                        state.lock().unwrap().turtle.id = data.id;

                        let response = JsonResponse {
                            success: true,
                            message: String::from("successfully connected!"),
                        };

                        let _ = sender
                            .send(Message::Text(serde_json::to_string(&response).unwrap()))
                            .await;
                    }
                }
                PacketTypeStr::ArtistFurnaceUpdate => {
                    let packet_type = PacketType::get_type(json_data.packet, json_data.data);

                    if let PacketType::ArtistFurnaceUpdate(data) = packet_type {
                        tracing::info!(
                            "hot = {:?}; cold = {:?}",
                            data.hot_furnaces,
                            data.cold_furnaces
                        );

                        state.lock().unwrap().artist.furnaces.cold_furnaces = data.cold_furnaces;
                        state.lock().unwrap().artist.furnaces.hot_furnaces = data.hot_furnaces;

                        let response = JsonResponse {
                            success: true,
                            message: String::from("Set new values for furnaces"),
                        };

                        let _ = sender
                            .send(Message::Text(serde_json::to_string(&response).unwrap()))
                            .await;
                    }
                }
                PacketTypeStr::ArtistInventoryUpdate => {
                    let packet_type = PacketType::get_type(json_data.packet, json_data.data);

                    if let PacketType::ArtistInventoryUpdate(data) = packet_type {

                        state.lock().unwrap().artist.inventory.full_slots = data.full_slots;
                        state.lock().unwrap().artist.inventory.used_slots = data.used_slots;
                        state.lock().unwrap().artist.inventory.slots = data.slots;

                        let response = JsonResponse {
                            success: true,
                            message: String::from("Set new values for inventory"),
                        };

                        let _ = sender
                            .send(Message::Text(serde_json::to_string(&response).unwrap()))
                            .await;
                    }
                }
            }
        }
    }
}
