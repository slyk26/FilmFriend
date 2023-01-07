use serenity::model::application::interaction::{Interaction, InteractionResponseType};

use serenity::prelude::*;
use crate::COMMANDS;
use crate::event_handler::events::interactions::*;

pub async fn call(ctx: &Context, interaction: &Interaction) {
    if interaction.guild_locale().is_some() {
        match interaction {

            // regular response (text) => returns the result of the called SlashCommand
            Interaction::ApplicationCommand(interaction) => {
                let cmd = COMMANDS.get(interaction.data.name.as_str()).expect("No Command found in command map");
                application_command::call(&ctx, &interaction, &cmd).await;
            }

            // response after you call a SlashCommand with .kind(InteractionResponseType::Modal)
            Interaction::ModalSubmit(interaction) => {
                modal_submit::call(&ctx, &interaction).await;
            }

            _ => { println!("unknown interaction type") }
        }
    } else {
        match interaction {
            Interaction::ApplicationCommand(interaction) => {
                let _ = interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|message| message.content("The bot only works in a Server"))
                    }).await;
            }

            _ => { println!("unknown interaction type") }
        }
     }
}