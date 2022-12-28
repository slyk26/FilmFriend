use serenity::async_trait;

use serenity::model::application::command::Command;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::commands;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_command = Command::create_global_application_command(&ctx.http, |command| {
            commands::ping::register(command)
        })
            .await;

        println!("I created the following global slash command: {:#?}", guild_command);
    }
}