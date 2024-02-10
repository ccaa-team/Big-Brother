use nekosbest::Category;
use poise::{
    command,
    serenity_prelude::{CreateEmbed, CreateEmbedFooter},
    CreateReply,
};

use crate::{Context, Error};

lazy_static::lazy_static! {
static ref CATEGORIES: Vec<&'static str> = Category::ALL_VARIANTS
    .iter()
    .map(|v| v.to_url_name())
    .collect();
}

async fn autocomplete_category<'a>(_ctx: Context<'_>, partial: &'a str) -> Vec<String> {
    CATEGORIES
        .iter()
        .filter(|v| partial.is_empty() || v.starts_with(partial))
        .map(|v| v.to_string())
        .collect()
}

#[command(slash_command, prefix_command)]
/// Get a random image/gif from a category you choose by using nekos.best
pub async fn neko(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_category"] category: String,
    amount: Option<u8>,
) -> Result<(), Error> {
    let amount = amount.unwrap_or(1).clamp(1, 20);
    let category = if CATEGORIES.contains(&category.as_str()) {
        Category::from_url_name(&category).unwrap()
    } else {
        let m = CreateReply::default().content(format!(
            "{} is not a valid category, the available categories are:\n{}",
            category,
            CATEGORIES.join(", ")
        ));

        ctx.send(m).await?;
        return Ok(());
    };

    let imgs = nekosbest::get_amount(category, amount).await?;

    for c in imgs.chunks(10) {
        let mut m = CreateReply::default();
        for img in c.iter() {
            let url = &img.url;
            let source = match &img.details {
                nekosbest::details::Details::Image(d) => d.source_url.as_str(),
                nekosbest::details::Details::Gif(d) => &d.anime_name,
                _ => todo!(),
            };
            let f = CreateEmbedFooter::new(source);
            let e = CreateEmbed::new().image(url).footer(f);
            m = m.embed(e);
        }
        ctx.send(m).await?;
    }

    Ok(())
}
