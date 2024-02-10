use poise::{
    command,
    serenity_prelude::{CreateAllowedMentions, GetMessages},
    CreateReply,
};
use sqlx::{query, query_as};

use crate::{
    events::{create_entry, update_entry},
    structs::BoardEntry,
    Context, Error, MOYAI,
};

async fn viwty(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.author().id == 852877128844050432)
}

#[command(prefix_command, check = "viwty", guild_only)]
pub async fn scan(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(CreateReply::default().content("Scanning")).await?;

    query!("delete from moyai *")
        .execute(&ctx.data().db)
        .await?;

    let g = ctx.guild_id().unwrap();
    let channels = g.channels(ctx).await?;
    let channels = channels.iter().map(|c| c.0).collect::<Vec<_>>();

    for channel in channels.iter() {
        let mut total = vec![];
        let mut id = None;
        loop {
            let mut builder = GetMessages::new().limit(100);
            if let Some(id) = id {
                builder = builder.before(id);
            }
            let list = channel.messages(ctx, builder).await;
            let mut list = match list {
                Ok(l) => l,
                Err(_) => break,
            };
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
                match query_as!(BoardEntry, "select * from moyai where message_id = $1", id)
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

    let m = CreateReply::default()
        .content("Done")
        .allowed_mentions(CreateAllowedMentions::new().replied_user(true));

    ctx.send(m).await?;

    Ok(())
}
