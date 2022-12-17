use serde::{Serialize, Deserialize};
use confy;

const CONFIG_NAME: &str = "config";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Data {
    config: Config
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Config {
    pub user: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub database: String
}

pub fn retrieve () -> Config {
    println!("Retrieving config from {:?}.", confy::get_configuration_file_path("url_shortener", CONFIG_NAME).unwrap());
    let data: Data = confy::load("url_shortener", CONFIG_NAME).expect("Unable to read config.toml");
    println!("Config settings retrieved: {:?}", data.config);
    data.config
}