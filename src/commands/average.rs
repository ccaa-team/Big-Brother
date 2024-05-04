use poise::{command, say_reply};

use crate::{Context, Error};

#[command(slash_command, prefix_command, ephemeral)]
pub async fn average(
    ctx: Context<'_>,
    #[description = "Array of numbers, separated by spaces"] array: Vec<f64>,
) -> Result<(), Error> {
    let size = array.len() as f64;
    let avg: f64 = array.iter().sum::<f64>() / size;
    say_reply(ctx, avg.to_string()).await?;

    Ok(())
}
