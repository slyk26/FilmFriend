use std::env;
use mongodb::bson::doc;
use mongodb::Collection;
use regex::Regex;
use mongodb::{Client};
use mongodb::options::ClientOptions;
use once_cell::sync::OnceCell;
use crate::backend::models::movie::Movie;

static MONGO: OnceCell<Client> = OnceCell::new();
static MONGO_INITIALIZED: OnceCell<tokio::sync::Mutex<bool>> = OnceCell::new();

pub async fn get_mongo() -> Option<&'static Client> {
    let client_option = MONGO.get();
    if let Some(_) = client_option {
        return client_option;
    }

    let initializing_mutex = MONGO_INITIALIZED.get_or_init(|| tokio::sync::Mutex::new(false));
    let mut initialized = initializing_mutex.lock().await;

    if !*initialized {
        if let Ok(token) = env::var("DB_CONNECTION_STRING") {
            if let Ok(client_options) = ClientOptions::parse(&token).await {
                if let Ok(client) = Client::with_options(client_options) {
                    if let Ok(_) = MONGO.set(client) {
                        *initialized = true;
                    }
                }
            }
        }
    }
    drop(initialized);
    MONGO.get()
}

pub fn is_valid_imdb_link(link: &str) -> bool {
    let desktop = Regex::new(r"^https://www\.imdb\.com/title/tt[0-9]{7,8}+/$").unwrap();
    let mobile = Regex::new(r"^https://m\.imdb\.com/title/tt[0-9]{7,8}+/$").unwrap();

    desktop.is_match(link) || mobile.is_match(link)
}

pub async fn already_submitted(imdb_id: &String, table: &Collection<Movie>) -> bool {
    match table.find_one(doc!{ "id": imdb_id }, None).await {
        Err(_) => true,
        Ok(movie) => {
            if movie.is_none(){
                false
            } else {
                true
            }
        }
    }
}