mod commands;
mod event_handler;
mod backend;

use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use serenity::prelude;
use serenity::prelude::GatewayIntents;
use crate::commands::ping::Ping;
use crate::commands::submit::Submit;
use crate::commands::slash_command::SlashCommand;
use crate::event_handler::handler::Handler;

#[tokio::main]
async fn main() {

    // usable commands
    let commands = {
        let mut commands: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
        commands.insert(Ping.name().to_string(), Box::new(Ping));
        commands.insert(Submit.name().to_string(), Box::new(Submit));
        commands
    };


    // loading .env file
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // create bot
    let mut bot = prelude::Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler::new(commands).await)
        .await
        .expect("Error creating client");

    // start bot
    if let Err(why) = bot.start().await {
        println!("Client error: {:?}", why);
    }
}