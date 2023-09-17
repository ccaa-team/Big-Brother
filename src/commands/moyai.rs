use std::vec;

use poise::command;
use sqlx::query_as;

use crate::{structs::BoardEntry, Context, Error, MOYAI};

#[command(slash_command, subcommands("list", "top"))]
pub async fn moyai(_ctx: Context<'_>) -> Result<(), Error> {
    unreachable!();
}

// https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters#comment64327244_38461429
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[command(slash_command, prefix_command)]
/// List all the moyai board entries that fit the threshold
async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let list = query_as!(BoardEntry, "select * from moyai")
        .fetch_all(&ctx.data().db)
        .await?;
    let list: Vec<_> = list
        .iter()
        .filter(|p| p.moyai_count >= ctx.data().threshold as i64)
        .collect();
    ctx.send(|m| {
        for c in list.chunks(25) {
            m.embed(|e| {
                e.fields(c.iter().map(|p| {
                    (
                        format!("{}: {} {}", p.author, p.moyai_count, MOYAI),
                        truncate(&p.message_content, 128),
                        false,
                    )
                }))
            });
        }
        m
    })
    .await?;

    Ok(())
}

#[command(slash_command, prefix_command)]
/// List the top moyai board entries (defaults to 10)
async fn top(ctx: Context<'_>, amount: Option<i64>) -> Result<(), Error> {
    let amount = amount.unwrap_or(10);

    let entries = query_as!(
        BoardEntry,
        "select *
        from moyai
        order by moyai_count desc
        limit $1",
        amount
    )
    .fetch_all(&ctx.data().db)
    .await?;

    let fields = entries.iter().map(|e| {
        (
            format!("{}: {} {}", e.author, e.moyai_count, MOYAI),
            &e.message_content,
            false,
        )
    });
    ctx.send(|m| m.embed(|e| e.fields(fields))).await?;

    Ok(())
}
