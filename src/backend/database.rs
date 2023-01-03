use std::env;
use mongodb::bson::{Bson};
use crate::backend::models::movie::Movie;
use crate::backend::utils::{already_submitted, get_mongo};

pub async fn insert(server_id: String, movie: &Movie) -> (bool, String) {
    let table = get_mongo().await.unwrap()
        .database(env::var("DATABASE").unwrap().as_str()).collection::<Movie>(server_id.to_string().as_str());

    let response: (bool, String);

    if !already_submitted(&movie.id, &table).await {
        let result = table.insert_one(movie, None).await.unwrap();
        if result.inserted_id == Bson::Null {
            response = (true, "Submission failed because of a backend-error!".to_string());
        } else {
            response = (false, ["New Movie submitted: ", movie.link.as_str()].join(" "));
        }
    } else {
        response = (true, "This movie has already been submitted!".to_string());
    }
    response
}