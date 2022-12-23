use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use actix_web::web;
use crate::{AppData, database};
use crate::database::Count;


#[derive(Debug, Clone)]
pub struct TemplateSchema {
    pub content: String,
    pub shortened: String,
    pub count: Vec<Count>
}

pub trait IntoTemplateSchema {
    fn into(self) -> TemplateSchema;
}

impl IntoTemplateSchema for TemplateSchema {
    fn into(self) -> TemplateSchema {
        self
    }
}

impl IntoTemplateSchema for (String, String) {
    fn into(self) -> TemplateSchema {
        TemplateSchema {
            content: self.0,
            shortened: self.1,
            count: Vec::new()
        }
    }
}

impl IntoTemplateSchema for (String, String, Vec<Count>) {
    fn into(self) -> TemplateSchema {
        TemplateSchema {
            content: self.0,
            shortened: self.1,
            count: self.2
        }
    }
}

impl IntoTemplateSchema for (String, String, Count) {
    fn into(self) -> TemplateSchema {
        TemplateSchema {
            content: self.0,
            shortened: self.1,
            count: vec![self.2]
        }
    }
}

impl TemplateSchema {
    pub fn new<T: IntoTemplateSchema>(to_schema: T) -> TemplateSchema {
        to_schema.into()
    }
    pub fn create_null_schema() -> TemplateSchema {
        TemplateSchema {
            content: "".to_string(),
            shortened: "".to_string(),
            count: Vec::new()
        }
    }
}

async fn get_count_and_update_schema (schema: &mut TemplateSchema, app_data: &web::Data<AppData>) {
    let mut connection = app_data.database.get_conn().unwrap();
    let count = database::count_entries(&mut connection, database::ContentType::All).await;
    schema.count.push(count);
}

pub async fn get_necessary_value(key: String, key_value: u8, app_data: web::Data<AppData>, schema: &mut TemplateSchema) -> String {
    // create a vector of (ContentType, i64) tuples
    match key.as_str() {
        "content" => 
            schema.content.as_str().to_string(),
        "shortened" => 
            schema.shortened.as_str().to_string(),
        "domain" => 
            app_data.config.application.html.domain.clone(),
        "count" => 
            match schema.count.iter().find(|&x| x.content_type == key_value.into()) {
                Some(count) => count.count.to_string(),
                None => {
                    get_count_and_update_schema(schema, &app_data).await;
                    schema.count.iter().find(|&x| x.content_type == key_value.into()).unwrap().count.to_string()
                }
            },
        _ => panic!("Invalid key")
    }
}

pub async fn read_and_apply_templates(path: PathBuf, app_data: web::Data<AppData>, schema: &mut TemplateSchema) -> String {
    // read the file into a string
    let mut file = File::open(path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    // replace all instances of {key} with the value, and return the string
    let mut new_contents = String::new();
    let mut key = String::new();
    let mut key_value = 0;
    let mut in_key = false;
    for c in contents.chars() {
        if c == '{' {
            in_key = true;
        } else if c == '}' {
            in_key = false;
            let value = get_necessary_value(key.clone(), key_value, app_data.clone(), schema).await;
            new_contents.push_str(value.as_str());
            key = String::new();
            key_value = 0;
        } else if in_key {
            if c == ':' {
                key_value = key.parse::<u8>().unwrap();
                key = String::new();
            } else {
                key.push(c);
            }
        } else {
            new_contents.push(c);
        }
    } 
    new_contents
}
