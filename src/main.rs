mod commands;
mod event_handler;

use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use serenity::prelude::*;
use crate::commands::ping::Ping;
use crate::commands::poll::Poll;
use crate::commands::slash_command::SlashCommand;

#[tokio::main]
async fn main() {

    let commands = {
        let mut commands: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
        commands.insert(Ping.name().to_string(), Box::new(Ping));
        commands.insert(Poll.name().to_string(), Box::new(Poll));
        commands
    };

    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::GUILD_MESSAGES)
        .event_handler(event_handler::handler::Handler { commands })
        .await
        .expect("Error creating client");


    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}