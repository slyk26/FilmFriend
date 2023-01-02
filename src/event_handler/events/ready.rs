use std::collections::HashMap;
use serenity::model::application::command::Command;
use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::commands::slash_command::SlashCommand;

pub async fn call(ctx: &Context, ready: &Ready, commands: &HashMap<String, Box<dyn SlashCommand>>) {
    println!("{} is connected!", ready.user.name);

    // drop all registered commands
    let known_commands = Command::get_global_application_commands(&ctx.http).await.unwrap();

    println!("dropping existings commands");

    for cmd in known_commands {
        let _ = Command::delete_global_application_command(&ctx.http, cmd.id).await;
    }

    // create defined in main
    for cmd in commands {
        let result = Command::create_global_application_command(&ctx.http, |command| {
            cmd.1.register(command)
        }).await;

        let _ = match result {
            Ok(_) => println!("/{} registered", cmd.0),
            Err(_) => panic!("Problem creating command"),
        };
    }
}