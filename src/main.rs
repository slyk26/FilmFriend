mod commands;
mod event_handler;

use std::env;
use dotenv::dotenv;

use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(event_handler::handler::Handler)
        .await
        .expect("Error creating client");


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}