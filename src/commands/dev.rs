use sqlx::postgres::PgRow;
use sqlx::query;
use sqlx::{Column, Row};
use twilight_model::gateway::payload::incoming::MessageCreate;

use crate::{context::Context, utils::OWNER_ID};

macro_rules! kms {
    ($row:expr, $name:expr, $t:ty) => {
        $row.try_get::<$t, _>($name).ok().map(|v| v.to_string())
    };
}

fn stringify<T: AsRef<str>>(row: &PgRow, name: T) -> String {
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

async fn squeel<T: AsRef<str>>(sql: T, ctx: &Context) -> anyhow::Result<String> {
    let result = match query(sql.as_ref()).fetch_all(&ctx.db).await {
        Ok(v) => v,
        Err(e) => return Ok(format!("```\n{e}\n```")),
    };
    let mut out = String::from("```\n");

    for (i, row) in result.iter().enumerate() {
        out += &format!("{}:\n", i);
        for c in row.columns() {
            out += &format!("  {} -> {}\n", c.name(), stringify(row, c.name()));
        }
    }

    out.push_str("```");

    Ok(out)
}

pub async fn handle(msg: &MessageCreate, ctx: &Context) -> anyhow::Result<()> {
    if msg.author.id != OWNER_ID {
        return Ok(());
    }

    if let Some((cmd, arg)) = msg.content.split_once(" ") {
        let out = match cmd {
            ";sql" => squeel(arg, ctx).await?,
            _ => return Ok(()),
        };

        let mut iter = out.chars().peekable();
        while iter.peek().is_some() {
            let res: String = iter.by_ref().take(2000).collect();
            ctx.http
                .create_message(msg.channel_id)
                .reply(msg.id)
                .content(&res)
                .unwrap()
                .await?;
        }
    }

    Ok(())
}
