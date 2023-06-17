use std::sync::Arc;
use std::sync::Mutex;

use axum::{response::Html, routing::get, Router};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use artist_webui_axum::websocket;
use artist_webui_axum::AppState;

use artist_webui_axum::{ArtistInformation, TurtleInformation};

use artist_webui_axum::api;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "artist_webui=trace".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let turtle_information = TurtleInformation::default();
    let artist_information = ArtistInformation::default();

    let app_state = Arc::new(Mutex::new(AppState {
        turtle: turtle_information,
        artist: artist_information,
    }));

    let app = Router::new()
        .route("/", get(index))
        .route("/ws", get(websocket::websocket_handler))
        .nest(
            "/api",
            Router::new()
                .route(
                    "/artist/inventory",
                    get(api::artist::artist_inventory_slots),
                )
                .route(
                    "/artist/furnaces",
                    get(api::artist::artist_furnace_information),
                ),
        )
        .with_state(app_state);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// for now
async fn index() -> Html<&'static str> {
    Html("no data for u")
}
