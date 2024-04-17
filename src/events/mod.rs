mod autoreply;
mod reactions;
use crate::Context;
use twilight_gateway::Event;

pub async fn handle(event: Event, ctx: &Context) -> anyhow::Result<()> {
    match event {
        Event::MessageCreate(msg) => autoreply::handle(msg, ctx).await,
        Event::ReactionAdd(reaction) => reactions::add(reaction, ctx).await,
        Event::ReactionRemove(reaction) => reactions::remove(reaction, ctx).await,
        // I don't think this will appear
        //Event::ReactionRemoveAll(_) => todo!(),
        //Event::ReactionRemoveEmoji(reaction) => reactions::remove_all(reaction, ctx).await,
        _ => Ok(()),
    }
}
