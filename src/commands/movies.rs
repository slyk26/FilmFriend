use std::env;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::async_trait;
use crate::commands::slash_command::SlashCommand;

pub struct Movies;

#[async_trait]
impl SlashCommand for Movies {
    fn name(&self) -> String {
        "movies".to_string()
    }

    fn description(&self) -> String {
        "Get a list of all Movies".to_string()
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        let msg = format!("All submitted and watched Movies can be found at: {}", env::var("MOVIES").unwrap());
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(msg))
            }).await
    }
}