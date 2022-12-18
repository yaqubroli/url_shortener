use actix_web::{web, App, HttpServer, middleware::Condition, middleware::Logger, middleware::Compress};
// use mysql::Pool;

pub mod database;
pub mod endpoints;
pub mod settings;

/*
TODO:
 * - make config accessible universally, or somehow transfer config data to how functions work
 * - make database accessible universally
 * - make shortened url generation
 * - make shortened url redirect
 * - implement other functions (pastebin maybe?)
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let settings = settings::init();
    HttpServer::new(move || {
        let settings = settings.clone();
        App::new()
            // Grab compression settings from config.toml
            .wrap(Condition::new(
                settings.actix.enable_compression,
                Compress::default(),
            ))
            // Grab logger settings from config.toml
            .wrap(Condition::new(
                settings.actix.enable_log,
                Logger::default(),
            ))
            .app_data(web::Data::new(settings.clone()))
            .route("/", web::get().to(endpoints::hello))
            .route("/{filename:.*}", web::get().to(endpoints::file))
            .route("/", web::post().to(endpoints::submit_url))
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}