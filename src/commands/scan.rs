use poise::command;
use sqlx::query_as;

use crate::{
    events::{create_entry, update_entry},
    globals::{MOYAI, VIRT_ID},
    structs::BoardEntry,
    Context, Error,
};

async fn viwty(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author().id == VIRT_ID)
}

#[command(prefix_command, check = "viwty", guild_only)]
pub async fn scan(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|m| m.content("Scanning")).await?;

    let g = ctx.guild_id().unwrap();
    let channels = g.channels(ctx).await?;
    let channels = channels.iter().map(|c| c.0).collect::<Vec<_>>();

    for channel in channels.iter() {
        let mut total = vec![];
        let mut id = None;
        loop {
            let mut list = channel
                .messages(ctx, |l| {
                    l.limit(100);
                    if let Some(id) = id {
                        l.before(id);
                    }
                    l
                })
                .await?;
            if list.is_empty() {
                break;
            }
            id = Some(list.last().unwrap().id);
            list.reverse();
            total.push(list)
        }
        total.reverse();

        for msg in total.iter().flatten() {
            let reactions = msg
                .reactions
                .iter()
                .filter(|r| r.reaction_type.unicode_eq(MOYAI))
                .collect::<Vec<_>>();
            if let Some(reactions) = reactions.first() {
                let id = msg.id.to_string();
                let count = reactions.count as i64;
                match query_as!(BoardEntry, "select * from moyai where message_id = ?", id)
                    .fetch_optional(&ctx.data().db)
                    .await?
                {
                    Some(p) => {
                        update_entry(ctx.serenity_context(), ctx.data(), &p, msg, &count).await?
                    }
                    None => {
                        create_entry(ctx.serenity_context(), ctx.data(), msg, reactions).await?
                    }
                };
            }
        }
    }

    ctx.send(|m| {
        m.content("Done.")
            .reply(true)
            .allowed_mentions(|m| m.replied_user(true))
    })
    .await?;

    Ok(())
}
