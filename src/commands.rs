use std::time::Duration;

use poise::serenity_prelude::{self as serenity, AttachmentType};
use poise::serenity_prelude::{CacheHttp, ChannelId};

use crate::{db, Context, Entry, Error};

// https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters#comment64327244_38461429
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[poise::command(slash_command)]
pub async fn top10moyai(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;
    let list = db::list().await?;

    let list = async {
        let mut out = vec![];
        let mut i = 1;
        for e in list.iter() {
            if let (Ok(chn_id), Ok(msg_id)) = (e.channel_id.parse(), e.msg_id.parse::<u64>()) {
                let channel = ChannelId(chn_id);
                if let Ok(msg) = channel.message(ctx.http(), msg_id).await {
                    let author = msg.author.name.to_string();
                    let content = if msg.content.len() > 127 {
                        format!("{}...", truncate(&msg.content, 128))
                    } else {
                        msg.content.to_string()
                    };
                    let jump = msg.link().to_string();
                    let place = i;
                    let count = e.moyai_count;
                    let val = Entry {
                        author,
                        content,
                        jump,
                        place,
                        count,
                    };
                    out.push(val);
                }
                i += 1;
            };
        }
        out
    }
    .await;

    ctx.send(|m| {
        m.embed(|e| {
            for entry in list.iter() {
                e.field(
                    format!(
                        "{}: #{} - {}:moyai:",
                        entry.author, entry.place, entry.count
                    ),
                    format!("{}\n[Jump]({})", entry.content, entry.jump),
                    false,
                );
            }
            e.color(serenity::Colour(0xAA00BB))
        })
    })
    .await?;

    Ok(())
}

use crate::uwu::uwuify;

#[poise::command(slash_command)]
pub async fn uwu(
    ctx: Context<'_>,
    #[rest]
    #[description = "Text to uwuify"]
    text: String,
) -> Result<(), Error> {
    let reply = ctx.send(|r| r.content("ok").ephemeral(true)).await?;

    let (name, avatar_url) = ctx
        .author_member()
        .await
        .map(|member| (member.display_name().to_string(), member.face()))
        .unwrap_or_else(|| {
            let user = ctx.author();
            (user.name.to_owned(), user.face())
        });

    let channel_id = ctx.channel_id();
    let webhook = match channel_id
        .webhooks(&ctx.http())
        .await?
        .into_iter()
        .find(|w| w.token.is_some())
    {
        Some(webhook) => webhook,
        None => {
            channel_id
                .create_webhook(&ctx.http(), "uwu webhook")
                .await?
        }
    };

    let content = uwuify(text);
    webhook
        .execute(&ctx.http(), false, |m| {
            m.avatar_url(avatar_url).username(uwuify(name));

            if content.len() <= 2000 {
                m.content(content)
            } else {
                m.add_file(AttachmentType::Bytes {
                    data: std::borrow::Cow::Owned(content.into()),
                    filename: "uwu.txt".to_string(),
                })
            }
        })
        .await?;

    reply.delete(ctx).await?;

    Ok(())
}

#[poise::command(
    context_menu_command = "Embrace",
    guild_only,
    required_permissions = "MANAGE_ROLES"
)]
pub async fn embrace(ctx: Context<'_>, mem: serenity::User) -> Result<(), Error> {
    let gid = ctx.guild_id().expect("what");

    if gid == serenity::GuildId(1023332212403351563) {
        let mut member = gid.member(&ctx.http(), mem.id).await?;
        member.add_role(&ctx.http(), 1023334181952049203).await?;
        ctx.send(|m| m.content("Done!").ephemeral(true)).await?;
    } else {
        ctx.send(|m| {
            m.content("Sorry, but this command only works in a specific guild.")
                .ephemeral(true)
        })
        .await?;
    }

    Ok(())
}

// this is for some reason needed
#[allow(dead_code)]
pub async fn crazy_check(ctx: Context<'_>) -> Result<bool, Error> {
    Ok(ctx.invocation_string() == "crazy")
}

#[poise::command(prefix_command, check = "crazy_check")]
pub async fn crazy(ctx: Context<'_>) -> Result<(), Error> {
    let send = async move |msg| -> Result<(), Error> {
        ctx.send(|m| m.content(msg)).await?;
        Ok(())
    };
    send("Crazy?").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    send("I was crazy once.").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    send("They locked me in a room.").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    send("A rubber room.").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    send("A rubber room with rats.").await?;
    tokio::time::sleep(Duration::from_secs(1)).await;
    send("And rats make me crazy.").await?;

    Ok(())
}
