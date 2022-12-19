use actix_settings::BasicSettings;
use crate::settings;

use mysql::prelude::*;
use mysql::*;

#[derive(Debug)]
pub struct RetrievedUrl {
    pub url: String,
    pub success: bool
}

#[derive(Debug)]
pub struct SubmittedUrl {
    pub shortened: String,
    pub success: bool
}

// Description: This function takes in a settings struct and returns a mysql connection pool
pub async fn init (settings: &BasicSettings<settings::AppSettings>) -> Pool {
    let database_settings = &settings.application.database;
    let url = format!("mysql://{}:{}@{}:{}/{}", database_settings.username, database_settings.password, database_settings.host, database_settings.port, database_settings.database);
    let pool = Pool::new(url.as_str()).unwrap();
    // create the table if it doesn't exist
    let mut connection = pool.get_conn().unwrap();
    if create_table(&mut connection).await {
        println!("Created table `urls`");
    } else {
        println!("Table `urls` already exists");
    }
    pool
}

// Description: This function takes in a connection and a shortened url and returns a RetrievedUrl struct, where `url` is the url column and `success` is true if the shortened url exists in the database
pub async fn retrieve_url (connection: &mut PooledConn, shortened: &str) -> RetrievedUrl {
    let mut result = connection.exec_iter("SELECT url FROM urls WHERE shortened = :shortened", params! {
        "shortened" => shortened
    }).unwrap();
    let row = result.next().unwrap();
    let url = row.unwrap().get::<String, _>("url").unwrap();
    RetrievedUrl {
        url,
        success: true
    }
}

// Description: This function takes in a connection and a url and a shortened url and returns a SubmittedUrl struct, where `shortened` is the shortened url column and `success` is true if the shortened url does not exist in the database
pub async fn submit_url (connection: &mut PooledConn, url: &str, shortened: &str) -> SubmittedUrl {
    let row = connection.exec_iter("SELECT shortened FROM urls WHERE shortened = :shortened", params! {
        "shortened" => shortened
    }).unwrap().next();
    if row.is_some() {
        SubmittedUrl {
            shortened: shortened.to_string(),
            success: false
        }
    } else {
        let row = connection.exec_iter("SELECT shortened FROM urls WHERE url = :url", params! {
            "url" => url
        }).unwrap().next();
        if row.is_some() {
            let shortened = row.unwrap().unwrap().get::<String, _>("shortened").unwrap();
            SubmittedUrl {
                shortened,
                success: true
            }
        } else {
            connection.exec_drop("INSERT INTO urls (url, shortened) VALUES (:url, :shortened)", params! {
                "url" => url,
                "shortened" => shortened
            }).unwrap();
            SubmittedUrl {
                shortened: shortened.to_string(),
                success: true
            }
        }
    }
}

// Description: This function takes in a connection and returns a u64 which is the number of rows in the table
pub async fn count_urls (connection: &mut PooledConn) -> u64 {
    let mut result = connection.exec_iter("SELECT COUNT(*) FROM urls", ()).unwrap();
    let row = result.next().unwrap();
    let count = row.unwrap().get::<u64, _>("COUNT(*)").unwrap();
    count
}

// Description: This function takes in a connection and returns a bool which is true if the table was created
pub async fn create_table (connection: &mut PooledConn) -> bool {
    let mut result = connection.exec_iter("CREATE TABLE IF NOT EXISTS urls (url VARCHAR(255) NOT NULL, shortened VARCHAR(255) NOT NULL, PRIMARY KEY (shortened))", ()).unwrap();
    let row = result.next();
    if row.is_some() {
        true
    } else {
        false
    }
}

