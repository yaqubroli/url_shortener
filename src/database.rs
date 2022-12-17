use diesel::prelude::*;
use diesel::mysql::MysqlConnection;
use serde::Deserialize;
use std::fs;

use crate::config::Config;

pub fn init (config: Config) -> MysqlConnection {
    MysqlConnection::establish(
        format!("mysql://{}:{}@{}:{}/{}", config.user, config.password, config.host, config.port, config.database).as_str()
    ).expect("Unable to connect to database")
}