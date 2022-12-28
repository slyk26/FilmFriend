use serenity::model::application::command::Command;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::commands;

pub async fn call(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    let _guild_command = Command::create_global_application_command(&ctx.http, |command| {
        commands::ping::register(command)
    })
        .await;
}