use std::collections::HashMap;
use std::env;
use mongodb::Database;
use mongodb::options::ClientOptions;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;

use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::commands::slash_command::SlashCommand;
use crate::event_handler::events::*;

pub struct Handler {
    commands: HashMap<String, Box<dyn SlashCommand>>,
    db: Database
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, rdy: Ready){
        ready::call(&ctx, &rdy, &self.commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::call(&ctx, &interaction, &self.commands, &self.db).await;
    }
}

impl Handler {
    pub async fn new(commands: HashMap<String, Box<dyn SlashCommand>>) -> Handler {
        // database
        let connection_string = env::var("DB_CONNECTION_STRING").expect("Expected the connection string");
        let client_options = ClientOptions::parse(connection_string).await.unwrap();
        let client = mongodb::Client::with_options(client_options).unwrap();
        let db = client.database(env::var("DATABASE").expect("Expected the database").as_str());

        Handler{commands, db}
    }
}