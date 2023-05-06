use std::sync::{Arc, RwLock};

use actix::{Actor, StreamHandler};
use actix_web::{web, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use log::info;

use crate::{AppState, Packet, ArtistTurtleInformation, ArtistInventoryInformation};

pub struct WebsocketInstance {
    state: web::Data<Arc<RwLock<AppState>>>,
}

impl Actor for WebsocketInstance {
    type Context = ws::WebsocketContext<Self>;
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebsocketInstance {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => {
                let deserialized: serde_json::Value = serde_json::from_str(&text).unwrap();
                let packet_type: Packet =
                    serde_json::from_value(deserialized.get("packet_type").unwrap().clone())
                        .unwrap();

                match packet_type {
                    Packet::TurtleConnect => {
                        let turtle_information: ArtistTurtleInformation = serde_json::from_value(
                            deserialized.get("turtle_information").unwrap().clone(),
                        )
                        .unwrap();

                        info!(
                            "Turtle `{}` with id `{}` has connected",
                            turtle_information.name, turtle_information.id
                        );

                        self.state.write().unwrap().artist.turtle_information.name = turtle_information.name;
                        self.state.write().unwrap().artist.turtle_information.id = turtle_information.id;

                        ctx.text("done")
                    }
                    Packet::FurnaceUpdate => {}
                    Packet::InventoryUpdate => {}
                    Packet::InventoryPeripheralsUpdate => {
                        let inventory_information: ArtistInventoryInformation = serde_json::from_value(
                            deserialized.get("inventory").unwrap().clone(),
                        )
                        .unwrap();

                        self.state.write().unwrap().artist.inventory_information.used_slots = inventory_information.used_slots;
                        self.state.write().unwrap().artist.inventory_information.full_slots = inventory_information.full_slots;
                        self.state.write().unwrap().artist.inventory_information.total_slots = inventory_information.total_slots;
                        self.state.write().unwrap().artist.inventory_information.slots = serde_json::to_value(inventory_information.slots).unwrap();
                    }
                }

                //ctx.text(text)
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}

pub async fn websocket_index(
    req: HttpRequest,
    stream: web::Payload,
    state: web::Data<Arc<RwLock<AppState>>>,
) -> Result<HttpResponse, actix_web::Error> {
    let resp = ws::start(WebsocketInstance { state }, &req, stream);
    //println!("{:?}", resp);

    resp
}
