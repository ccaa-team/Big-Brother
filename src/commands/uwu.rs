use crate::{Context, Error};
use poise::{command, serenity_prelude::Webhook};

#[command(slash_command, prefix_command, guild_only)]
pub async fn uwu(ctx: Context<'_>, #[rest] text: String) -> Result<(), Error> {
    let out = crate::uwu::uwuify(text);
    if ctx.prefix() == "/" {
        ctx.send(|m| m.ephemeral(true).content("uwu")).await?;
    }

    let hook = if let Ok(hook) = get_webhook(ctx).await {
        hook
    } else {
        ctx.send(|m| m.ephemeral(true).content("Unable to find/create webhook"))
            .await?;
        return Ok(());
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
    hook.edit_avatar(ctx, ctx.data().bot_pfp.as_str()).await?;

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
