use poise::command;

use crate::{Context, Error};

#[command(slash_command, prefix_command, track_edits)]
/// Believe it or not, this is a calculator command
pub async fn calc(ctx: Context<'_>, #[rest] expr: String) -> Result<(), Error> {
    let result = match mexprp::eval::<f64>(&expr) {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    };

    ctx.send(|m| m.content(result).reply(true).ephemeral(true))
        .await?;

    Ok(())
}
