use mysql::prelude::*;
use mysql::*;

use crate::config::{DbConfig};

pub fn init(db_config: DbConfig) -> Pool {
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        db_config.user, db_config.password, db_config.host, db_config.port, db_config.database
    );
    println!("Connecting to database at {}.", url);
    let pool = Pool::new(url.as_str()).expect("Unable to connect to database.");
    if !does_table_exist(&pool) {
        println!("Table does not exist. Creating them.");
        create_table(&pool);
    }
    pool
}

pub fn does_table_exist(pool: &Pool) -> bool {
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<String> = conn
        .exec_map(
            r"
        SELECT table_name FROM information_schema.tables WHERE table_schema = :database
    ",
            params! {
                "database" => "url_shortener"
            },
            |table_name| table_name,
        )
        .unwrap();
    result.len() > 0
}

pub fn create_table(pool: &Pool) {
    let mut conn = pool.get_conn().unwrap();
    conn.query_drop(
        r"
        CREATE TABLE IF NOT EXISTS `urls` (
            `id` INT NOT NULL AUTO_INCREMENT,
            `url` VARCHAR(255) NOT NULL,
            `shortened` VARCHAR(255) NOT NULL,
            PRIMARY KEY (`id`)
        ) ENGINE=InnoDB;
    ",
    )
    .unwrap();
    println!("Table created.");
}

pub fn insert_url(pool: &Pool, url: &str, shortened: &str) {
    let mut conn = pool.get_conn().unwrap();
    conn.exec_drop(
        r"
        INSERT INTO `urls` (`url`, `shortened`) VALUES (:url, :shortened)
    ",
        params! {
            "url" => url,
            "shortened" => shortened
        },
    )
    .unwrap();
}

pub fn get_url(pool: &Pool, shortened: &str) -> Option<String> {
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<String> = conn
        .exec_map(
            r"
        SELECT `url` FROM `urls` WHERE `shortened` = :shortened
    ",
            params! {
                "shortened" => shortened
            },
            |url| url,
        )
        .unwrap();
    result.into_iter().next()
}

pub fn get_shortened(pool: &Pool, url: &str) -> Option<String> {
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<String> = conn
        .exec_map(
            r"
        SELECT `shortened` FROM `urls` WHERE `url` = :url
    ",
            params! {
                "url" => url
            },
            |shortened| shortened,
        )
        .unwrap();
    result.into_iter().next()
}

pub fn url_exists(pool: &Pool, url: &str) -> bool {
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<String> = conn
        .exec_map(
            r"
        SELECT `url` FROM `urls` WHERE `url` = :url
    ",
            params! {
                "url" => url
            },
            |url| url,
        )
        .unwrap();
    result.len() > 0
}

pub fn shortened_exists(pool: &Pool, shortened: &str) -> bool {
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<String> = conn
        .exec_map(
            r"
        SELECT `shortened` FROM `urls` WHERE `shortened` = :shortened
    ",
            params! {
                "shortened" => shortened
            },
            |shortened| shortened,
        )
        .unwrap();
    result.len() > 0
}

pub fn count_urls(pool: &Pool) -> u64 {
    let mut conn = pool.get_conn().unwrap();
    let result: Vec<u64> = conn
        .exec_map(
            r"
        SELECT COUNT(*) FROM `urls`
    ",
            (),
            |count| count,
        )
        .unwrap();
    result.into_iter().next().unwrap()
}
