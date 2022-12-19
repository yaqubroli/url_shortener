use std::path::PathBuf;
use actix_files::NamedFile;
use actix_web::{web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};

use crate::{database, shortener, url, templating, full_path};
use crate::database::ContentType;

#[derive(Deserialize, Serialize, Debug)]
pub struct Submission {
    content: String,
    content_type: ContentType
}

pub async fn static_file(path: web::Path<String>, app_data: web::Data<crate::AppData>) -> impl Responder {
    let path_string = path.into_inner();
    println!("Accessing file {:?}", path_string);
    let body = if app_data.config.application.html.template_static && path_string.ends_with(".html") {
        templating::read_and_apply_templates(
            full_path::get_full_path(&app_data, &path_string, true),
            templating::TemplateSchema {
                content: "".to_string(),
                shortened: "".to_string(),
                domain: app_data.config.application.html.domain.clone(),
                count: if app_data.config.application.html.count {
                    database::count_entries(&mut app_data.database.get_conn().unwrap(), ContentType::All).await.to_string()
                } else {
                    "".to_string()
                }
            }
        )
    } else {
        std::fs::read_to_string(full_path::get_full_path(&app_data, &path_string, true)).unwrap()
    };
    HttpResponse::Ok().body(body)
}

pub async fn index(app_data: web::Data<crate::AppData>) -> impl Responder {
    // serve index.html, templated if enabled
    if app_data.config.application.html.template_index {
        HttpResponse::Ok().body(
            templating::read_and_apply_templates(
                full_path::get_full_path(
                    &app_data, 
                    "index.html", 
                    false
                ),
                templating::TemplateSchema {
                    content: "".to_string(),
                    shortened: "".to_string(),
                    domain: app_data.config.application.html.domain.clone(),
                    count: if app_data.config.application.html.count {
                        database::count_entries(&mut app_data.database.get_conn().unwrap(), ContentType::All).await.to_string()
                    } else {
                        "".to_string()
                    }
                }
            )
        )
    } else {
        HttpResponse::Ok().body(
            std::fs::read_to_string(full_path::get_full_path(&app_data, "index.html", true)).unwrap()
        )
    }
}


// Takes some content and submits an entry to the database, and serves the appropriate response (url.html or paste.html) depending on the content type
pub async fn submit_entry(form: web::Form<Submission>, app_data: web::Data<crate::AppData>) -> impl Responder {
    let mut connection = app_data.database.get_conn().unwrap();
    let count = database::count_entries(&mut connection, ContentType::All).await;
    for _n in 0..3 {
        let submitted_entry = database::submit_entry(&mut connection, &form.content, &form.content_type).await;
        if submitted_entry.success {
            if app_data.config.application.html.template {
                return HttpResponse::Ok().body(
                    templating::read_and_apply_templates(
                        full_path::get_full_path(
                            &app_data, 
                            // If the content type is a URL, serve the url.html template, otherwise serve the paste.html template
                            if form.content_type == ContentType::Url {
                                "url.html"
                            } else {
                                "paste.html"
                            }, 
                            false),
                        templating::TemplateSchema {
                            content: form.content.clone(),
                            shortened: submitted_entry.shortened.clone(),
                            domain: app_data.config.application.html.domain.clone(),
                            count: count.to_string()
                        }
                    )
                );
            } else {
                // If the content type is a URL, tell them the shortened URL, otherwise tell them the paste ID
                if form.content_type == ContentType::Url {
                    return HttpResponse::Ok().body(format!("Your URL has been shortened to {}/{}", app_data.config.application.html.domain, submitted_entry.shortened));
                } else {
                    return HttpResponse::Ok().body(format!("Your paste is accessible at {}/{}", app_data.config.application.html.domain, submitted_entry.shortened));
                }
            }
        } else {
            return HttpResponse::InternalServerError().body("An error occured while submitting your entry.")
        }
    }
    HttpResponse::InternalServerError().body("An error occured while submitting your URL")
}

// Takes a shortened URL and, depending on whether the content type is a URL or a paste, redirects to the original URL or serves the paste as plaintext
pub async fn serve_entry(path: web::Path<String>, app_data: web::Data<crate::AppData>) -> impl Responder {
    let shortened = path.into_inner();
    let mut connection = app_data.database.get_conn().unwrap();
    let entry = database::retrieve_entry(&mut connection, &shortened).await;
    if entry.success {
        if entry.content_type == ContentType::Url {
            HttpResponse::Found().append_header(("Location", entry.content)).finish()
        } else {
            // If the content type is a paste, serve it as plaintext. Set the MIME type before serving it.
            HttpResponse::Ok().content_type("text/plain").body(entry.content)
        }
    } else {
        HttpResponse::NotFound().body(
            if app_data.config.application.html.template {
                templating::read_and_apply_templates(
                    full_path::get_full_path(&app_data, "404.html", false),
                    templating::TemplateSchema {
                        content: "".to_string(),
                        shortened: "".to_string(),
                        domain: app_data.config.application.html.domain.clone(),
                        count: if app_data.config.application.html.count {
                            database::count_entries(&mut app_data.database.get_conn().unwrap(), ContentType::All).await.to_string()
                        } else {
                            "".to_string()
                        }
                    }
                )
            } else {
                std::fs::read_to_string(PathBuf::from(format!("static/404.html"))).unwrap()
            }
        )
    }
} 