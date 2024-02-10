use crate::{Context, Error};
use poise::{
    command,
    serenity_prelude::{
        CreateAttachment, CreateWebhook, ExecuteWebhook, Webhook,
    },
    CreateReply,
};

#[command(slash_command, prefix_command, guild_only)]
/// uwuify a string
pub async fn uwu(ctx: Context<'_>, #[rest] text: String) -> Result<(), Error> {
    let out = crate::uwu::uwuify(text);
    if ctx.prefix() == "/" {
        let m = CreateReply::default().content(";").ephemeral(true);
        ctx.send(m).await?;
    }

    let hook = match get_webhook(ctx).await {
        Ok(hook) => hook,
        Err(e) => {
            let content = format!("Unable to find/create webhook: {}", e);
            let message = CreateReply::default().content(content).ephemeral(true);
            ctx.send(message).await?;
            return Ok(());
        }
    };

    let user = match ctx.author_member().await {
        Some(u) => u,
        None => unreachable!(),
    };

    let m = ExecuteWebhook::new()
        .username(user.display_name())
        .avatar_url(user.face())
        .content(out);

    hook.execute(ctx, false, m).await?;

    Ok(())
}

async fn create_webhook(ctx: Context<'_>) -> Result<Webhook, Error> {
    let channel = ctx.channel_id();

    let mut create_hook = CreateWebhook::new("AutoVirt: uwu");

    if let Some(pfp) = &ctx.data().bot_pfp {
        let attachment = CreateAttachment::url(ctx, pfp).await?;
        create_hook = create_hook.avatar(&attachment);
    }

    let hook = channel.create_webhook(ctx, create_hook).await?;

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
