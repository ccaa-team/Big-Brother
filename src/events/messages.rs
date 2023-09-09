use poise::serenity_prelude::{Context, Message};
use rand::Rng;
use sqlx::query_as;

use crate::{
    structs::{AutoReply, Data},
    Error,
};

pub async fn message(ctx: &Context, data: &Data, msg: &Message) -> Result<(), Error> {
    if msg.author.bot {
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
        // i'll find a better way later:tm:
        .map(|r| r.reply.clone())
        .reduce(|a, b| format!("{a} {b}"))
        .unwrap_or_else(|| "".to_string());

    out += &content;

    if !out.is_empty() {
        msg.reply(ctx, out).await?;
    }

    Ok(())
}
