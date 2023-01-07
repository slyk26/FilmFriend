pub mod ping;
pub mod submit;
pub mod help;
pub mod movies;
pub mod generate;

pub mod slash_command {
    use serenity::async_trait;
    use serenity::builder::{CreateApplicationCommand};
    use serenity::client::Context;
    use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
    use serenity::prelude::SerenityError;

    /// main Trait for a Discord SlashCommand
    #[async_trait]
    pub trait SlashCommand: Send + Sync {
        /// returns the name of this [SlashCommand]
        fn name(&self) -> String;

        /// returns the description of this [SlashCommand]
        fn description(&self) -> String;

        /// modifies a [CreateApplicationCommand]
        /// this method is called for each defined [SlashCommand] in main.rs
        fn register<'a>(&self, command: &'a mut CreateApplicationCommand) -> &'a mut CreateApplicationCommand {
            command.name(self.name()).description(self.description())
        }

        /// executes code implemented in each Command struct that implements this Trait
        async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError>;
    }
}