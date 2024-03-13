mod commands;
mod events;
pub mod structs;
mod uwu;

use poise::{
    serenity_prelude::{
        self, CacheHttp, ChannelId, CreateEmbed, CreateMessage, GatewayIntents, Ready, UserId,
    },
    CreateReply, Framework, FrameworkError,
};
use serenity_prelude as serenity;
use sqlx::{postgres::PgPoolOptions, query_as, PgPool};
use std::{env};
use structs::*;
use tokio::sync::{RwLock};

pub type Error = Box<dyn std::error::Error + Send + Sync>;

pub type Context<'a> = poise::Context<'a, Data, Error>;

pub const MOYAI: &str = "🗿";

#[macro_export]
#[allow(clippy::crate_in_macro_def)]

macro_rules! get_env {
    ($u:literal) => {
        &env::var($u).expect(&format!("{} not found", $u))
    };
}

async fn error_handler(err: FrameworkError<'_, Data, Error>) {
    match err {
        FrameworkError::Command { error, ctx, .. } => {
            // https://tenor.com/view/not-caring-budy-i-am-not-caring-dont-care-gif-22625369
            let e = CreateEmbed::new()
                .title("go yell at virt")
                .color(0x2b2d31)
                .field("Error", format!("```rs\n{}\n```", error), false)
                .field("Command", format!("`{}`", ctx.invocation_string()), false);
            let m = CreateReply::default()
                .content("<@852877128844050432>")
                .embed(e);
            let _ = ctx.send(m).await;
        }
        FrameworkError::EventHandler {
            error,
            ctx,
            event,
            framework,
            ..
        } => {
            let channel = &framework.user_data.logs_channel;

            let embed = CreateEmbed::new()
                .field("Event", format!("```\n{:?}\n```", event), false)
                .field("Error", error.to_string(), false);
            let message = CreateMessage::new().embed(embed);
            let _ = channel.send_message(ctx, message).await;
        }
        _ => (),
    }
}

async fn setup(
    ctx: &poise::serenity_prelude::Context,
    ready: &Ready,
    frm: &Framework<Data, Error>,
    db: PgPool,
) -> Result<Data, Error> {
    poise::builtins::register_globally(ctx, &frm.options().commands).await?;

    let user = ctx.http().get_current_user().await?;
    //let avatar = CreateAttachment::path("./pfp.gif").await?;
    //user.edit(ctx.http(), EditProfile::new().avatar(&avatar))
    //    .await?;

    let bot_pfp = user.avatar_url();

    let bot = ready.user.clone();

    let logs_channel = UserId::from(852877128844050432)
        .create_dm_channel(ctx)
        .await?;

    let cursed_id: u64 = get_env!("cursed_board")
        .parse()
        .expect("'cursed_board' is not a number");

    let threshold: u64 = get_env!("threshold")
        .parse()
        .expect("'threshold' is not a number");

    let cursed_channel = ChannelId::from(cursed_id);

    let startup = chrono::Local::now();

    let autoreplies: Vec<AutoReply> = query_as!(AutoReply, "select * from replies")
        .fetch_all(&db)
        .await?;

    Ok(Data {
        bot_pfp,
        bot,
        logs_channel,
        cursed_channel,
        threshold,
        db,
        startup,
        autoreplies: RwLock::new(autoreplies),
    })
}

#[tokio::main]

async fn main() -> Result<(), Error> {
    let token = get_env!("token");

    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(get_env!("db_url"))?;

    sqlx::migrate!().run(&db).await.expect("Migrations failed.");

    use commands::*;

    let commands = vec![
        uwu(),
        autoreply(),
        calc(),
        uptime(),
        neko(),
        nerd(),
        moyai(),
        scan(),
    ];

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let framework = poise::Framework::builder()
        .setup(|ctx, ready, frm| Box::pin(async move { setup(ctx, ready, frm, db).await }))
        .options(poise::FrameworkOptions {
            commands,
            on_error: |err| Box::pin(error_handler(err)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                edit_tracker: Some(
                    poise::EditTracker::for_timespan(std::time::Duration::from_secs(60 * 60))
                        .into(),
                ),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    Ok(())
}
