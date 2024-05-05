use std::{
    collections::HashSet,
    env,
    sync::{Arc, RwLock},
    time::Instant,
};
pub mod commands;
pub mod events;
pub mod structs;
pub mod utils;
use sqlx::{query_as, PgPool};
use structs::Rule;

use poise::{serenity_prelude as serenity, EditTracker};
use tracing::info;
use utils::OWNER_ID;

pub struct Data {
    pub rules: RwLock<Vec<Rule>>,
    pub start: Instant,
    pub db: PgPool,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    let token = env::var("token").expect("blehh");
    let intents = serenity::GatewayIntents::all();

    let db = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;

    match sqlx::migrate!().run(&db).await {
        Ok(_) => (),
        Err(e) => {
            info!(?e);
        }
    };

    let rules: Vec<Rule> = query_as("select * from rules").fetch_all(&db).await?;

    let mut owners = HashSet::new();
    owners.insert(OWNER_ID);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::list(),
            //on_error: (),
            //pre_command: (),
            //post_command: (),
            // maybe at some point paywall shit? :tshrug:
            //command_check: (),
            skip_checks_for_owners: true,
            //reply_callback: (),
            manual_cooldowns: false,
            //require_cache_for_guild_check: (),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::handle(ctx, event, framework, data))
            },
            //listener: (),
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(utils::PREFIX.to_owned()),
                additional_prefixes: vec![],
                //neat, but later
                //dynamic_prefix: (),
                //stripped_dynamic_prefix: (),
                mention_as_prefix: true,
                edit_tracker: Some(Arc::new(EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                execute_untracked_edits: true,
                //ignore_edits_if_not_yet_responded: (),
                //execute_self_messages: (),
                //ignore_bots: (),
                //ignore_thread_creation: (),
                case_insensitive_commands: true,
                ..Default::default()
            },
            owners,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    rules: rules.into(),
                    start: Instant::now(),
                    db,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client?.start().await?;

    Ok(())
}
