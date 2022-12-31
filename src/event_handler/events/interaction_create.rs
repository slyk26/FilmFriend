use std::collections::HashMap;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};

use serenity::prelude::*;
use crate::commands::slash_command::SlashCommand;

pub async fn call(ctx: &Context, interaction: &Interaction, commands: &HashMap<String, Box<dyn SlashCommand>>) {
    if let Interaction::ApplicationCommand(command) = interaction {
        println!("Received command interaction: {:#?}", command.data.name);

        let cmd = commands.get(command.data.name.as_str()).expect("No Command found in command map");

        let content = cmd.run(&command.data.options);

        if let Err(why) = command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(content))
            })
            .await
        {
            println!("Cannot respond to slash command: {}", why);
        }
    }
}