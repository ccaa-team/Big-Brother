use core::str;
use std::sync::Arc;

use crate::{
    structs::{BoardEntry, Settings},
    Context,
};
use sqlx::{query_as, PgPool};
use tracing::info;
use twilight_http::{
    request::channel::reaction::RequestReactionType::Unicode, Client as HttpClient,
};
use twilight_model::{
    channel::{message::ReactionType, Message},
    gateway::payload::incoming::{ReactionAdd, ReactionRemove},
    id::{
        marker::{ChannelMarker, GuildMarker, MessageMarker, UserMarker},
        Id,
    },
};
use twilight_util::builder::embed::{
    EmbedAuthorBuilder, EmbedBuilder, EmbedFieldBuilder, ImageSource,
};
const MOYAI: &str = "🗿";

pub async fn reaction_count(
    channel_id: Id<ChannelMarker>,
    message_id: Id<MessageMarker>,
    author: Id<UserMarker>,
    http: &Arc<HttpClient>,
) -> anyhow::Result<i32> {
    let mut out = 0;
    let mut after = None;

    while let Ok(reactions) = {
        let mut tmp = http.reactions(channel_id, message_id, &Unicode { name: MOYAI });
        if let Some(id) = after {
            tmp = tmp.after(id)
        };
        tmp.await
    } {
        let models = reactions.models().await?;
        if models.is_empty() {
            break;
        }
        after = Some(
            models
                .last()
                .map(|u| u.id)
                .or(after)
                .expect("it not to fail"),
        );
        out += models.iter().filter(|u| !u.bot && u.id != author).count();
    }

    // if you assholes get this to fail i'm gonna be amazed
    Ok(out
        .try_into()
        .expect("there to be < 2147483647 reactions on a message"))
}

/// TODO: Get a better name.
async fn reactions_changed(
    msg: Id<MessageMarker>,
    channel: Id<ChannelMarker>,
    guild: Id<GuildMarker>,
    count: i32,
    db: &PgPool,
    ctx: &Context,
) -> anyhow::Result<()> {
    let settings: Settings = query_as("select * from settings where guild = $1")
        .bind(guild.to_string())
        .fetch_optional(db)
        .await?
        .unwrap_or(Settings {
            guild,
            board_threshold: 0,
            board_channel: None,
        });

    if settings.board_threshold == 0 {
        return Ok(());
    }
    let entry: Option<BoardEntry> = query_as("select * from board where message_id = $1")
        .bind(msg.to_string())
        .fetch_optional(db)
        .await?;

    if let Some(entry) = entry {
        if count >= settings.board_threshold.into() {
            update_post(entry, count, &settings, db, ctx).await
        } else {
            delete_post(entry, &settings, db, ctx).await
        }
    } else if count >= settings.board_threshold.into() {
        let message = ctx.http.message(channel, msg).await?.model().await?;
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
    sqlx::query!(
        "delete from board where post_id = $1",
        entry.post_id.to_string()
    )
    .execute(db)
    .await?;

    ctx.http
        .delete_message(board_channel, entry.post_id)
        .await?;

    Ok(())
}

async fn create_post(
    msg: Message,
    guild: Id<GuildMarker>,
    count: i32,
    settings: &Settings,
    db: &sqlx::Pool<sqlx::Postgres>,
    ctx: &Context,
) -> anyhow::Result<()> {
    let pfp = {
        let avatar = msg.author.avatar;
        if let Some(ava) = avatar {
            let ext = if ava.is_animated() { "gif" } else { "png" };
            ImageSource::url(format!(
                "https://cdn.discordapp.com/avatars/{}/{}.{}",
                msg.author.id, ava, ext
            ))
        } else {
            ImageSource::url(&ctx.pfp)
        }?
    };
    let mut embed = EmbedBuilder::new()
        .color(crate::EMBED_COLOR)
        .author(EmbedAuthorBuilder::new(msg.author.name).icon_url(pfp))
        .description(&msg.content)
        .field(EmbedFieldBuilder::new(
            "Source",
            format!(
                "[Jump](https://discord.com/channels/{}/{}/{})",
                guild, msg.channel_id, msg.id
            ),
        ));
    if let Some(attachment) = msg.attachments.first() {
        embed = embed.image(ImageSource::url(&attachment.url)?);
    };

    let post = match ctx
        .http
        .create_message(settings.board_channel.expect("We know the channel is set"))
        .content(&format!("{count} {MOYAI}"))?
        .embeds(&[embed.build()])?
        .await
    {
        Err(e) => {
            info!(?e);
            return Ok(());
        }
        Ok(m) => m.model().await?,
    };

    sqlx::query("insert into board values ($1, $2, $3, $4, $5, $6)")
        .bind(msg.content)
        .bind(guild.to_string())
        .bind(msg.channel_id.to_string())
        .bind(msg.id.to_string())
        .bind(post.id.to_string())
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
    let channel = settings.board_channel.unwrap();
    ctx.http
        .update_message(channel, entry.post_id)
        .content(Some(&format!("{count} {MOYAI}")))?
        .await?;

    sqlx::query!(
        "update board set stars = $1 where message_id = $2",
        count,
        entry.message_id.to_string(),
    )
    .execute(db)
    .await?;
    Ok(())
}

pub async fn add(reac: Box<ReactionAdd>, ctx: &Context) -> anyhow::Result<()> {
    let name = if let ReactionType::Unicode { name } = &reac.emoji {
        name
    } else {
        return Ok(());
    };
    if name != MOYAI || reac.guild_id.is_none() {
        return Ok(());
    }

    let count = reaction_count(
        reac.channel_id,
        reac.message_id,
        reac.message_author_id.expect("guh??"),
        &ctx.http,
    )
    .await?;

    reactions_changed(
        reac.message_id,
        reac.channel_id,
        reac.guild_id.unwrap(),
        count,
        &ctx.db,
        ctx,
    )
    .await
}

pub async fn remove(reac: Box<ReactionRemove>, ctx: &Context) -> anyhow::Result<()> {
    let name = if let ReactionType::Unicode { name } = &reac.emoji {
        name
    } else {
        return Ok(());
    };
    if name != MOYAI || reac.guild_id.is_none() {
        return Ok(());
    }

    // you know how the last function works? yeah this one doesn't and i have to do this shit
    let msg = ctx
        .http
        .message(reac.channel_id, reac.message_id)
        .await?
        .model()
        .await?;
    let count = reaction_count(reac.channel_id, reac.message_id, msg.author.id, &ctx.http).await?;

    reactions_changed(
        reac.message_id,
        reac.channel_id,
        reac.guild_id.unwrap(),
        count,
        &ctx.db,
        ctx,
    )
    .await
}

//pub async fn remove_all(reac: ReactionRemoveEmoji, ctx: &Context) -> anyhow::Result<()> {
//    todo!()
//}
