use actix_settings::{BasicSettings, ApplySettings};
use actix_web::{web, App, HttpServer, middleware::Condition, middleware::Logger, middleware::Compress};
use mysql::Pool;

pub mod database;
pub mod endpoints;
pub mod settings;
pub mod shortener;
pub mod url;
pub mod templating;
pub mod full_path;

/*
TODO:
 * - clean up code
 * - add logging
*/

#[derive(Clone)]
pub struct AppData {
    config: BasicSettings<settings::AppSettings>,
    database: Pool
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Importing settings...");
    let settings = settings::init();
    println!("Starting server...");
    let server = HttpServer::new({
        let app_data = AppData {
            config: settings.clone(),
            database: database::init(&settings).await
        };
        move || {
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
                .app_data(web::Data::new(app_data.clone()))
                .route("/", web::get().to(endpoints::index))
                .route("/static/{filename}", web::get().to(endpoints::static_file))
                .route("/{shortened}", web::get().to(endpoints::serve_entry))
                .route("/", web::post().to(endpoints::submit_entry))
        }
    })
    .apply_settings(&settings)
    .run();
    println!("Server started!");
    server.await
}