use std::collections::HashMap;
use serenity::model::application::command::Command;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::commands::slash_command::SlashCommand;

pub async fn call(ctx: &Context, ready: &Ready, commands: &HashMap<String, Box<dyn SlashCommand>>) {
    println!("{} is connected!", ready.user.name);

    for cmd in commands {
        create_command(cmd.0, cmd.1, ctx).await;
    }
}

async fn create_command(name: &str, cmd: &Box<dyn SlashCommand>, ctx: &Context) {
    let result = Command::create_global_application_command(&ctx.http, |command| {
        cmd.register(command)
    }).await;

    let _ = match result {
        Ok(_)  => println!("/{} registered", name),
        Err(_) => panic!("Problem creating command"),
    };
}