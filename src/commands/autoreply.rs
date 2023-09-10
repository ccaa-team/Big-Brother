use poise::command;
use sqlx::{query, query_as};

use crate::{structs::AutoReply, Context, Error};

#[command(slash_command, subcommands("list", "add", "remove"))]
pub async fn autoreply(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!();
}

#[command(slash_command, prefix_command)]
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let rules = query_as!(AutoReply, "select * from replies")
        .fetch_all(&ctx.data().db)
        .await?;

    let list = rules
        .iter()
        .map(|r| format!("* {}\n  * {}", &r.trigger, &r.reply))
        .reduce(|a, b| format!("{}\n{}", a, b))
        .unwrap_or_else(|| "No autoreply rules found.".to_string());

    ctx.send(|m| {
        m.embed(|e| {
            e.title("AutoReply Rules")
                .author(|a| a.icon_url(&ctx.data().bot_pfp))
                .description(list)
        })
    })
    .await?;

    Ok(())
}

#[command(slash_command, prefix_command)]
async fn add(ctx: Context<'_>, trigger: String, #[rest] reply: String) -> Result<(), Error> {
    let result = query!(
        "insert into replies
        values(?, ?)",
        trigger,
        reply
    )
    .execute(&ctx.data().db)
    .await;

    ctx.send(|m| {
        m.content(match result {
            Ok(_) => format!("Added rule `{}`", trigger),
            Err(e) => {
                format!("Failed to add rule: ```rs\n{}\n```", e.to_string())
            }
        })
        .ephemeral(true)
    })
    .await?;

    Ok(())
}

#[command(slash_command, prefix_command)]
async fn remove(ctx: Context<'_>, trigger: String) -> Result<(), Error> {
    let result = query!(
        "delete from replies
        where trigger = ?",
        trigger
    )
    .execute(&ctx.data().db)
    .await?;

    ctx.send(|m| {
        m.content(if result.rows_affected() == 0 {
            "Failed to remove a rule, does it exist?".to_string()
        } else {
            format!("Removed rule `{}`", trigger)
        })
        .ephemeral(true)
    })
    .await?;

    Ok(())
}