use poise::command;

use crate::{Context, Error};

#[command(slash_command, prefix_command)]
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let elapsed = ctx.data().start.elapsed();
    let out = format!(
        "Uptime: {}d {}h {}m {}s {}ms {}micros {}ns",
        elapsed.as_secs() / 3600 / 24,
        (elapsed.as_secs() / 3600) % 24,
        (elapsed.as_secs() / 60) % 60,
        elapsed.as_secs() % 60,
        elapsed.as_millis() % 1000,
        elapsed.as_micros() % 1000,
        elapsed.as_nanos() % 1000,
    );

    poise::say_reply(ctx, out).await?;
    Ok(())
}
