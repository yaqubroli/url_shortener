use actix_settings::BasicSettings;
use serde::{Serialize, Deserialize};
use crate::{settings, shortener, url};

use mysql::prelude::*;
use mysql::*;

#[derive(PartialEq, Eq, Debug, Serialize, Deserialize, Clone)]
pub enum ContentType {
    Url,
    Pastebin,
    Unimplemented,
    All
}

impl From<u8> for ContentType {
    fn from(item: u8) -> Self {
        match item {
            0 => ContentType::Url,
            1 => ContentType::Pastebin,
            255 => ContentType::All,
            _ => ContentType::Unimplemented
        }
    }
}

impl From<ContentType> for u8 {
    fn from(item: ContentType) -> Self {
        match item {
            ContentType::Url => 0,
            ContentType::Pastebin => 1,
            ContentType::All => 255,
            _ => 255
        }
    }
}

#[derive(Debug, Clone)]
pub struct Count {
    pub count: u64,
    pub content_type: ContentType
}

impl From<Count> for u64 {
    fn from(item: Count) -> Self {
        item.count
    }
}

impl From<u64> for Count {
    fn from(item: u64) -> Self {
        Count {
            count: item,
            content_type: ContentType::All
        }
    }
}

#[derive(Debug)]
pub struct RetrievedUrl {
    pub url: String,
    pub success: bool
}

#[derive(Debug)]
pub struct SubmittedUrl {
    pub shortened: String,
    pub success: bool,
}

pub struct SubmittedEntry {
    pub shortened: String,
    pub success: bool,
}

pub struct RetrievedEntry {
    pub content: String,
    pub success: bool,
    pub content_type: ContentType
}

// This function sanitizes the input to prevent SQL injection
fn sanitize_input (input: String) -> String {
    input.replace("'", "''")
}

// Description: This function takes in a settings struct and returns a mysql connection pool
pub async fn init (settings: &BasicSettings<settings::AppSettings>) -> Pool {
    let database_settings = &settings.application.database;
    let url = format!("mysql://{}:{}@{}:{}/{}", database_settings.username, database_settings.password, database_settings.host, database_settings.port, database_settings.database);
    let pool = Pool::new(url.as_str()).unwrap();
    // create the table if it doesn't exist
    let mut connection = pool.get_conn().unwrap();
    if create_entries_table(&mut connection).await {
        println!("Created table `entries`");
    } else {
        println!("Table `entries` already exists, no need to create it");
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

// Description: This function takes in a connection and a ContentType, and counts the number of entries with that content_type
pub async fn count_entries (connection: &mut PooledConn, content_type: ContentType) -> Count {
    // if content_type is ContentType::All, then we don't need to filter by content_type
    if content_type == ContentType::All {
        let mut result = connection.exec_iter("SELECT COUNT(*) FROM entries", ()).unwrap();
        let row = result.next().unwrap();
        let count = row.unwrap().get::<u64, _>("COUNT(*)").unwrap();
        return Count {
            count,
            content_type
        };
    }
    let mut result = connection.exec_iter("SELECT COUNT(*) FROM entries WHERE content_type = :content_type", params! {
        "content_type" => content_type.clone() as u8
    }).unwrap();
    let row = result.next().unwrap();
    let count = row.unwrap().get::<u64, _>("COUNT(*)").unwrap();
    Count {
        count,
        content_type
    }
}

// Description: This function takes in a connection and a shortened url and returns a RetrievedEntry struct, where `content` is the content column and `success` is true if the shortened url exists in the database
pub async fn retrieve_entry (connection: &mut PooledConn, shortened: &str) -> RetrievedEntry {
    let mut result = connection.exec_iter("SELECT content, content_type FROM entries WHERE shortened = :shortened", params! {
        "shortened" => shortened
    }).unwrap();
    let row = result.next().expect("No entry found");
    let content = row.as_ref().unwrap().get::<String, _>("content").unwrap();
    let content_type = ContentType::from(row.as_ref().unwrap().get::<u8, _>("content_type").unwrap());
    RetrievedEntry {
        content,
        success: true,
        content_type
    }
}

// Description: This function takes in a connection, a `content` string, and a content_type u8 and returns a SubmittedEntry struct, where `shortened` is the shortened url column and `success` is true if the shortened url does not exist in the database.
pub async fn submit_entry (connection: &mut PooledConn, content: &str, content_type: &ContentType) -> SubmittedEntry {
    let shortened = shortener::base64(count_entries(connection, ContentType::All).await.count);
    let row = connection.exec_iter("SELECT shortened FROM entries WHERE shortened = :shortened", params! {
        "shortened" => shortened.clone()
    }).unwrap().next();
    if row.is_some() {
        SubmittedEntry {
            shortened: shortened.clone().to_string(),
            success: false
        }
    } else {
        connection.exec_drop("INSERT INTO entries (content, shortened, content_type) VALUES (:content, :shortened, :content_type)", params! {
            "content" => 
                if content_type == &ContentType::Url {
                    sanitize_input(url::format_url(content.to_string()))
                } else {
                    sanitize_input(content.to_string())
                },
            "shortened" => shortened.clone(),
            "content_type" => u8::from(content_type.clone())
        }).unwrap();
        SubmittedEntry {
            shortened: shortened.clone().to_string(),
            success: true
        }
    }
}

// Description: This function takes in a connection and creates the table `entries`, which has rows `content`, `shortened`, and `content_type` (byte)
pub async fn create_entries_table (connection: &mut PooledConn) -> bool {
    let mut result = connection.exec_iter("CREATE TABLE IF NOT EXISTS entries (content TEXT(65535) NOT NULL, shortened VARCHAR(255) NOT NULL, content_type TINYINT NOT NULL, PRIMARY KEY (shortened))", ()).unwrap();
    let row = result.next();
    if row.is_some() {
        true
    } else {
        false
    }
}