use mongodb::bson::{Bson};
use mongodb::{Database};
use crate::backend::models::movie::Movie;
use crate::backend::utils::already_submitted;

pub async fn insert(server_id: String, movie: &Movie, db: &Database) -> String {
    let response: String;
    let table = db.collection::<Movie>(server_id.to_string().as_str());

    if !already_submitted(&movie.id, &table).await {
        let result = table.insert_one(movie, None).await.unwrap();
        if result.inserted_id == Bson::Null {
            response = "Submission failed because of a backend-error!".to_string();
        } else {
            response = "Movie submitted successfully!".to_string();
        }
    } else {
        response = "This movie has already been submitted!".to_string();
    }
    response
}