use serenity::builder::CreateEmbed;
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::prelude::SerenityError;
use crate::commands::slash_command::SlashCommand;
use serenity::async_trait;
use serenity::utils::Color;
use crate::COMMANDS;

pub struct Help;

#[async_trait]
impl SlashCommand for Help{
    fn name(&self) -> String {
        "help".to_string()
    }

    fn description(&self) -> String {
        "Overview of all available commands and what they do".to_string()
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        let mut embed = CreateEmbed::default();

        embed.title("FilmFriend - Commands")
            .description("I know the following Commands:");

        // adding all known commands
        for command in COMMANDS.iter() {
            embed.field(format!("/{}", command.0) , command.1.description(), false);
        }

        // setting accent color
        embed.colour(Color::from_rgb(255, 255 ,255));

        command
            .create_interaction_response(&ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.add_embed(embed))
            }).await
    }
}