use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};

pub mod database;
pub mod config;

#[derive(Serialize, Deserialize, Debug)]
struct Submission {
    url: String
}

#[post("/submit")]
async fn submit(form: web::Form<Submission>) -> impl Responder {
    println!("{:?}", form);
    HttpResponse::Ok().body("You submitted something.")
}

#[get("/debug")]
async fn debug(req: HttpRequest) -> impl Responder {
    println!("{:?}", req);
    HttpResponse::Ok().body("You submit'n't something.")
}

#[get("/{test}")]
async fn url(path: web::Path<String,>) -> impl Responder {
    HttpResponse::Ok().body(path.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::retrieve();
    let database_connection= database::init(config.db);
    println!("Creating HTTP server at {}:{}", config.http.host, config.http.port);
    HttpServer::new(|| {
       App::new()
            .service(submit)
            .service(url)
    })
    .bind((config.http.host, config.http.port))?
    .run()
    .await
}