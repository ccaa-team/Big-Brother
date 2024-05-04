use twilight_model::gateway::payload::incoming::{PresenceUpdate, TypingStart};

use crate::{context::Context, utils::OWNER_ID};

pub async fn presense_update(e: Box<PresenceUpdate>, ctx: &Context) -> anyhow::Result<()> {
    //if e.user.id() != OWNER_ID {
    //    return Ok(());
    //}

    //tracing::info!(?e);

    Ok(())
}

pub async fn typing_start(e: Box<TypingStart>, ctx: &Context) -> anyhow::Result<()> {
    tracing::info!(?e);

    Ok(())
}
