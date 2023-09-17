mod commands;
mod events;
pub mod structs;
mod uwu;
use poise::{
    serenity_prelude::{ChannelId, GatewayIntents, Ready, UserId},
    Framework, FrameworkError,
};
use shuttle_poise::ShuttlePoise;
use shuttle_runtime::Context as ShuttleContext;
use shuttle_secrets::SecretStore;
use sqlx::PgPool;
use structs::*;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub const MOYAI: &str = "🗿";

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

async fn setup(
    secret_store: &SecretStore,
    ctx: &poise::serenity_prelude::Context,
    ready: &Ready,
    frm: &Framework<Data, Error>,
    db: PgPool,
) -> Result<Data, Error> {
    poise::builtins::register_globally(ctx, &frm.options().commands).await?;
    let bot_pfp = ready.user.avatar_url();
    let bot = ready.user.clone();
    let logs_channel = UserId::from(852877128844050432)
        .create_dm_channel(ctx)
        .await?;
    let cursed_id: u64 = secret_store
        .get("cursed_board")
        .context("'cursed_board' not found")?
        .parse()
        .context("'cursed_board' is not a number")?;

    let threshold: u64 = secret_store
        .get("threshold")
        .context("'threshold' not found")?
        .parse()
        .context("'threshold' is not a number")?;

    let cursed_channel = ChannelId::from(cursed_id);
    let startup = chrono::Local::now();

    Ok(Data {
        bot_pfp,
        bot,
        logs_channel,
        cursed_channel,
        threshold,
        db,
        startup,
    })
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
    #[shuttle_shared_db::Postgres] db: PgPool,
) -> ShuttlePoise<Data, Error> {
    let token = secret_store.get("token").context("'token' not found")?;
    sqlx::migrate!()
        .run(&db)
        .await
        .context("Migration failed")?;

    use commands::*;
    let commands = vec![
        uwu(),
        moyai(),
        autoreply(),
        scan(),
        calc(),
        uptime(),
        neko(),
    ];

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let framework = poise::Framework::builder()
        .token(token)
        .intents(intents)
        .setup(|ctx, ready, frm| {
            Box::pin(async move { setup(&secret_store, ctx, ready, frm, db).await })
        })
        .options(poise::FrameworkOptions {
            commands,
            on_error: |err| Box::pin(error_handler(err)),
            event_handler: |ctx, event, framework, data| {
                Box::pin(events::handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some(";".into()),
                edit_tracker: Some(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(60 * 60),
                )),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .build()
        .await
        .map_err(shuttle_runtime::CustomError::new);

    Ok(framework?.into())
}
