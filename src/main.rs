mod commands;
mod handlers;

use std::env;
use dotenv::dotenv;

use serenity::prelude::*;
use handlers::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(ready::Handler)
        .event_handler(interaction_create::Handler)
        .await
        .expect("Error creating client");


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}