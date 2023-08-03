#![feature(async_closure)]

mod db;
mod globals;
mod uwu;
mod commands;

use std::{fs::File, io::Read, path::Path};

use poise::serenity_prelude::{
    self as serenity, AttachmentType, CacheHttp, Channel, GatewayIntents, Message,
    Reaction,
};

use globals::*;
use rand::Rng;
use uwu::uwuify;

pub struct Data {
    pub bot_pfp: Option<String>,
}
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Default)]
struct Entry {
    pub author: String,
    pub content: String,
    pub jump: String,
    pub place: u8,
    pub count: u8,
}

#[cfg(debug_assertions)]
static TOKEN_FILE: &str = "dtoken.txt";
#[cfg(not(debug_assertions))]
static TOKEN_FILE: &str = "token.txt";

#[tokio::main]
async fn main() {
    db::init().await.unwrap();

    let token = {
        let mut f = File::open(TOKEN_FILE).unwrap_or_else(|_| panic!("{TOKEN_FILE} not found."));
        let mut s = String::new();
        f.read_to_string(&mut s).expect("Failed to read token.");
        s
    };

    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;

    use crate::commands::*;
    let commands = vec![top10moyai(), uwu(), embrace(), crazy()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("".into()),
                case_insensitive_commands: true,
                ..Default::default()
            },
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
    if !react.emoji.unicode_eq("🗿") {
        return Ok(());
    };

    if react.channel_id == CURSED_BOARD {
        return Ok(());
    }

    let msg = ctx
        .http()
        .get_message(react.channel_id.into(), react.message_id.into())
        .await?;

    let mut iter = msg.reactions.iter();
    let count = loop {
        if let Some(reaction) = iter.next() {
            if reaction.reaction_type.unicode_eq("🗿") {
                break reaction.count;
            }
        }
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
    if count >= THRESHOLD && !db::exists(react.message_id.into()).await? {
        let channel = ctx.http.get_channel(CURSED_BOARD).await?;
        if let Channel::Guild(ch) = channel {
            let message = react.message(ctx.http()).await?;
            let author_pfp = message
                .author
                .avatar_url()
                .unwrap_or(BACKUP_PFP.to_string());
            let author_nick = message
                .author_nick(ctx.http())
                .await
                .unwrap_or(message.author.name.clone());
            let jump_url = message.link();
            let msg = ch
                .send_message(ctx.http(), |m| {
                    m.embed(|e| {
                        e.author(|a| a.name(author_nick).icon_url(author_pfp))
                            .description(message.content)
                            .field("Source", format!("[Jump]({})", jump_url), true)
                            .footer(|f| {
                                if let Some(pfp) = &user_data.bot_pfp {
                                    f.icon_url(pfp);
                                };
                                f.text(message.timestamp)
                            });
                        if let Some(attachment) = message.attachments.first() {
                            e.image(&attachment.url);
                        }
                        e.color(serenity::Colour(0xAA00BB))
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

    Ok(())
}

async fn reaction_remove(ctx: &serenity::Context, react: &Reaction) -> Result<(), Error> {
    if !react.emoji.unicode_eq("🗿") {
        return Ok(());
    };

    let list = db::clean().await?;
    for msg in list.iter() {
        if *msg != 0 {
            let message = ctx.http.get_message(CURSED_BOARD, *msg).await?;
            message.delete(ctx.http()).await?;
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
        reply_content += "*pees in your ass*[Citation needed]";
    }

    if files.is_empty() && reply_content.is_empty() {
        return Ok(());
    }
    reply_content = uwuify(reply_content);

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
    if content.matches(":3").count() > 0 {
        out += "bottom detected :3\n";
    }
    let matches = { content.matches("moyai").count() + content.matches('🗿').count() };
    out += &"🗿".repeat(matches);
    if content.contains("balls") {
        out += "🫴";
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
