use crate::context::Context;
use twilight_model::{
    application::interaction::application_command::CommandData,
    gateway::payload::incoming::InteractionCreate, http::interaction::InteractionResponseData,
};
use twilight_util::builder::InteractionResponseDataBuilder;

pub async fn interaction(
    _data: &CommandData,
    _int: &InteractionCreate,
    ctx: &Context,
) -> anyhow::Result<InteractionResponseData> {
    let elapsed = ctx.start.elapsed();
    let uptime_str = format!(
        "Uptime: {}d {}h {}m {}s {}ms {}micros {}ns",
        elapsed.as_secs() / 3600 / 24,
        (elapsed.as_secs() / 3600) % 24,
        elapsed.as_secs() / 60,
        elapsed.as_secs() % 60,
        elapsed.as_millis() % 1000,
        elapsed.as_micros() % 1000,
        elapsed.as_nanos() % 1000,
    );

    Ok(InteractionResponseDataBuilder::new()
        .content(uptime_str)
        .build())
}
