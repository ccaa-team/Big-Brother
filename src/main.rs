mod uwu;
use std::fs::{self, File};
use std::io::Read;
use std::path::Path;

use poise::serenity_prelude::{
    self as serenity, AttachmentType, CacheHttp, GatewayIntents,
};

struct Data {}
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

use rand::Rng;
use serenity::model::webhook::Webhook;
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

    let (name, avatar_url) = if let Some(member) = ctx.author_member().await {
        (member.display_name().to_string(), member.face())
    } else {
        let user = ctx.author();
        (user.name.to_owned(), user.face())
    };

    let get_webhook = |webhooks: Vec<Webhook>| -> Option<Webhook> {
        for webhook in webhooks {
            if let Some(_) = &webhook.token {
                return Some(webhook);
            }
        }

        None
    };

    let channel_id = ctx.channel_id();
    let webhook = match get_webhook(channel_id.webhooks(&ctx.http()).await?) {
        Some(hook) => hook,
        None => {
            channel_id
                .create_webhook(&ctx.http(), "Uwu webhook")
                .await?
        }
    };

    let content = uwuify(text);
    webhook
        .execute(&ctx.http(), false, |m| {
            m.content(content).avatar_url(avatar_url).username(name)
        })
        .await?;

    reply.delete(ctx).await?;

    Ok(())
}

async fn autocomplete_file<'a>(
    _ctx: Context<'_>,
    _partial: &'a str,
) -> impl Iterator<Item = poise::AutocompleteChoice<String>> {
    let paths = fs::read_dir("./assets/pedo").expect("assets folder not found.");
    let mut autoc_paths = vec![];
    for path in paths {
        let path2 = path.unwrap().path();
        let name = path2.display().to_string();
        let display = name.replace("./assets/pedo/", "");

        let choice = poise::AutocompleteChoice {
            name: display,
            value: name,
        };
        autoc_paths.push(choice);
    }

    autoc_paths.into_iter()
}

#[poise::command(slash_command)]
async fn pedo(
    ctx: Context<'_>,
    #[description = "File to send"]
    #[autocomplete = "autocomplete_file"]
    name: String,
) -> Result<(), Error> {
    let file = Path::new(&name);

    ctx.send(|r| r.attachment(serenity::AttachmentType::Path(file)))
        .await?;

    Ok(())
}

#[poise::command(slash_command)]
async fn capy64(
    ctx: Context<'_>
) -> Result<(), Error> {
    ctx.say("https://discord.gg/ZCXKGTM6Mm").await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = {
        let mut f = File::open("token.txt").expect("token.txt not found");
        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => (),
            Err(_) => panic!("Failed to read token."),
        };
        s
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![mrbeast(), uwu(), pedo(), capy64()],
            event_handler: |ctx, event, _framework, _user_data| {
                Box::pin(async move {
                    match event {
                        poise::Event::Message { new_message } => {
                            if new_message.author.bot {
                                return Ok(());
                            };
                            let content = new_message.content.to_lowercase();
                            let files = get_files(&content);
                            let mut reply_content = reply_content(&content);

                            let piss = rand::thread_rng().gen_ratio(1, 50);

                            if piss {
                                reply_content += "*pees in your ass*";
                            };

                            if files.is_empty() && reply_content.is_empty() {
                                return Ok(());
                            };

                            new_message.channel_id.send_message(&ctx.http, |m| {
                                m.files(files).content(reply_content);
                                if piss {
                                    m.reference_message(new_message);
                                };
                                m
                            }).await?;
                        },
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
        out.push(AttachmentType::Path(Path::new("assets/rust.mp4")));
    };
    if content.contains("waaa") {
        out.push(AttachmentType::Path(Path::new("assets/rust.mp4")));
    };
    out
}
