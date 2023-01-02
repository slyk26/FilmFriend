use serenity::builder::{CreateApplicationCommand};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use crate::commands::slash_command::SlashCommand;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use serenity::async_trait;

pub struct Ping;

#[async_trait]
impl SlashCommand for Ping {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description("A ping command")
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content("Ping!"))
            }).await
    }
}