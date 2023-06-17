use std::sync::Arc;
use std::sync::Mutex;

use axum::http::Method;
use axum::{response::Html, routing::get, Router};
use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
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
        ready: artist_webui_axum::ReadyState::NotConnected,
    }));


    let cors = CorsLayer::new()
        // allow `GET` and `POST` when accessing the resource
        .allow_methods([Method::GET, Method::POST])
        // allow requests from any origin
        .allow_origin(Any);

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
                )
                .route("/status", get(api::status)),
        )
        .with_state(app_state)
        .layer(cors);

    axum::Server::bind(&"0.0.0.0:8080".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// for now
async fn index() -> Html<&'static str> {
    Html("no data for u")
}
