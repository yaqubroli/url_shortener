use actix_settings::{ApplySettings as _, Settings, BasicSettings};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AppSettings {
    pub database: DatabaseSettings
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String
}

pub fn init () -> BasicSettings<AppSettings> {
    BasicSettings::<AppSettings>::parse_toml("./config.toml").unwrap()
}