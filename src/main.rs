#![allow(unused_assignments)] 
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]

use actix_web::{get, post, web, App, HttpResponse, HttpRequest, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct Url {
    url: String,
}

#[get("/submit")]
async fn submit(target: web::Form<Url>) -> impl Responder {
    HttpResponse::Ok().body(&target.url)
}

#[get("/{test}")]
async fn url(path: web::Path<String,>) -> impl Responder {
    HttpResponse::Ok().body(path.into_inner())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(submit)
            .service(url)
    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
}