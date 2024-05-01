mod autoreply;
mod reactions;
use crate::{commands, Context};
use twilight_gateway::Event;

pub async fn handle(event: Event, ctx: &Context) -> anyhow::Result<()> {
    match event {
        Event::MessageCreate(msg) => {
            commands::dev::handle(&msg, ctx).await?;
            autoreply::handle(&msg, ctx).await
        }
        Event::ReactionAdd(reaction) => reactions::add(reaction, ctx).await,
        Event::ReactionRemove(reaction) => reactions::remove(reaction, ctx).await,

        _ => Ok(()),
    }
}
