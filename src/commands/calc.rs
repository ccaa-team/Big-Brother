use poise::command;

use crate::{Context, Error};

fn meth(expr: String) -> String {
    let mut ctx: mexprp::Context<f64> = mexprp::Context::new();
    ctx.cfg.precision = 1024;

    match mexprp::eval_ctx(&expr, &ctx) {
        Ok(r) => r.to_string(),
        Err(e) => e.to_string(),
    }
}

#[command(slash_command, prefix_command, track_edits)]
/// Believe it or not, this is a calculator command
pub async fn calc(ctx: Context<'_>, #[rest] expr: String) -> Result<(), Error> {
    let result = meth(expr);

    ctx.send(|m| m.content(result).reply(true).ephemeral(true))
        .await?;

    Ok(())
}
