use serde::{Serialize, Deserialize};
use confy;

const CONFIG_NAME: &str = "config";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub http: HttpConfig,
    pub db: DbConfig
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DbConfig {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String
}
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct HttpConfig {
    pub host: String,
    pub port: u16
}

pub fn retrieve () -> Config {
    println!("Retrieving config from {:?}.", confy::get_configuration_file_path("url_shortener", CONFIG_NAME).unwrap());
    let data: Config = confy::load("url_shortener", CONFIG_NAME).expect("Unable to read config.toml");
    println!("Config settings retrieved: {:?}", data);
    data
}