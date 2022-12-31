use std::collections::HashMap;
use serenity::async_trait;
use serenity::model::application::interaction::Interaction;

use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::commands::slash_command::SlashCommand;
use crate::event_handler::events::*;

pub struct Handler {
    pub commands: HashMap<String, Box<dyn SlashCommand>>
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, rdy: Ready){
        ready::call(&ctx, &rdy, &self.commands).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::call(&ctx, &interaction, &self.commands).await;
    }
}