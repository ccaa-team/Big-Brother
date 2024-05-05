use markov::Chain;
use poise::{command, say_reply};

use crate::{Context, Error};

static mut CHAIN: Option<&mut Chain<String>> = None;

fn generate() -> String {
    unsafe {
        if CHAIN.is_none() {
            let mut chain = Chain::<String>::of_order(1);
            chain.feed_file("./data").unwrap();
            let chain = Box::new(chain);
            let leaked = Box::leak(chain);
            CHAIN = Some(leaked);
        };

        CHAIN.as_ref().unwrap().generate_str()
    }
}

#[command(slash_command, prefix_command)]
pub async fn markov(ctx: Context<'_>) -> Result<(), Error> {
    ctx.channel_id().broadcast_typing(ctx.http()).await?;

    say_reply(ctx, generate()).await.unwrap();

    Ok(())
}
