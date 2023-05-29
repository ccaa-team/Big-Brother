mod db;
mod globals;
mod uwu;

use std::{fs::File, io::Read, path::Path};

use poise::serenity_prelude::{
    self as serenity, AttachmentType, CacheHttp, Channel, ChannelId, GatewayIntents, Message,
    Reaction,
};

use globals::*;

struct Data {
    pub bot_pfp: Option<String>,
}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use rand::Rng;
use uwu::uwuify;

#[poise::command(slash_command)]
async fn mrbeast(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://tenor.com/view/mrbeast-ytpmv-rap-battle-squid-game-squid-game-vs-mrbeast-gif-25491394").await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn uwu(
    ctx: Context<'_>,
    #[description = "Text to uwuify"] text: String,
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
                .create_webhook(&ctx.http(), "Uwu webhook")
                .await?
        }
    };

    webhook
        .execute(&ctx.http(), false, |m| {
            m.content(uwuify(text))
                .avatar_url(avatar_url)
                .username(name)
        })
        .await?;

    reply.delete(ctx).await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn capy64(ctx: Context<'_>) -> Result<(), Error> {
    ctx.say("https://discord.gg/ZCXKGTM6Mm").await?;
    Ok(())
}

#[derive(Default)]
struct Entry {
    pub author: String,
    pub content: String,
    pub jump: String,
    pub place: u8,
    pub count: u8,
}

// https://stackoverflow.com/questions/38461429/how-can-i-truncate-a-string-to-have-at-most-n-characters#comment64327244_38461429
fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

#[poise::command(slash_command)]
async fn top10moyai(ctx: Context<'_>) -> Result<(), Error> {
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

#[poise::command(
    context_menu_command = "Embrace",
    guild_only,
    required_permissions = "MANAGE_ROLES"
)]
async fn embrace(ctx: Context<'_>, mem: serenity::User) -> Result<(), Error> {
    let gid = if let Some(gid) = ctx.guild_id() {
        gid
    } else {
        unreachable!();
    };

    if gid == serenity::GuildId(1023332212403351563) {
        let mut member = gid.member(&ctx.http(), mem.id).await?;
        member.add_role(&ctx.http(), 1023334181952049203).await?;
    } else {
        ctx.send(|m| {
            m.content("Sorry, but this command only works in a specific guild.")
                .ephemeral(true)
        })
        .await?;
    }

    ctx.send(|m| m.content("Done!").ephemeral(true)).await?;

    Ok(())
}

#[poise::command(slash_command)]
#[allow(unused_variables)]
async fn e621(
    ctx: Context<'_>,
    #[description = "List of tags separated by commas"] tags: String,
) -> Result<(), Error> {
    ctx.say("https://tenor.com/view/4k-caught-caught-in4k-caught-in8k-8k-gif-20014426")
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    db::init().await.unwrap();

    let token = {
        #[cfg(debug_assertions)]
        let mut f = File::open("./dtoken.txt").expect("dtoken.txt not found");
        #[cfg(not(debug_assertions))]
        let mut f = File::open("./token.txt").expect("token.txt not found");
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(_) => panic!("Failed to read token."),
        };
        s
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    let commands = vec![capy64(), mrbeast(), uwu(), embrace(), e621(), top10moyai()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, user_data| {
                Box::pin(async move {
                    match event {
                        poise::Event::Message { new_message } => {
                            message_handler(ctx, new_message).await?;
                        }
                        poise::Event::ReactionAdd { add_reaction } => {
                            reaction_handler(ctx, add_reaction, user_data).await?;
                        }
                        poise::Event::ReactionRemove { removed_reaction } => {
                            reaction_handler(ctx, removed_reaction, user_data).await?;
                            reaction_remove(ctx, removed_reaction).await?;
                        }
                        _ => (),
                    };
                    Ok(())
                })
            },
            ..Default::default()
        })
        .token(token)
        .intents(intents)
        .setup(|ctx, ready, frm| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &frm.options().commands).await?;
                let bot_pfp = ready.user.avatar_url();
                Ok(Data { bot_pfp })
            })
        });

    framework.run().await.unwrap();
}

async fn reaction_handler(
    ctx: &serenity::Context,
    react: &Reaction,
    user_data: &Data,
) -> Result<(), Error> {
    let is_moyai = match &react.emoji {
        #[allow(unused_variables)]
        serenity::ReactionType::Unicode(char) => char == "🗿",
        _ => false,
    };
    if react.channel_id == CURSED_BOARD {
        return Ok(());
    }

    if is_moyai {
        let msg = ctx
            .http()
            .get_message(react.channel_id.into(), react.message_id.into())
            .await?;
        let count = {
            let mut result = 0;
            for reaction in msg.reactions.iter() {
                if reaction.reaction_type.unicode_eq("🗿") {
                    result = reaction.count;
                    break;
                }
            }
            result
        }
        .try_into()
        .unwrap();
        let msg = db::get(react.message_id.into()).await?;
        db::set(
            msg.msg_id,
            msg.channel_id,
            msg.post_id.clone(),
            msg.link,
            count,
        )
        .await?;
        if count >= THRESHOLD {
            if !db::exists(react.message_id.into()).await? {
                let channel = ctx.http.get_channel(CURSED_BOARD).await?;
                if let Channel::Guild(ch) = channel {
                    let message = react.message(ctx.http()).await?;
                    let author_pfp = message.author.avatar_url().unwrap_or("https://cdn.discordapp.com/attachments/1078686956705284158/1102276838513971311/nix.png".to_string());
                    let author_nick = message
                        .author_nick(ctx.http())
                        .await
                        .unwrap_or(message.author.name.clone());
                    let image = {
                        let img = message.attachments.first();
                        if img.is_some() {
                            img.unwrap().url.to_owned()
                        } else {
                            "".to_string()
                        }
                    };
                    let color = serenity::Colour(0xAA00BB);
                    let jump_url = message.link();
                    let msg = ch
                        .send_message(ctx.http(), |m| {
                            m.embed(|e| {
                                e.author(|a| a.name(author_nick).icon_url(author_pfp));
                                e.description(message.content);
                                e.field("Source", format!("[Jump]({})", jump_url), true);
                                e.footer(|f| {
                                    if let Some(pfp) = &user_data.bot_pfp {
                                        f.icon_url(pfp);
                                    };
                                    f.text(message.timestamp)
                                });
                                e.image(image);
                                e.color(color)
                            });
                            m.content(format!("<#{}>", message.channel_id.as_u64()))
                        })
                        .await?;
                    db::set(
                        react.message_id.to_string(),
                        react.channel_id.to_string(),
                        msg.id.to_string(),
                        msg.link(),
                        count,
                    )
                    .await?;
                }
            }
        }
    }

    Ok(())
}

async fn reaction_remove(ctx: &serenity::Context, react: &Reaction) -> Result<(), Error> {
    let is_moyai = match &react.emoji {
        #[allow(unused_variables)]
        serenity::ReactionType::Unicode(char) => char == "🗿",
        #[allow(unused_variables)]
        serenity::ReactionType::Custom { animated, id, name } => false,
        _ => false,
    };

    if is_moyai {
        let list = db::clean().await?;
        for msg in list.iter() {
            if *msg != 0 {
                let message = ctx.http.get_message(CURSED_BOARD, *msg).await?;
                message.delete(ctx.http()).await?;
            }
        }
    }

    Ok(())
}

async fn message_handler(ctx: &serenity::Context, msg: &Message) -> Result<(), Error> {
    if msg.author.bot {
        return Ok(());
    }
    let content = msg.content.to_lowercase();
    let files = get_files(&content);
    let mut reply_content = reply_content(&content);

    let piss = rand::thread_rng().gen_ratio(1, 100);
    if piss {
        reply_content += "*pees in your ass*";
    }

    if files.is_empty() && reply_content.is_empty() {
        return Ok(());
    }

    msg.channel_id
        .send_message(&ctx.http(), |m| {
            m.files(files).content(reply_content);
            if piss {
                m.reference_message(msg);
            };
            m
        })
        .await?;
    Ok(())
}

fn reply_content(content: &str) -> String {
    let mut out = String::from("");
    if content.contains("moyai") || content.contains('🗿') {
        out += "🗿";
    };
    if content.contains("balls") {
        out += "<:whatwedotoyourballs:1023352899075571752> ";
    };
    if content.contains("connor") {
        out += ":skull: ";
    };
    if content.contains("stupid") {
        let stupid = rand::thread_rng().gen_ratio(1, 10);
        if stupid {
            out += "https://www.youtube.com/watch?v=nnkyInAj6Z8 ";
        }
    };
    out
}

fn get_files(content: &str) -> Vec<AttachmentType> {
    let mut out = vec![];
    if content.contains("rust") && content.contains("capy64") {
        out.push(AttachmentType::Path(Path::new("./assets/rust.mp4")));
    };
    if content.contains("waaa") {
        out.push(AttachmentType::Path(Path::new("./assets/waaa.mp4")));
    };
    out
}
