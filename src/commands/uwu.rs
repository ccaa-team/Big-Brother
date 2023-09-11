use crate::{Context, Error};
use poise::{command, serenity_prelude::Webhook};

#[command(slash_command, prefix_command, guild_only)]
pub async fn uwu(ctx: Context<'_>, #[rest] text: String) -> Result<(), Error> {
    let out = crate::uwu::uwuify(text);
    if ctx.prefix() == "/" {
        ctx.send(|m| m.ephemeral(true).content("uwu")).await?;
    }

    let hook = match get_webhook(ctx).await {
        Ok(hook) => hook,
        Err(e) => {
            let content = format!("Unable to find/create webhook: {}", e);
            ctx.send(|m| m.ephemeral(true).content(content)).await?;
            return Ok(());
        }
    };

    let user = match ctx.author_member().await {
        Some(u) => u,
        None => unreachable!(),
    };

    hook.execute(ctx, false, |m| {
        m.username(user.display_name())
            .avatar_url(user.face())
            .content(out)
    })
    .await?;

    Ok(())
}

async fn create_webhook(ctx: Context<'_>) -> Result<Webhook, Error> {
    let channel = ctx.channel_id();

    let mut hook = channel.create_webhook(ctx, "AutoVirt: uwu").await?;
    if let Some(pfp) = &ctx.data().bot_pfp {
        hook.edit_avatar(ctx, pfp.as_str()).await?;
    }

    Ok(hook)
}

async fn get_webhook(ctx: Context<'_>) -> Result<Webhook, Error> {
    let hooks = ctx.channel_id().webhooks(&ctx).await?;

    let hook = match hooks.iter().find(|w| w.token.is_some()) {
        Some(w) => Ok(w.to_owned()),
        None => create_webhook(ctx).await,
    };

    hook
}
