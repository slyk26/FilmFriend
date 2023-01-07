use mongodb::bson::{DateTime};
use serenity::client::Context;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::SerenityError;
use serenity::async_trait;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::Permissions;
use crate::backend::database::{insert_key, is_key_for_server_in_db};
use crate::backend::models::server_key::ServerKey;
use crate::commands::slash_command::SlashCommand;

pub struct Generate;

#[async_trait]
impl SlashCommand for Generate {
    fn name(&self) -> String {
        "generate".to_string()
    }

    fn description(&self) -> String {
        "[admin] generate a new password for editing movies".to_string()
    }

    async fn execute(&self, ctx: &Context, command: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
        let is_admin = command.member.as_ref().unwrap().permissions.unwrap().contains(Permissions::ADMINISTRATOR);
        let key_existed = is_key_for_server_in_db(command.guild_id.as_ref().unwrap().as_u64()).await;
        let server_name = ctx.http.get_guild(command.guild_id.unwrap().as_u64().clone()).await.unwrap().name;
        let mut msg: &str = "";
        let key = DateTime::now().timestamp_millis();

        if is_admin && !key_existed {
            insert_key(
                ServerKey{
                    key,
                    created_by: command.user.id.0,
                    server_id: command.guild_id.unwrap().0.to_string(),
                    added: DateTime::now(),
                }
            ).await
        }

        if is_admin {
            command.user.dm(&ctx.http, |create_message| {
                if !key_existed {
                    msg = "**Watch Key has been generated!**";

                    create_message.add_embed(|embed| {
                        embed.title("Generated Key")
                            .description(format!("This is your access key for editing movies of {}.\n Do not forget it, and share it with Caution!", server_name))
                            .field("KEY: ", key, true);
                        embed
                    })

                } else {
                    msg = "Watch Key has already been generated";

                    create_message.add_embed(|embed| {
                        embed.title("Key has already been created!")
                            .description(format!("{} already has a Server Key", server_name));
                        embed
                    })
                }
            }).await.expect("[/generate] couldnt send dm");

            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(msg.to_string()).ephemeral(key_existed))
                }).await
        } else {
            command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Only Admins can issue this command!").ephemeral(true))
                }).await
        }
    }
}