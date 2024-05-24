use std::time::Instant;

use poise::command;
use poise::serenity_prelude::{CreateEmbed, CreateEmbedFooter};
use sqlx::postgres::PgRow;
use sqlx::query;
use sqlx::{Column, Row};

use crate::utils::EMBED_COLOR;
use crate::{mommy, Context, Error};

macro_rules! kms {
    ($row:expr, $name:expr, $t:ty) => {
        $row.try_get::<$t, _>($name).ok().map(|v| v.to_string())
    };
}

fn stringify(row: &PgRow, name: impl AsRef<str>) -> String {
    let name = name.as_ref();
    row.try_get::<String, _>(name)
        .ok()
        .or_else(|| kms!(row, name, i32))
        .or_else(|| Some("None".to_owned()))
        .map(|s| {
            if s.starts_with("http") {
                format!("<{s}>")
            } else {
                s
            }
        })
        .unwrap()
}

#[command(prefix_command, owners_only, hide_in_help, track_edits)]
pub async fn sql(ctx: Context<'_>, #[rest] sql: String) -> Result<(), Error> {
    let start = Instant::now();
    let result = query(&sql).fetch_all(&ctx.data().db).await?;
    let elapsed = start.elapsed();

    let mut out = String::new();

    for (i, row) in result.iter().enumerate() {
        out += &format!("{}:\n", i);
        for c in row.columns() {
            out += &format!("  {} -> {}\n", c.name(), stringify(row, c.name()));
        }
    }

    let mut iter = out.chars().peekable();
    while iter.peek().is_some() {
        let res: String = iter.by_ref().take(4096).collect();
        poise::send_reply(
            ctx,
            poise::CreateReply::default()
                .embed(
                    CreateEmbed::new()
                        .color(EMBED_COLOR)
                        .footer(CreateEmbedFooter::new(format!(
                            "Query took {}micros",
                            elapsed.as_micros()
                        )))
                        .description(res),
                )
                .content(mommy::praise()),
        )
        .await?;
    }

    Ok(())
}
