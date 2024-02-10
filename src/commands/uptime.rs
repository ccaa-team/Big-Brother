use poise::{command, CreateReply};

use crate::{Context, Error};

#[command(slash_command, prefix_command)]
/// Show the bot's uptime
pub async fn uptime(ctx: Context<'_>) -> Result<(), Error> {
    // good luck fucking with the clock
    let now = chrono::Local::now();
    let start = &ctx.data().startup;
    let time = now.signed_duration_since(start);

    let time = format!(
        "Uptime: `{}d {}h {}m {}s {}ms` (started <t:{}>)",
        time.num_days(),
        time.num_hours() % 24,
        time.num_minutes() % 60,
        time.num_seconds() % 60,
        time.num_milliseconds() % 1000,
        start.timestamp()
    );

    let m = CreateReply::default().content(time);
    ctx.send(m).await?;

    Ok(())
}
