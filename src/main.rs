mod commands;
mod event_handler;
mod backend;

use std::collections::HashMap;
use std::env;
use dotenv::dotenv;
use lazy_static::lazy_static;
use serenity::prelude;
use serenity::prelude::GatewayIntents;
use crate::commands::generate::Generate;

use crate::commands::help::Help;
use crate::commands::movies::Movies;
use crate::commands::ping::Ping;
use crate::commands::submit::Submit;
use crate::commands::slash_command::SlashCommand;
use crate::event_handler::handler::Handler;

lazy_static! {
    // all usable commands
    static ref COMMANDS: HashMap<String, Box<dyn SlashCommand>> = {
        let mut m: HashMap<String, Box<dyn SlashCommand>> = HashMap::new();
        m.insert(Generate.name(), Box::new(Generate));
        m.insert(Movies.name(), Box::new(Movies));
        m.insert(Ping.name(), Box::new(Ping));
        m.insert(Submit.name(), Box::new(Submit));
        m.insert(Help.name(), Box::new(Help));
        m
    };
}

#[tokio::main]
async fn main() {
    // loading .env file
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    // create bot
    let mut bot = prelude::Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    // start bot
    if let Err(why) = bot.start().await {
        println!("Client error: {:?}", why);
    }
}