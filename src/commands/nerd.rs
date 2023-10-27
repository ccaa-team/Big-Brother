use poise::{command, serenity_prelude::Message};

use crate::{Context, Error};

#[command(context_menu_command = "Nerd")]
pub async fn nerd(ctx: Context<'_>, msg: Message) -> Result<(), Error> {
    let content = format!("\"{}\"\n\\- :nerd:", msg.content);

    ctx.say(content).await?;

    Ok(())
}
