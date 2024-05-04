use core::str;

use poise::serenity_prelude::{
    CacheHttp, ChannelId, Context, CreateEmbed, CreateEmbedAuthor, CreateMessage, EditMessage,
    GuildId, Message, MessageId, Reaction, ReactionType, UserId,
};
use sqlx::{query_as, PgPool};
use tracing::info;

use crate::{
    structs::{BoardEntry, Settings},
    utils::{EMBED_COLOR, PFP},
    Data,
};
const MOYAI: &str = "🗿";

pub async fn reaction_count(
    channel_id: ChannelId,
    message_id: MessageId,
    author: UserId,
    ctx: &Context,
) -> anyhow::Result<i32> {
    let mut out = 0;
    let mut after = None;

    let msg = ctx.http.get_message(channel_id, message_id).await?;
    while let Some(reactions) = {
        let reactions = msg
            .reaction_users(
                &ctx.http,
                ReactionType::Unicode(MOYAI.to_owned()),
                Some(100),
                after,
            )
            .await?;
        if let Some(last) = reactions.last() {
            after = Some(last.id);
        }
        if reactions.is_empty() {
            None
        } else {
            Some(reactions)
        }
    } {
        out += reactions
            .iter()
            .filter(|u| !u.bot && u.id != author)
            .count();
    }

    // if you assholes get this to fail i'm gonna be amazed
    Ok(out
        .try_into()
        .expect("there to be < 2147483647 reactions on a message"))
}

/// TODO: Get a better name.
async fn reactions_changed(
    ctx: &Context,
    msg: MessageId,
    channel: ChannelId,
    guild: GuildId,
    count: i32,
    db: &PgPool,
) -> anyhow::Result<()> {
    let settings: Option<Settings> = query_as("select * from settings where guild = $1")
        .bind(guild.to_string())
        .fetch_optional(db)
        .await?;

    if settings.is_none() {
        return Ok(());
    }
    let settings = settings.unwrap();

    if settings.board_threshold.is_none() {
        return Ok(());
    }
    let threshold = settings.board_threshold.unwrap();

    let entry: Option<BoardEntry> = query_as("select * from board where message_id = $1")
        .bind(msg.to_string())
        .fetch_optional(db)
        .await?;

    if let Some(entry) = entry {
        if count >= threshold {
            update_post(entry, count, &settings, db, ctx).await
        } else {
            delete_post(entry, &settings, db, ctx).await
        }
    } else if count >= threshold {
        let message = ctx.http().get_message(channel, msg).await?;
        create_post(message, guild, count, &settings, db, ctx).await
    } else {
        Ok(())
    }
}

async fn delete_post(
    entry: BoardEntry,
    settings: &Settings,
    db: &sqlx::Pool<sqlx::Postgres>,
    ctx: &Context,
) -> anyhow::Result<()> {
    let board_channel = settings.board_channel.unwrap();
    if let Some(post_id) = entry.post_id {
        sqlx::query("delete from board where post_id = $1")
            .bind(post_id.to_string())
            .execute(db)
            .await?;

        ctx.http()
            .delete_message(
                board_channel,
                post_id,
                Some(&format!("{MOYAI} threshold no longer met")),
            )
            .await?;
    }

    Ok(())
}

async fn create_post(
    msg: Message,
    guild: GuildId,
    count: i32,
    settings: &Settings,
    db: &sqlx::Pool<sqlx::Postgres>,
    ctx: &Context,
) -> anyhow::Result<()> {
    let pfp = msg.author.avatar_url().unwrap_or_else(|| PFP.to_owned());
    let mut embed = CreateEmbed::new()
        .color(EMBED_COLOR)
        .author(CreateEmbedAuthor::new(&msg.author.name).icon_url(pfp))
        .description(&msg.content)
        .field("Source", msg.link(), false);
    if let Some(attachment) = msg.attachments.first() {
        embed = embed.image(&attachment.url);
    };

    let post = match settings
        .board_channel
        .unwrap()
        .send_message(
            ctx.http(),
            CreateMessage::new()
                .content(format!("{count} {MOYAI}"))
                .embed(embed),
        )
        .await
    {
        Err(e) => {
            info!(?e);
            None
        }
        Ok(m) => Some(m),
    };

    sqlx::query("insert into board values ($1, $2, $3, $4, $5)")
        .bind(msg.content)
        .bind(guild.to_string())
        .bind(msg.id.to_string())
        .bind(post.map(|m| m.id.to_string()))
        .bind(count)
        .fetch_all(db)
        .await?;

    Ok(())
}

async fn update_post(
    entry: BoardEntry,
    count: i32,
    settings: &Settings,
    db: &sqlx::Pool<sqlx::Postgres>,
    ctx: &Context,
) -> anyhow::Result<()> {
    sqlx::query("update board set stars = $1 where message_id = $2")
        .bind(count)
        .bind(entry.message_id.to_string())
        .execute(db)
        .await?;

    if let Some(post_id) = entry.post_id {
        let channel = settings.board_channel.unwrap();
        channel
            .edit_message(
                ctx.http(),
                post_id,
                EditMessage::new().content(format!("{count} {MOYAI}")),
            )
            .await?;
    }
    Ok(())
}

pub async fn handle(ctx: &Context, data: &Data, reac: &Reaction) -> anyhow::Result<()> {
    let name = if let ReactionType::Unicode(name) = &reac.emoji {
        name
    } else {
        return Ok(());
    };

    if name != MOYAI || reac.guild_id.is_none() {
        return Ok(());
    }

    let msg = reac.message(ctx.http()).await?;
    let count = reaction_count(reac.channel_id, reac.message_id, msg.author.id, ctx).await?;

    reactions_changed(
        ctx,
        msg.id,
        reac.channel_id,
        reac.guild_id.unwrap(),
        count,
        &data.db,
    )
    .await?;

    Ok(())
}
