mod uwu;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use rand::Rng;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::application::interaction::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::prelude::command::{Command, CommandOptionType};
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::{AttachmentType, Message, Reaction};
use serenity::model::webhook::Webhook;
use serenity::prelude::*;
use serenity::{async_trait, Client};
use uwu::uwuify;

struct Handler;

async fn mrbeast(
    ctx: &Context,
    inter: &ApplicationCommandInteraction,
) -> Result<(), SerenityError> {
    inter.create_interaction_response(&ctx.http, |res| {
        res.kind(serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|msg| msg.content("https://tenor.com/view/mrbeast-ytpmv-rap-battle-squid-game-squid-game-vs-mrbeast-gif-25491394"))
    }).await?;

    Ok(())
}
async fn uwu(ctx: &Context, inter: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
    inter
        .create_interaction_response(&ctx.http, |res| {
            res.interaction_response_data(|d| d.content("ok").ephemeral(true))
        })
        .await?;

    // Crashing should be impossible because the argument is required.
    let arg = inter
        .data
        .options
        .get(0)
        .unwrap()
        .resolved
        .as_ref()
        .unwrap();

    let content = if let CommandDataOptionValue::String(input) = arg {
        uwuify(input.to_string())
    } else {
        panic!();
    };

    let (name, avatar_url) = if let Some(member) = &inter.member {
        (member.display_name().to_string(), member.face())
    } else {
        let user = &inter.user;
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

    let channel_id = inter.channel_id;
    let webhook = match get_webhook(channel_id.webhooks(&ctx.http).await?) {
        Some(hook) => hook,
        None => channel_id.create_webhook(&ctx.http, "Uwu webhook").await?,
    };

    webhook
        .execute(&ctx.http, false, |m| {
            m.content(content).avatar_url(avatar_url).username(name)
        })
        .await?;

    inter
        .delete_original_interaction_response(&ctx.http)
        .await?;

    Ok(())
}

async fn pedo(ctx: &Context, inter: &ApplicationCommandInteraction) -> Result<(), SerenityError> {
    let file = Path::new("assets/pedo.png");
    inter
        .create_interaction_response(&ctx.http, |res| {
            res.interaction_response_data(|d| d.add_file(file))
        })
        .await?;

    Ok(())
}

async fn cmd_todo(
    ctx: &Context,
    inter: &ApplicationCommandInteraction,
) -> Result<(), SerenityError> {
    inter
        .create_interaction_response(&ctx.http, |res| {
            res.kind(serenity::model::prelude::interaction::InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|msg| {
                    msg.content("This is probably still being worked on")
                        .ephemeral(true)
                })
        })
        .await?;

    Ok(())
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let get_files = |text: &str| {
            let mut out = vec![];
            if text.contains("rust") || text.contains("iron oxide") || text.contains("fe203") {
                out.push(AttachmentType::Path(Path::new("assets/rust.mp4")));
            }
            if text.contains("waaa") {
                out.push(AttachmentType::Path(Path::new("assets/waaa.mp4")));
            }
            out
        };

        let get_content = |text: &str| {
            let mut out = String::from("");

            if text.contains("moyai") || text.contains("🗿") {
                out += "🗿 ";
            }
            if text.contains("balls") {
                out += "<:whatwedotoyourballs:1023352899075571752> ";
            }

            out
        };

        let content = msg.content.to_lowercase();

        let files = get_files(&content);
        let mut content = get_content(&content);

        let num = rand::thread_rng().gen_range(0..100);
        let piss = num < 2;

        if files.is_empty() && content.is_empty() && !piss {
            return;
        }

        if piss {
            content = String::from("*pees in your ass* ") + &content;
        }

        let _ = msg
            .channel_id
            .send_message(&ctx.http, |m| {
                m.files(files).content(content);
                if piss {
                    m.reference_message(&msg);
                }
                m
            })
            .await;
    }
    async fn ready(&self, ctx: Context, _ready: Ready) {
        let _ = Command::set_global_application_commands(&ctx.http, |commands| {
            commands.create_application_command(|cmd| {
                cmd.name("mrbeast").description("OMG IT'S MRBEAST'")
            });
            commands.create_application_command(|cmd| {
                cmd.name("uwu").description("uwu").create_option(|option| {
                    option
                        .name("text")
                        .description("The text to uwuify")
                        .kind(CommandOptionType::String)
                        .required(true)
                })
            });
            commands.create_application_command(|cmd| cmd.name("pedo").description("pedo"))
        })
        .await;
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(cmd) = interaction {
            let _ = match cmd.data.name.as_str() {
                "mrbeast" => mrbeast(&ctx, &cmd).await,
                "uwu" => uwu(&ctx, &cmd).await,
                "pedo" => pedo(&ctx, &cmd).await,
                _ => cmd_todo(&ctx, &cmd).await,
            };
        };
    }
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

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Error creating client.");

    client.start().await.unwrap();
}
