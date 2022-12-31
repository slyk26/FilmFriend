pub mod ping;
pub mod poll;

pub mod slash_command {
    use serenity::builder::CreateApplicationCommand;
    use serenity::model::prelude::interaction::application_command::CommandDataOption;

    pub trait SlashCommand: Send + Sync {
        fn name(&self) -> &str;
        fn run(&self, _options: &[CommandDataOption]) -> String;
        fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand;
    }
}