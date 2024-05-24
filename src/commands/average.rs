use poise::{command, say_reply};

use crate::{mommy, Context, Error};

#[command(prefix_command, ephemeral, track_edits)]
/// Calculate the average of an array of numbers, separated by spaces
///
/// Somehow doesn't work as a slash command
///
/// `;average 7 8 9`
pub async fn average(
    ctx: Context<'_>,
    #[description = "Array of numbers, separated by spaces"] array: Vec<f64>,
) -> Result<(), Error> {
    let size = array.len() as f64;
    let avg: f64 = array.iter().sum::<f64>() / size;

    say_reply(ctx, format!("{avg}\n{}", mommy::praise())).await?;

    Ok(())
}
