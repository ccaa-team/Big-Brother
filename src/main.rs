use poise::serenity_prelude::{self as serenity, Webhook};

use uwuifier::uwuify_str_sse;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

struct Data {
    webhook: Webhook,
}

static NORMALCHANNEL: u64 = 1023332213078626448;

async fn event_listener(
    ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    user_data: &Data,
) -> Result<(), Error> {
    match event {
        poise::Event::Message { new_message } => {
            let msg = new_message;

            let content = &msg.content_safe(&ctx.cache);
            let channel_id = &msg.channel_id;

            if content != "" && channel_id.as_u64().to_owned() == NORMALCHANNEL {
                let normal_name = &msg.author.name;

                let author_name = uwuify_str_sse(&normal_name);

                let avatar_url = match msg.author.avatar_url() {
                    Some(x) => x,
                    None => "".to_string(),
                };

                let uwud = uwuify_str_sse(content);

                match user_data
                    .webhook
                    .execute(&ctx.http, false, |w| {
                        w.avatar_url(avatar_url);
                        w.username(author_name);
                        w.content(uwud)
                    })
                    .await
                {
                    Ok(_) => (),
                    Err(why) => println!("Error sending message: {:?}", why),
                };
            }
        }
        _ => {}
    }

    Ok(())
}

#[poise::command(prefix_command)]
async fn help(ctx: Context<'_>) -> Result<(), Error> {
    match ctx
        .send(|msg| {
            msg.embed(|e| {
                e.title("Big brother bot");
                e.description("I uwuify text and will get rid of 🥹 when alex lets me")
            })
        })
        .await
    {
        Ok(_) => (),
        Err(why) => println!("Erro replying to help command: {:?}", why),
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![help()],
            listener: |ctx, event, framework, user_data| {
                Box::pin(event_listener(ctx, event, framework, user_data))
            },
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
        .user_data_setup(move |ctx, _, _| {
            Box::pin(async move {
                Ok(Data {
                    webhook: Webhook::from_id(&ctx.http, 1024655591270002718).await?,
                })
            })
        });

    framework.run().await.unwrap();
}
