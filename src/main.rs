#![feature(async_closure)]

mod commands;
mod events;
mod globals;
pub mod structs;
mod uwu;

use std::{fs::File, io::Read};
use structs::*;

use poise::{
    serenity_prelude::{ChannelId, GatewayIntents, UserId},
    FrameworkError,
};
use sqlx::{sqlite::SqliteConnectOptions, Pool, Sqlite};

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

async fn error_handler(err: FrameworkError<'_, Data, Error>) {
    match err {
        FrameworkError::Command { error, ctx } => {
            // https://tenor.com/view/not-caring-budy-i-am-not-caring-dont-care-gif-22625369
            let _ = ctx
                .send(|m| {
                    // ping me for a slightly higher chance of me noticing
                    m.content("<@852877128844050432>");
                    m.embed(|e| {
                        e.title("go yell at virt")
                            .color(0x2B2D31)
                            .field("Error", format!("```\n{}\n```", error), false)
                            .field(
                                "Command",
                                format!("```\n{}\n```", ctx.invocation_string()),
                                false,
                            )
                    })
                })
                .await;
        }
        FrameworkError::EventHandler {
            error,
            ctx,
            event,
            framework,
        } => {
            let channel = &framework.user_data.logs_channel;
            let _ = channel
                .send_message(ctx, |m| {
                    m.embed(|e| {
                        e.field("Event", format!("```\n{:?}\n```", event), false)
                            .field("Error", error, false)
                    })
                })
                .await;
        }
        _ => (),
    }
}

#[tokio::main]
async fn main() {
    let token = {
        let mut f = File::open("token.txt").expect("Token file not found");
        let mut s = String::new();
        f.read_to_string(&mut s).expect("Failed to read token.");
        s
    };

    use commands::*;
    let commands = vec![uwu(), moyai(), autoreply()];

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let framework = poise::Framework::builder()
        .token(token)
        .intents(intents)
        .setup(|ctx, ready, frm| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &frm.options().commands).await?;
                let bot_pfp = ready
                    .user
                    .avatar_url()
                    .unwrap_or_else(|| globals::BACKUP_PFP.to_string());
                let logs_channel = UserId::from(globals::VIRT_ID)
                    .create_dm_channel(ctx)
                    .await?;
                let cursed_channel = ChannelId::from(globals::CURSED_BOARD);
                let options = SqliteConnectOptions::new()
                    .filename("autovirt.db")
                    .create_if_missing(true);
                let db = Pool::<Sqlite>::connect_with(options).await?;

                Ok(Data {
                    bot_pfp,
                    logs_channel,
                    cursed_channel,
                    db,
                })
            })
        })
        .options(poise::FrameworkOptions {
            commands,
            on_error: |err| Box::pin(error_handler(err)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        });

    framework.run().await.unwrap();
}
