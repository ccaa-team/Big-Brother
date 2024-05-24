use crate::Data;
use poise::serenity_prelude::{Context, Message};

pub async fn handle(ctx: &Context, data: &Data, msg: &Message) -> anyhow::Result<()> {
    if msg.author.bot || msg.guild_id.is_none() {
        return Ok(());
    }

    let content = msg.content.to_lowercase();
    let guild = msg.guild_id.unwrap();
    let mut out = String::new();
    data.rules
        .read()
        .unwrap()
        .iter()
        .filter(|r| r.guild == guild && content.contains(&r.trigger))
        .for_each(|s| {
            out += &s.reply;
            out += " ";
        });

    if out.is_empty() {
        return Ok(());
    }

    msg.reply(&ctx.http, out).await?;

    Ok(())
}
