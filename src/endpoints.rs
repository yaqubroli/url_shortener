use actix_files::NamedFile;
use std::path::PathBuf;
use actix_web::{web, HttpResponse, HttpRequest, Responder};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Submission {
    url: String
}

pub async fn file(req: HttpRequest) -> Result<NamedFile, actix_web::Error> {
    let path_string = req.match_info().query("filename");
    let path: PathBuf = if path_string != "" {
        PathBuf::from(path_string)
    } else {
        PathBuf::from("index.html")
    };
    Ok(NamedFile::open(path)?)
}

pub async fn hello(req: HttpRequest) -> impl Responder {
    //println!("HTTP Host Address: {:?}", req.app_data::<AppData>().unwrap().config.http.host);
    HttpResponse::Ok().body("Hello world!")
}

pub async fn submit_url(form: web::Form<Submission>) -> impl Responder {
    println!("Received submission: {:?}", form);
    HttpResponse::Ok().body(format!("Received submission: {:?}", form))
}