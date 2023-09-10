use poise::serenity_prelude::{Context, Message, MessageId, MessageReaction, Reaction};
use sqlx::{query, query_as};

use crate::{
    globals::{self, CURSED_BOARD, MOYAI, THRESHOLD},
    structs::BoardEntry,
    Data, Error,
};

async fn create_post(
    ctx: &Context,
    data: &Data,
    count: &i64,
    msg: &Message,
) -> Result<MessageId, Error> {
    let msg = data
        .cursed_channel
        .send_message(ctx, |m| {
            m.content(&format!("{} {}", *count, MOYAI)).embed(|e| {
                e.author(|a| a.name(&msg.author.name).icon_url(msg.author.face()));
                if let Some(attachment) = msg.attachments.first() {
                    e.image(&attachment.url);
                };
                e.description(&msg.content)
                    .field("Source", format!("[jump]({})", msg.link()), false)
                    .color(0xAC00BB)
            })
        })
        .await?;
    Ok(msg.id)
}

pub async fn update_entry(
    ctx: &Context,
    data: &Data,
    post: &BoardEntry,
    msg: &Message,
    count: &i64,
) -> Result<(), Error> {
    sqlx::query!(
        "update moyai
         set moyai_count = ?
         where message_id = ?",
        count,
        post.message_id
    )
    .execute(&data.db)
    .await?;

    if *count as u64 >= globals::THRESHOLD && post.post_id.is_none() {
        let id = create_post(ctx, data, count, msg).await?;
        let id = id.to_string();
        sqlx::query!(
            "update moyai
             set post_id = ?
             where message_id = ?",
            id,
            post.message_id
        )
        .execute(&data.db)
        .await?;
    } else {
        let mut msg = ctx
            .http
            .get_message(
                CURSED_BOARD,
                post.post_id.as_ref().unwrap().parse().unwrap(),
            )
            .await?;
        msg.edit(ctx, |m| m.content(format!("{} {}", count, MOYAI)))
            .await?;
    }

    Ok(())
}

pub async fn create_entry(
    ctx: &Context,
    data: &Data,
    msg: &Message,
    reactions: &MessageReaction,
) -> Result<(), Error> {
    let id = msg.id.to_string();
    let (post, author) = if reactions.count >= globals::THRESHOLD {
        let id = create_post(ctx, data, &(reactions.count as i64), msg).await?;
        let id = id.to_string();
        let count = reactions.count as i64;
        sqlx::query!(
            "update moyai
            set moyai_count = ?
            where message_id = ?",
            count,
            id
        )
        .execute(&data.db)
        .await?;
        (Some(id), msg.author.name.to_owned())
    } else {
        (None, msg.author.name.to_owned())
    };
    let author = author.to_string();
    let count = reactions.count.to_string();
    sqlx::query!(
        "insert into moyai
        values(?, ?, ?, ?, ?)",
        id,
        post,
        msg.content,
        count,
        author
    )
    .execute(&data.db)
    .await?;

    Ok(())
}

pub async fn reaction_add(ctx: &Context, data: &Data, reaction: &Reaction) -> Result<(), Error> {
    if !reaction.emoji.unicode_eq(MOYAI) || reaction.channel_id == CURSED_BOARD {
        return Ok(());
    }
    let msg = reaction.message(ctx).await?;

    // there has to be a better way
    if let Some(reactions) = msg
        .reactions
        .iter()
        .filter(|r| r.reaction_type.unicode_eq(MOYAI))
        .collect::<Vec<_>>()
        .first()
    {
        let id = msg.id.to_string();
        let count = reactions.count as i64;
        match query_as!(BoardEntry, "select * from moyai where message_id = ?", id)
            .fetch_optional(&data.db)
            .await?
        {
            Some(p) => update_entry(ctx, data, &p, &msg, &count).await?,
            None => create_entry(ctx, data, &msg, reactions).await?,
        };
    };

    Ok(())
}

pub async fn reaction_remove(ctx: &Context, data: &Data, reaction: &Reaction) -> Result<(), Error> {
    if !reaction.emoji.unicode_eq(MOYAI) {
        return Ok(());
    }
    let msg = reaction.message(ctx).await?;
    let id = msg.id.to_string();

    let post = query_as!(BoardEntry, "select * from moyai where message_id = ?", id)
        .fetch_optional(&data.db)
        .await?;
    if post.is_none() {
        // It's not even in the db :tf:
        return Ok(());
    }

    // This is guaranteed to be Some()
    if let Some(post_id) = post.unwrap().post_id {
        let post = ctx
            .http
            .get_message(CURSED_BOARD, post_id.parse().unwrap())
            .await?;
        let reactions = msg
            .reactions
            .iter()
            .filter(|r| r.reaction_type.unicode_eq(MOYAI))
            .collect::<Vec<_>>();
        let reactions = reactions.first();
        if let Some(reactions) = reactions {
            if reactions.count < THRESHOLD {
                post.delete(ctx).await?;
                query!(
                    "update moyai
                    set post_id = ?
                    where message_id = ?",
                    None::<String>,
                    id
                )
                .execute(&data.db)
                .await?;
            } else {
                let count = reactions.count as i64;
                query!(
                    "update moyai
                    set moyai_count = ?
                    where message_id = ?",
                    count,
                    id
                )
                .execute(&data.db)
                .await?;
                let mut msg = ctx
                    .http
                    .get_message(CURSED_BOARD, post_id.parse().unwrap())
                    .await?;
                msg.edit(ctx, |m| m.content(format!("{} {}", count, MOYAI)))
                    .await?;
            }
        } else {
            post.delete(ctx).await?;
            query!(
                "update moyai
                    set post_id = ?
                    where message_id = ?",
                None::<String>,
                id
            )
            .execute(&data.db)
            .await?;
        }
    }

    Ok(())
}