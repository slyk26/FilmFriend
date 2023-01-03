use serenity::async_trait;
use serenity::model::application::interaction::Interaction;

use serenity::model::gateway::Ready;

use serenity::prelude::*;
use crate::event_handler::events::*;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, rdy: Ready){
        ready::call(&ctx, &rdy).await;
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        interaction_create::call(&ctx, &interaction).await;
    }
}