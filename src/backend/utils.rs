use mongodb::bson::doc;
use mongodb::Collection;
use regex::Regex;
use crate::backend::models::movie::Movie;

pub fn is_valid_imdb_link(link: &str) -> bool {
    let desktop = Regex::new(r"^https://www\.imdb\.com/title/tt[0-9]{7}+/$").unwrap();
    let mobile = Regex::new(r"^https://m\.imdb\.com/title/tt[0-9]{7}+/$").unwrap();

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