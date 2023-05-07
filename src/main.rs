use std::sync::{Arc, RwLock};

use actix_files::Files;
use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use tera::Tera;

use artist_webui::{
    api, websocket, AppState, Artist, ArtistFurnaceInformation, ArtistInventoryInformation,
    ArtistTurtleInformation,
};

#[get("/")]
async fn hello(state: web::Data<AppState>) -> impl Responder {
    let template = &state.templates;

    let ctx = tera::Context::new();

    let body = template
        .render("index.html.tera", &ctx)
        .map_err(|_| actix_web::error::ErrorInternalServerError("Template error"))
        .unwrap();

    HttpResponse::Ok().content_type("text/html").body(body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let templates = Tera::new("./templates/**/*").unwrap();

    let inventory_information = ArtistInventoryInformation {
        used_slots: 0,
        full_slots: 0,
        total_slots: 0,

        slots: Default::default(),
    };

    let tutel_information = ArtistTurtleInformation {
        name: String::from("No Name"),
        id: -1,
    };

    let furnace_information = ArtistFurnaceInformation {
        hot_furnaces: 0,
        cold_furnaces: 0,
    };

    let artist = Artist {
        turtle_information: tutel_information,
        inventory_information,
        furnace_information,
    };

    let state = Arc::new(RwLock::new(AppState { templates, artist }));

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(state.clone()))
            .service(Files::new("/static", "./static"))
            .route("/ws", web::get().to(websocket::websocket_index))
            .service(hello)
            .service(
                web::scope("/api")
                    .service(api::turtle_information)
                    .service(api::artist_information),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
