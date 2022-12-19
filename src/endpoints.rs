use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{database::{self, SubmittedUrl}, shortener::{self, base64}, url, templating};

#[derive(Deserialize, Serialize, Debug)]
pub struct Submission {
    url: String
}
// write a version of the above function, but without using NamedFile and using standard rust fs libraries instead
pub async fn static_file(path: web::Path<String>) -> impl Responder {
    let path_string = path.into_inner();
    println!("Accessing file {:?}", path_string);
    let file = std::fs::read_to_string(PathBuf::from(format!("static/{}", path_string))).unwrap();
    HttpResponse::Ok().body(file)
}

/* -> Result<NamedFile, actix_web::Error> {
    let path_string = path.into_inner();
    println!("Accessing file {:?}", path_string);
    Ok(NamedFile::open(PathBuf::from(format!("static/{}", path_string)))?)
} */

pub async fn index() -> Result<NamedFile, actix_web::Error> {
    Ok(NamedFile::open("index.html")?)
}
pub async fn submit_url(form: web::Form<Submission>, app_data: web::Data<crate::AppData>) -> impl Responder {
    let url = url::format_url(form.url.clone());
    let mut connection = app_data.database.get_conn().unwrap();
    let count = database::count_urls(&mut connection).await;
    for n in 0..3{
        let shortened = shortener::base64(count);
        let submitted_url = database::submit_url(&mut connection, &url, &shortened).await;
        if submitted_url.success {
            return HttpResponse::Ok().body(
                templating::read_and_apply_templates(
                    PathBuf::from("results.html"),
                    templating::TemplateSchema {
                        url: url,
                        shortened: submitted_url.shortened,
                        domain: app_data.config.application.templating.domain.clone(),
                        count: count.to_string()
                    }
                )
            );
        }
    }
    HttpResponse::InternalServerError().body("An error occured while submitting your URL")
}

// Takes a shortened URL and redirects to the original URL
pub async fn redirect_url(path: web::Path<(String)>, app_data: web::Data<crate::AppData>) -> impl Responder {
    println!("Redirect request recieved to {:?}", path);
    let (shortened) = path.into_inner();
    let mut connection = app_data.database.get_conn().unwrap();
    let retrieved_url = database::retrieve_url(&mut connection, &shortened).await;
    println!("Redirecting to {:?}", retrieved_url.url);
    HttpResponse::Found().header("Location", retrieved_url.url).finish()
}