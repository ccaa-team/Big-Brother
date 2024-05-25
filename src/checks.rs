use rand::{thread_rng, Rng};

use crate::{mommy, Context, Error};

const CHANCE: f64 = 0.1;

async fn mommy(ctx: Context<'_>) -> Result<bool, Error> {
    let mut data = ctx.data().has_to_beg.lock().await;

    Ok(
        if data.get(&ctx.author().id).is_some() || thread_rng().gen_bool(CHANCE) {
            ctx.say(mommy::beg()).await?;
            data.insert(ctx.author().id);
            false
        } else {
            true
        },
    )
}

pub async fn check(ctx: Context<'_>) -> Result<bool, Error> {
    mommy(ctx).await
}
