use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use crate::commands::slash_command::SlashCommand;

pub struct Ping;

impl SlashCommand for Ping {
    fn name(&self) -> &str {
        "ping"
    }

    fn run(&self, _options: &[CommandDataOption]) -> String {
        "Pong!".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description("A ping command")
    }
}