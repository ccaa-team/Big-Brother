use poise::command;

use crate::{mommy, Context, Error};

#[command(slash_command, prefix_command, track_edits)]
/// Get the bot's uptime, shrimple as that
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    let elapsed = ctx.data().start.elapsed();
    let out = format!(
        "Uptime: {}d {}h {}m {}s {}ms {}micros {}ns\n{}",
        elapsed.as_secs() / 3600 / 24,
        (elapsed.as_secs() / 3600) % 24,
        (elapsed.as_secs() / 60) % 60,
        elapsed.as_secs() % 60,
        elapsed.as_millis() % 1000,
        elapsed.as_micros() % 1000,
        elapsed.as_nanos() % 1000,
        mommy::praise()
    );

    poise::say_reply(ctx, out).await?;
    Ok(())
}
