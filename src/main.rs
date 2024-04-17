pub mod commands;
mod context;
mod events;
pub mod structs;

use commands::commands;
use context::Context;
use sqlx::{migrate, query_as};
use std::env;
use std::sync::Arc;
use structs::Rule;
use tokio::task::JoinSet;
use twilight_gateway::{stream::create_recommended, Config, ConfigBuilder, Event, Intents, Shard};
use twilight_http::Client as HttpClient;
use twilight_model::id::{
    marker::{GuildMarker, UserMarker},
    Id,
};

pub const OWNER_ID: Id<UserMarker> = Id::new(852877128844050432);
pub const TEST_GUILD: Id<GuildMarker> = Id::new(1089645999787610287);
pub const EMBED_COLOR: u32 = 0x7e68d0;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize the tracing subscriber.
    tracing_subscriber::fmt::init();

    let token = env::var("token")?;
    let intents = Intents::GUILDS
        | Intents::GUILD_MESSAGES
        | Intents::MESSAGE_CONTENT
        | Intents::GUILD_MESSAGE_REACTIONS;

    let http = Arc::new(HttpClient::new(token.clone()));
    let app = http.current_user_application().await?.model().await?;

    let config = Config::new(token.clone(), intents);
    let config_callback = |_, builder: ConfigBuilder| builder.build();
    let shards = create_recommended(&http, config, config_callback).await?;

    let db = sqlx::PgPool::connect(&env::var("DATABASE_URL")?).await?;
    migrate!().run(&db).await?;
    let rules: Vec<Rule> = query_as("select * from rules").fetch_all(&db).await?;

    let user = http.current_user().await?.model().await?;
    let avatar = user.avatar.unwrap();
    let ext = if avatar.is_animated() { "gif" } else { "png" };
    let pfp = format!("https://cdn.discordapp.com/{}/{}.{}", user.id, avatar, ext);

    let ctx = Context::new(app.id, http, db, pfp, rules);

    ctx.interaction()
        .set_guild_commands(TEST_GUILD, &commands())
        .await?;

    let mut set = JoinSet::new();
    for shard in shards {
        set.spawn(tokio::spawn(runner(shard, ctx.clone())));
    }

    set.join_next().await;
    Ok(())
}

async fn runner(mut shard: Shard, ctx: Context) {
    loop {
        let event = match shard.next_event().await {
            Ok(event) => event,
            Err(source) => {
                tracing::warn!(?source, "error receiving event");
                if source.is_fatal() {
                    break;
                }

                continue;
            }
        };

        tokio::spawn({
            let ctx = ctx.clone();
            async move {
                match handle_event(event, &ctx).await {
                    Ok(_) => (),
                    Err(err) => tracing::warn!(?err),
                };
            }
        });
    }
}

async fn handle_event(e: Event, ctx: &Context) -> anyhow::Result<()> {
    match e {
        Event::Ready(ready) => {
            tracing::info!("Running in {} guilds.", ready.guilds.len());
            Ok(())
        }
        Event::InteractionCreate(int) => commands::interaction(int, ctx).await,
        _ => events::handle(e, ctx).await,
    }
}
