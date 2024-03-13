use crate::{structs::AutoReply, Context, Error};
use poise::{
    command,
    serenity_prelude::{CreateEmbed, CreateEmbedAuthor},
    CreateReply,
};
use sqlx::{query, query_as};

#[command(slash_command, subcommands("list", "add", "remove"))]
pub async fn autoreply(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!();
}

#[command(slash_command, prefix_command)]
/// List all the autoreply rules
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let replies = ctx.data().autoreplies.read().await;
    let list = replies
        .iter()
        .map(|r| format!("* {}\n  * {}", &r.trigger, &r.reply))
        .reduce(|a, b| format!("{}\n{}", a, b))
        .unwrap_or_else(|| "No autoreply rules found.".to_string());
    drop(replies);

    let mut e = CreateEmbed::new()
        .title("AutoReply rules")
        .description(list);
    if let Some(pfp) = &ctx.data().bot_pfp {
        e = e.author(CreateEmbedAuthor::new("").icon_url(pfp));
    }
    let m = CreateReply::default().embed(e);
    ctx.send(m).await?;

    Ok(())
}

#[command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES"
)]
/// Add an autoreply rule
async fn add(ctx: Context<'_>, trigger: String, #[rest] reply: String) -> Result<(), Error> {
    let result = query!(
        "insert into replies
        values($1, $2)",
        trigger,
        reply
    )
    .execute(&ctx.data().db)
    .await;

    let m = CreateReply::default()
        .content(match result {
            Ok(_) => {
                let out = format!("Added rule `{}`", &trigger);
                let mut list = ctx.data().autoreplies.write().await;
                list.push(AutoReply { trigger, reply });
                out
            }
            Err(e) => {
                format!("Failed to add rule: ```rs\n{}\n```", e)
            }
        })
        .ephemeral(true);

    ctx.send(m).await?;

    Ok(())
}

#[command(
    slash_command,
    prefix_command,
    required_permissions = "MANAGE_MESSAGES"
)]
/// Remove an autoreply rule
async fn remove(ctx: Context<'_>, trigger: String) -> Result<(), Error> {
    let result = query!(
        "delete from replies
        where trigger = $1",
        trigger
    )
    .execute(&ctx.data().db)
    .await?;

    let m = CreateReply::default()
        .content(if result.rows_affected() == 0 {
            "Failed to remove a rule, does it exist?".to_string()
        } else {
            // just fetch that shit, it's not worth it
            let data: Vec<AutoReply> = query_as!(AutoReply, "select * from replies")
                .fetch_all(&ctx.data().db)
                .await?;
            *ctx.data().autoreplies.write().await = data;
            format!("Removed rule `{}`", trigger)
        })
        .ephemeral(true);

    ctx.send(m).await?;

    Ok(())
}
