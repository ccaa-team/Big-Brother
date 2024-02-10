use poise::serenity_prelude::FullEvent as Event;
use poise::{serenity_prelude::Context, FrameworkContext};

use crate::{Data, Error};

use super::*;

pub async fn handler(
    ctx: &Context,
    event: &Event,
    _framework: FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        Event::ReactionAdd { add_reaction } => reaction_add(ctx, data, add_reaction).await,
        Event::ReactionRemove { removed_reaction } => {
            reaction_remove(ctx, data, removed_reaction).await
        }
        Event::Message { new_message } => message(ctx, data, new_message).await,
        _ => Ok(()),
    }?;
    Ok(())
}
