use tokio::time::Instant;
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::context::Context;

pub async fn handle(msg: Box<MessageCreate>, ctx: &Context) -> anyhow::Result<()> {
    if msg.author.bot {
        return Ok(());
    }
    let start = Instant::now();

    let out = ctx
        .data
        .read()
        .await
        .rules
        .iter()
        .filter(|r| msg.content.contains(&r.trigger))
        .map(|r| r.reply.clone())
        .collect::<Vec<String>>()
        .join(" ");
    if out.is_empty() {
        return Ok(());
    }

    let runtime = start.elapsed().as_nanos();

    ctx.http
        .create_message(msg.channel_id)
        .content(&format!("{}\n\nProcessed in {} ns", out, runtime))?
        .reply(msg.id)
        .allowed_mentions(None)
        .await?;

    Ok(())
}
