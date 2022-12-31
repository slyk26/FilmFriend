use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use crate::commands::slash_command::SlashCommand;

pub struct Poll;

impl SlashCommand for Poll {
    fn name(&self) -> &str {
        "poll"
    }

    fn run(&self, _options: &[CommandDataOption]) -> String {
        "Hi!".to_string()
    }

    fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
        command.name(self.name()).description("create a poll")
    }
}