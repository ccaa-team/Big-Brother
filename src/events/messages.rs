use poise::serenity_prelude::{Context, Message};
use rand::Rng;
use sqlx::query_as;

use crate::{
    structs::{AutoReply, Data},
    Error,
};

pub async fn message(ctx: &Context, data: &Data, msg: &Message) -> Result<(), Error> {
    if msg.author.id == data.bot.id {
        return Ok(());
    }

    let rules = query_as!(AutoReply, "select * from replies")
        .fetch_all(&data.db)
        .await?;

    let mut out = String::new();

    if rand::thread_rng().gen_ratio(1, 100) {
        out += "*pees in your ass*";
    }

    let content = rules
        .iter()
        .filter(|r| msg.content.contains(&r.trigger))
        .map(|r| r.reply.as_str())
        .collect::<Vec<_>>()
        .join(" ");

    out.push_str(&content);

    if !out.is_empty() {
        msg.reply(ctx, out).await?;
    }

    Ok(())
}
