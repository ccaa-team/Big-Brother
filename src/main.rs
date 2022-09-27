use poise::{
    async_trait,
    serenity_prelude::{self as serenity, EventHandler, Message},
};

use uwuifier::uwuify_str_sse;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {}

struct Handler {}

static UWUCHANNEL: serenity::ChannelId = serenity::ChannelId(1024420622647967824);
static NORMALCHANNEL: u64 = 1023332213078626448;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: serenity::Context, msg: Message) {
        let content = &msg.content_safe(&ctx.cache);
        let channel_id = &msg.channel_id;

        let author_name = match msg.author_nick(&ctx.http).await {
            Some(x) => x,
            None => msg.author.name,
        };

        if content == "" || channel_id.as_u64().to_owned() != NORMALCHANNEL {
            return;
        }

        let uwud = uwuify_str_sse(content);

        UWUCHANNEL
            .send_message(&ctx.http, |msg| {
                msg.embed(|e| {
                    e.author(|a| a.name(author_name));
                    e.description(uwud)
                })
            })
            .await
            .unwrap();
    }
}

#[poise::command(prefix_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    ctx.send(|m| m.embed(|e| e.title("shit yorself"))).await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help()],
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("'".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .token(std::env::var("token").expect("No token in env"))
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .user_data_setup(move |_, _, _| Box::pin(async move { Ok(Data {}) }))
        .client_settings(|f| f.event_handler(Handler {}));

    framework.run().await.unwrap();
}
