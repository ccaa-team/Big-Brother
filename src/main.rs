use poise::serenity_prelude as serenity;

use uwuifier::uwuify_str_sse;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {}

struct Handler {}

static UWUCHANNEL: serenity::ChannelId = serenity::ChannelId(1024420622647967824);
static NORMALCHANNEL: u64 = 1023332213078626448;

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Message { new_message } => {
            let msg = new_message;

            let content = &msg.content_safe(&ctx.cache);
            let channel_id = &msg.channel_id;

            let author_name = msg.author.tag();

            let avatar_url = match msg.author.avatar_url() {
                Some(x) => x,
                None => "".to_string(),
            };

            if content != "" && channel_id.as_u64().to_owned() == NORMALCHANNEL {
                let uwud = uwuify_str_sse(content);

                UWUCHANNEL
                    .send_message(&ctx.http, |msg| {
                        msg.embed(|e| {
                            e.author(|a| a.name(author_name).icon_url(avatar_url));
                            e.description(uwud);
                            e.color(0xffffff)
                        })
                    })
                    .await
                    .unwrap();
            }
        }
        _ => {}
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }));

    framework.run().await.unwrap();
}
