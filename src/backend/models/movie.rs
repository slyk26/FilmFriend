use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: String, // imdb id
    pub link: String,
    pub submitted_by: u64, // user id
    pub info: String,
    pub added: DateTime,
    pub watched: bool
}