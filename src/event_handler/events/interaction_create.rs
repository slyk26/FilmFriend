use std::collections::HashMap;
use mongodb::Database;
use serenity::model::application::interaction::{Interaction};

use serenity::prelude::*;
use crate::commands::slash_command::SlashCommand;
use crate::event_handler::events::interactions::*;

pub async fn call(ctx: &Context, interaction: &Interaction, commands: &HashMap<String, Box<dyn SlashCommand>>, db: &Database) {
    match interaction {

        // regular response (text) => returns the result of the called SlashCommand
        Interaction::ApplicationCommand(interaction) => {
            let cmd = commands.get(interaction.data.name.as_str()).expect("No Command found in command map");
            application_command::call(&ctx, &interaction, &cmd).await;
        }

        // response after you call a SlashCommand with .kind(InteractionResponseType::Modal)
        Interaction::ModalSubmit(interaction) => {
            modal_submit::call(&ctx, &interaction, &db).await;
        }

        _ => { println!("unknown interaction type") }
    }
}