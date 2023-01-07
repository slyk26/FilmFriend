use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerKey {
    pub key: i64,
    pub created_by: u64,
    pub server_id: String,
    pub added: DateTime
}