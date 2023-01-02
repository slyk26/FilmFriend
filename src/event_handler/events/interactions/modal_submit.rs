use mongodb::Database;
use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::modal::ModalSubmitInteraction;
use serenity::prelude::Context;
use crate::commands::submit::Submit;

pub async fn call(ctx: &Context, msi: &ModalSubmitInteraction, db: &Database) {
    let response_of_modal = {

        // handle modals here
        match msi.data.custom_id.as_str() {
            "submit" => {
                Submit::handle_modal(&msi.data.components, &msi.user, &msi.channel_id, &db).await
            }

            _ => {"".to_string()}
        }
    };

    // sending a response to the modal caller
    let _ = msi.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.content(response_of_modal).ephemeral(true)
            })
    }).await;
}