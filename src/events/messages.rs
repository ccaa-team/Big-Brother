use poise::serenity_prelude::{Context, Message};
use rand::Rng;

use crate::{structs::Data, Error};

pub async fn message(ctx: &Context, data: &Data, msg: &Message) -> Result<(), Error> {
    if msg.author.bot {
        return Ok(());
    }

    let mut out = String::new();

    if rand::thread_rng().gen_ratio(1, 5000) {
        out += "*pees in your ass*";
    }

    let replies = data.autoreplies.lock().await;
    let content = replies
        .iter()
        .filter(|r| msg.content.contains(&r.trigger))
        .map(|r| r.reply.as_str())
        .collect::<Vec<_>>()
        .join(" ");
    drop(replies);

    out.push_str(&content);

    if !out.is_empty() {
        msg.reply(ctx, out).await?;
    }

    Ok(())
}
