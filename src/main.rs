pub mod commands;

use crate::commands::*;

use std::env;

use serenity::async_trait;
use serenity::framework::StandardFramework;
use serenity::framework::standard::macros::group;
use serenity::prelude::*;

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("'"))
        .group(&GENERAL_GROUP);

    let token = env::var("token").expect("token");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Errored while running the client: {:?}", why)
    }
}
