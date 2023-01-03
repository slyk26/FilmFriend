use std::time::Duration;
use serenity::model::application::command::Command;
use serenity::model::gateway::{Activity, Ready};
use serenity::prelude::*;
use tokio::time::interval;
use crate::COMMANDS;

pub async fn call(ctx: &Context, ready: &Ready) {
    println!("{} is online!", ready.user.name);

    let ctx_for_thread = ctx.clone();
    // create a seperate thread to update the bots activity in an interval
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        // rotates through a new activity every 60s
        loop {
            interval.tick().await;
            ctx_for_thread.set_activity(Activity::watching("Movies with Bajs")).await;
            interval.tick().await;
            ctx_for_thread.set_activity(Activity::listening("/help")).await;
        }
    });

    // create or update commands defined lazy_init
    for cmd in COMMANDS.iter() {
        let result = Command::create_global_application_command(&ctx.http, |command| {
            cmd.1.register(command)
        }).await;

        let _ = match result {
            Ok(_) => println!("/{} registered", cmd.0),
            Err(_) => panic!("Problem creating command"),
        };
    }
}