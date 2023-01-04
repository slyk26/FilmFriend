use serenity::model::application::interaction::InteractionResponseType;
use serenity::model::prelude::interaction::modal::ModalSubmitInteraction;
use serenity::prelude::Context;
use crate::commands::submit::Submit;

pub async fn call(ctx: &Context, msi: &ModalSubmitInteraction) {
    let response_of_modal = {
        // handle modals here
        match msi.data.custom_id.as_str().split("-").collect::<Vec<&str>>()[0] {
            "submit" => {
                Submit::handle_modal(&msi.data.components, &msi.user, &msi.guild_id.unwrap()).await
            }

            _ => { (true, "".to_string()) }
        }
    };

    // sending a response to the modal caller
    let _ = msi.create_interaction_response(&ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| {
                msg.content(response_of_modal.1).ephemeral(response_of_modal.0)
            })
    }).await;
}