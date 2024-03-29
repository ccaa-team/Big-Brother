use poise::serenity_prelude::{
    Context, CreateEmbed, CreateEmbedAuthor, CreateMessage, EditMessage,
    Message, MessageId, MessageReaction, Reaction,
};
use sqlx::{query, query_as};

use crate::{structs::BoardEntry, Data, Error, MOYAI};

async fn create_post(
    ctx: &Context,
    data: &Data,
    count: &i64,
    msg: &Message,
) -> Result<MessageId, Error> {
    let author = CreateEmbedAuthor::new(&msg.author.name).icon_url(msg.author.face());
    let mut embed = CreateEmbed::new()
        .author(author)
        .description(&msg.content)
        .field("Source", format!("[jump]({})", msg.link()), false)
        .color(0xAC00BB);
    if let Some(attachment) = msg.attachments.first() {
        embed = embed.image(&attachment.url);
    }
    let message = CreateMessage::new()
        .content(format!("{} {}", *count, MOYAI))
        .embed(embed);
    let msg = data.cursed_channel.send_message(ctx, message).await?;
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
         set moyai_count = $1
         where message_id = $2",
        count,
        post.message_id
    )
    .execute(&data.db)
    .await?;

    if *count as u64 >= data.threshold && post.post_id.is_none() {
        let id = create_post(ctx, data, count, msg).await?;
        let id = id.to_string();
        sqlx::query!(
            "update moyai
             set post_id = $1
             where message_id = $2",
            id,
            post.message_id
        )
        .execute(&data.db)
        .await?;
    } else if post.post_id.is_some() {
        let post_id: u64 = post.post_id.clone().unwrap().parse().unwrap();
        let mut msg = data.cursed_channel.message(ctx, post_id).await?;
        msg.edit(
            ctx,
            EditMessage::new().content(format!("{} {}", count, MOYAI)),
        )
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
    let count = reactions.count as i64;
    let id = msg.id.to_string();
    let (post, author) = if reactions.count >= data.threshold {
        let id = create_post(ctx, data, &(reactions.count as i64), msg).await?;
        let id = id.to_string();
        sqlx::query!(
            "update moyai
            set moyai_count = $1
            where message_id = $2",
            count,
            id
        )
        .execute(&data.db)
        .await?;
        (Some(id), msg.author.name.to_owned())
    } else {
        (None, msg.author.name.to_owned())
    };
    sqlx::query!(
        "insert into moyai
        values($1, $2, $3, $4, $5)",
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
    if !reaction.emoji.unicode_eq(MOYAI) || reaction.channel_id == data.cursed_channel {
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
        match query_as!(BoardEntry, "select * from moyai where message_id = $1", id)
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

    let post = query_as!(BoardEntry, "select * from moyai where message_id = $1", id)
        .fetch_optional(&data.db)
        .await?;
    if post.is_none() {
        // It's not even in the db :tf:
        return Ok(());
    }

    // This is guaranteed to be Some()
    if let Some(post_id) = post.unwrap().post_id {
        let post_id: u64 = post_id.parse().unwrap();
        let post = data.cursed_channel.message(ctx, post_id).await?;
        let reactions = msg
            .reactions
            .iter()
            .filter(|r| r.reaction_type.unicode_eq(MOYAI))
            .collect::<Vec<_>>();
        let reactions = reactions.first();
        if let Some(reactions) = reactions {
            if reactions.count < data.threshold {
                post.delete(ctx).await?;
                query!(
                    "update moyai
                    set post_id = $1
                    where message_id = $2",
                    None::<String>,
                    id
                )
                .execute(&data.db)
                .await?;
            } else {
                let count = reactions.count as i64;
                query!(
                    "update moyai
                    set moyai_count = $1
                    where message_id = $2",
                    count,
                    id
                )
                .execute(&data.db)
                .await?;
                let mut msg = data.cursed_channel.message(ctx, post_id).await?;
                msg.edit(
                    ctx,
                    EditMessage::new().content(format!("{} {}", count, MOYAI)),
                )
                .await?;
            }
        } else {
            post.delete(ctx).await?;
            query!(
                "update moyai
                    set post_id = $1
                    where message_id = $2",
                None::<String>,
                id
            )
            .execute(&data.db)
            .await?;
        }
    }

    Ok(())
}
