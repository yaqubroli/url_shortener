use actix_settings::{BasicSettings};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub database: DatabaseSettings,
    pub html: HtmlSettings
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct HtmlSettings {
    pub template: bool,
    pub template_index: bool,
    pub template_static: bool,
    pub count: bool,
    pub domain: String,
    pub path: String,
    pub static_path: String
}

pub fn init () -> BasicSettings<AppSettings> {
    BasicSettings::<AppSettings>::parse_toml("./config.toml").unwrap()
}