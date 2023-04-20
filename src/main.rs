mod uwu;

use poise::serenity_prelude::{
    self as serenity, AttachmentType, CacheHttp, GatewayIntents, Message,
};

struct Data {}
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
    //let token = {
    //    let mut f = File::open("token.txt").expect("token.txt not found");
    //    let mut s = String::new();
    //    match f.read_to_string(&mut s) {
    //        Ok(_) => (),
    //        Err(_) => panic!("Failed to read token."),
    //    };
    //    s
    //};

    let token = include_str!("../token.txt");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let commands = vec![capy64(), mrbeast(), uwu(), embrace(), e621()];

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands,
            event_handler: |ctx, event, _framework, _user_data| {
                Box::pin(async move {
                    match event {
                        poise::Event::Message { new_message } => {
                            message_handler(ctx, new_message).await?;
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
        .setup(|ctx, _ready, frm| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &frm.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
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
        reply_content += "# *pees in your ass*";
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
    if content.contains("moyai") || content.contains("🗿") {
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
        out.push(AttachmentType::Bytes {
            data: std::borrow::Cow::Borrowed(include_bytes!("../assets/rust.mp4")),
            filename: "rust.mp4".to_string(),
        });
    };
    if content.contains("waaa") {
        out.push(AttachmentType::Bytes {
            data: std::borrow::Cow::Borrowed(include_bytes!("../assets/waaa.mp4")),
            filename: "waaa.mp4".to_string(),
        });
    };
    out
}
