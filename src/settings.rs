use actix_settings::{BasicSettings};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub database: DatabaseSettings,
    pub templating: TemplatingSettings
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
pub struct TemplatingSettings {
    pub enabled: bool,
    pub domain: String
}

pub fn init () -> BasicSettings<AppSettings> {
    BasicSettings::<AppSettings>::parse_toml("./config.toml").unwrap()
}